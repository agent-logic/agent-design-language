//! Deterministic resident incident ownership. No model or network call occurs
//! while observing health. Effects are reserved durably before dispatch.
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs, io, path::PathBuf};

pub const DETECTION_INTERVAL_MILLIS: u64 = 5_000;
pub const RESPONSE_TIMEOUT_MILLIS: u64 = 30_000;
const MAX_RESPONSE_ATTEMPTS: u32 = 3;

#[derive(Clone, Debug)]
pub struct ResidentObservation {
    pub id: String,
    pub binding: String,
    pub unhealthy: bool,
    pub reason: &'static str,
    pub inference_verified: bool,
    pub inference_observed_at_unix_millis: u64,
    pub present: bool,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncidentState {
    Open,
    Recovered,
    Retired,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ResidentIncident {
    pub incident_id: String,
    pub sequence: u64,
    pub revision: u64,
    pub resident_id: String,
    pub binding: String,
    pub reason: String,
    pub state: IncidentState,
    pub opened_at_unix_millis: u64,
    pub updated_at_unix_millis: u64,
    pub response_attempts: u32,
    pub response_deadline_unix_millis: Option<u64>,
    pub next_response_at_unix_millis: u64,
    pub response_status: String,
    pub escalated: bool,
    pub alert_attempts: u32,
    pub next_alert_at_unix_millis: u64,
    pub alert_delivered: bool,
    pub alert_status: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Journal {
    namespace: String,
    sequence: u64,
    incidents: BTreeMap<String, ResidentIncident>,
}

impl Default for Journal {
    fn default() -> Self {
        Self {
            namespace: uuid::Uuid::new_v4().to_string(),
            sequence: 0,
            incidents: BTreeMap::new(),
        }
    }
}

#[derive(Default)]
pub struct ResidentHealthSupervisor {
    journal: Journal,
    path: Option<PathBuf>,
}
impl ResidentHealthSupervisor {
    pub fn open(path: PathBuf) -> io::Result<Self> {
        let journal = match fs::read(&path) {
            Ok(bytes) => serde_json::from_slice(&bytes).map_err(io::Error::other)?,
            Err(e) if e.kind() == io::ErrorKind::NotFound => Journal::default(),
            Err(e) => return Err(e),
        };
        Ok(Self {
            journal,
            path: Some(path),
        })
    }
    fn commit(&mut self, next: Journal) -> io::Result<()> {
        if let Some(path) = &self.path {
            let parent = path
                .parent()
                .ok_or_else(|| io::Error::other("incident path parent"))?;
            fs::create_dir_all(parent)?;
            let tmp = path.with_extension("pending");
            let mut options = fs::OpenOptions::new();
            options.write(true).create(true).truncate(true);
            #[cfg(unix)]
            {
                use std::os::unix::fs::OpenOptionsExt;
                options.mode(0o600);
            }
            let file = options.open(&tmp)?;
            serde_json::to_writer(&file, &next).map_err(io::Error::other)?;
            file.sync_all()?;
            fs::rename(tmp, path)?;
            #[cfg(unix)]
            fs::File::open(parent)?.sync_all()?;
        }
        self.journal = next;
        Ok(())
    }
    pub fn snapshot(&self) -> Vec<ResidentIncident> {
        let mut incidents = self.journal.incidents.values().cloned().collect::<Vec<_>>();
        incidents.sort_by_key(|i| i.sequence);
        incidents
    }
    fn create(next: &mut Journal, o: &ResidentObservation, reason: &str, now: u64) {
        next.sequence += 1;
        let incident_id = format!("resident-health-{}-{}", next.namespace, next.sequence);
        next.incidents.insert(
            incident_id.clone(),
            ResidentIncident {
                incident_id,
                sequence: next.sequence,
                revision: 1,
                resident_id: o.id.clone(),
                binding: o.binding.clone(),
                reason: reason.to_owned(),
                state: IncidentState::Open,
                opened_at_unix_millis: now,
                updated_at_unix_millis: now,
                response_attempts: 0,
                response_deadline_unix_millis: None,
                next_response_at_unix_millis: now,
                response_status: "pending".into(),
                escalated: false,
                alert_attempts: 0,
                next_alert_at_unix_millis: now,
                alert_delivered: false,
                alert_status: "pending".into(),
            },
        );
    }
    pub fn observe(
        &mut self,
        observations: &[ResidentObservation],
        shepherd_ready: bool,
        now: u64,
    ) -> io::Result<()> {
        let mut next = self.journal.clone();
        for incident in next
            .incidents
            .values_mut()
            .filter(|i| i.state == IncidentState::Open)
        {
            match observations
                .iter()
                .find(|o| o.id == incident.resident_id && o.present)
            {
                None => {
                    incident.state = IncidentState::Retired;
                    incident.response_status = "resident_removed".into();
                }
                Some(o) if o.binding != incident.binding => {
                    incident.binding = o.binding.clone();
                    incident.response_deadline_unix_millis = None;
                    incident.response_status = "binding_changed".into();
                    // Preserve ownership and bounded budget across replacements.
                }
                Some(o)
                    if !o.unhealthy
                        && o.inference_verified
                        && o.inference_observed_at_unix_millis > incident.opened_at_unix_millis =>
                {
                    incident.state = IncidentState::Recovered;
                    incident.response_status = "verified_recovery".into();
                }
                _ => {}
            }
            if let Some(o) = observations
                .iter()
                .find(|o| o.id == incident.resident_id && o.present)
            {
                fn severity(reason: &str) -> u8 {
                    match reason {
                        "observed_health_failure" => 3,
                        "resident_observation_stale" => 2,
                        "inference_evidence_stale" => 1,
                        _ => 0,
                    }
                }
                if incident.state == IncidentState::Open
                    && severity(o.reason) > severity(&incident.reason)
                {
                    incident.reason = o.reason.into();
                    incident.revision = incident.revision.saturating_add(1);
                    incident.alert_delivered = false;
                    incident.alert_status = "pending_health_worsened".into();
                    incident.next_alert_at_unix_millis = now;
                    incident.response_deadline_unix_millis = None;
                    incident.escalated = true;
                    incident.updated_at_unix_millis = now;
                }
            }
            if incident.state != IncidentState::Open {
                incident.response_deadline_unix_millis = None;
                incident.updated_at_unix_millis = now;
                continue;
            }
            if incident
                .response_deadline_unix_millis
                .is_some_and(|d| now >= d)
            {
                incident.response_deadline_unix_millis = None;
                incident.response_status = "response_timeout".into();
                incident.escalated = true;
            }
            if now.saturating_sub(incident.opened_at_unix_millis) >= RESPONSE_TIMEOUT_MILLIS {
                incident.escalated = true;
            }
            if !shepherd_ready {
                incident.escalated = true;
                incident.response_status = "shepherd_unavailable".into();
            }
        }
        for o in observations.iter().filter(|o| o.present && o.unhealthy) {
            if !next
                .incidents
                .values()
                .any(|i| i.resident_id == o.id && i.state == IncidentState::Open)
            {
                Self::create(&mut next, o, o.reason, now);
                if !shepherd_ready {
                    for i in next
                        .incidents
                        .values_mut()
                        .filter(|i| i.resident_id == o.id && i.state == IncidentState::Open)
                    {
                        i.escalated = true;
                    }
                }
            }
        }
        if serde_json::to_vec(&next)? != serde_json::to_vec(&self.journal)? {
            self.commit(next)?;
        }
        Ok(())
    }
    pub fn request_help(&mut self, o: &ResidentObservation, now: u64) -> io::Result<()> {
        let mut next = self.journal.clone();
        if !next
            .incidents
            .values()
            .any(|i| i.resident_id == o.id && i.state == IncidentState::Open)
        {
            Self::create(&mut next, o, "explicit_help_request", now);
        }
        for i in next
            .incidents
            .values_mut()
            .filter(|i| i.resident_id == o.id && i.state == IncidentState::Open)
        {
            if matches!(
                i.reason.as_str(),
                "inference_unverified" | "inference_evidence_stale"
            ) {
                i.reason = "explicit_help_request".into();
                i.revision = i.revision.saturating_add(1);
                i.alert_delivered = false;
                i.next_alert_at_unix_millis = now;
            }
            i.escalated = true;
        }
        self.commit(next)
    }
    pub fn reserve_response(&mut self, now: u64) -> io::Result<Option<ResidentIncident>> {
        let mut next = self.journal.clone();
        let Some(i) = next.incidents.values_mut().find(|i| {
            i.state == IncidentState::Open
                && !matches!(
                    i.reason.as_str(),
                    "inference_unverified" | "inference_evidence_stale"
                )
                && i.response_deadline_unix_millis.is_none()
                && i.response_attempts < MAX_RESPONSE_ATTEMPTS
                && now >= i.next_response_at_unix_millis
        }) else {
            return Ok(None);
        };
        i.response_attempts += 1;
        i.response_deadline_unix_millis = Some(now.saturating_add(RESPONSE_TIMEOUT_MILLIS));
        i.next_response_at_unix_millis = now
            .saturating_add(RESPONSE_TIMEOUT_MILLIS)
            .saturating_add(10_000 * (1 << i.response_attempts));
        i.response_status = "responding".into();
        let result = i.clone();
        self.commit(next)?;
        Ok(Some(result))
    }
    pub fn finish_response(
        &mut self,
        reserved: &ResidentIncident,
        success: bool,
        now: u64,
    ) -> io::Result<()> {
        let mut next = self.journal.clone();
        if let Some(i) = next.incidents.get_mut(&reserved.incident_id).filter(|i| {
            i.state == IncidentState::Open
                && i.binding == reserved.binding
                && i.revision == reserved.revision
                && i.response_attempts == reserved.response_attempts
                && i.response_deadline_unix_millis == reserved.response_deadline_unix_millis
        }) {
            i.response_deadline_unix_millis = None;
            i.response_status = if success {
                "acknowledged_awaiting_verified_recovery"
            } else {
                "response_failed"
            }
            .into();
            i.updated_at_unix_millis = now;
            i.escalated = true; // Acknowledgment is not evidence of repair.
        }
        self.commit(next)
    }
    pub fn reserve_alert(&mut self, now: u64) -> io::Result<Option<ResidentIncident>> {
        let mut next = self.journal.clone();
        let Some(i) = next
            .incidents
            .values_mut()
            .find(|i| i.escalated && !i.alert_delivered && now >= i.next_alert_at_unix_millis)
        else {
            return Ok(None);
        };
        i.alert_attempts = i.alert_attempts.saturating_add(1);
        i.next_alert_at_unix_millis = now
            .saturating_add((5_000u64.saturating_mul(1 << i.alert_attempts.min(6))).min(300_000));
        i.alert_status = "sending".into();
        let result = i.clone();
        self.commit(next)?;
        Ok(Some(result))
    }
    pub fn finish_alert(&mut self, reserved: &ResidentIncident, delivered: bool) -> io::Result<()> {
        let mut next = self.journal.clone();
        if let Some(i) = next.incidents.get_mut(&reserved.incident_id).filter(|i| {
            i.alert_attempts == reserved.alert_attempts && i.revision == reserved.revision
        }) {
            i.alert_delivered = delivered;
            i.alert_status = if delivered {
                "sns_accepted"
            } else {
                "delivery_failed_retry_pending"
            }
            .into();
        }
        self.commit(next)
    }
}

/// Explicit opt-in transport. The selected profile must resolve to the exact
/// restricted assumed role; an administrator identity is never accepted.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ResidentAlertConfig {
    pub profile: String,
    pub region: String,
    pub topic_arn: String,
    pub role_arn: String,
}
impl ResidentAlertConfig {
    pub fn validate(&self) -> Result<(), &'static str> {
        let topic: Vec<_> = self.topic_arn.split(':').collect();
        let role: Vec<_> = self.role_arn.split(':').collect();
        if self.profile.is_empty()
            || self.profile.len() > 128
            || !self
                .profile
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
            || topic.len() != 6
            || role.len() != 6
            || topic[0..3] != ["arn", "aws", "sns"]
            || role[0..3] != ["arn", "aws", "iam"]
            || topic[3] != self.region
            || !role[3].is_empty()
            || topic[4].len() != 12
            || !topic[4].bytes().all(|c| c.is_ascii_digit())
            || topic[4] != role[4]
            || topic[5].is_empty()
            || !topic[5]
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
            || !role[5].starts_with("role/")
            || role[5][5..].is_empty()
            || !role[5][5..]
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || "-_".contains(c))
        {
            return Err("invalid_resident_alert_config");
        }
        Ok(())
    }
    fn accepts_identity(&self, identity: &serde_json::Value) -> bool {
        if self.validate().is_err() {
            return false;
        }
        let role: Vec<_> = self.role_arn.split(':').collect();
        let prefix = format!("arn:aws:sts::{}:assumed-role/{}/", role[4], &role[5][5..]);
        identity["Account"].as_str() == Some(role[4])
            && identity["Arn"]
                .as_str()
                .is_some_and(|arn| arn.starts_with(&prefix) && arn.len() > prefix.len())
    }
    async fn command(
        &self,
        program: &std::path::Path,
        args: &[&str],
    ) -> Result<serde_json::Value, &'static str> {
        let mut command = tokio::process::Command::new(program);
        command
            .args([
                "--profile",
                &self.profile,
                "--region",
                &self.region,
                "--output",
                "json",
                "--no-cli-pager",
            ])
            .args(args)
            .env("AWS_PAGER", "")
            .env("AWS_IGNORE_CONFIGURED_ENDPOINT_URLS", "true")
            .env_remove("AWS_ACCESS_KEY_ID")
            .env_remove("AWS_SECRET_ACCESS_KEY")
            .env_remove("AWS_SESSION_TOKEN")
            .env_remove("AWS_ENDPOINT_URL")
            .env_remove("AWS_ENDPOINT_URL_SNS")
            .env_remove("AWS_ENDPOINT_URL_STS")
            .kill_on_drop(true)
            .stdin(std::process::Stdio::null())
            .stderr(std::process::Stdio::null());
        let output = tokio::time::timeout(std::time::Duration::from_secs(10), command.output())
            .await
            .map_err(|_| "alert_transport_timeout")?
            .map_err(|_| "alert_transport_unavailable")?;
        if !output.status.success() {
            return Err("alert_transport_failed");
        }
        serde_json::from_slice(&output.stdout).map_err(|_| "alert_transport_invalid_receipt")
    }
    pub async fn publish(&self, incident: &ResidentIncident) -> Result<(), &'static str> {
        self.publish_using(std::path::Path::new("aws"), incident)
            .await
    }
    async fn publish_using(
        &self,
        program: &std::path::Path,
        incident: &ResidentIncident,
    ) -> Result<(), &'static str> {
        self.validate()?;
        let identity = self
            .command(program, &["sts", "get-caller-identity"])
            .await?;
        if !self.accepts_identity(&identity) {
            return Err("alert_role_mismatch");
        }
        // Structured, bounded facts only: never include prompts, response text,
        // provider errors, endpoint URLs, or credential material.
        let message = serde_json::json!({
            "schema":"adl.runtime.resident_help.v1", "incident_id":incident.incident_id, "revision":incident.revision,
            "resident_id":incident.resident_id, "reason":incident.reason,
            "urgency":"operator_attention", "response_attempts":incident.response_attempts,
            "response_status":incident.response_status, "state":incident.state,
            "opened_at_unix_millis":incident.opened_at_unix_millis,
        })
        .to_string();
        let receipt = self
            .command(
                program,
                &[
                    "sns",
                    "publish",
                    "--topic-arn",
                    &self.topic_arn,
                    "--message",
                    &message,
                ],
            )
            .await?;
        if receipt["MessageId"].as_str().is_none_or(str::is_empty) {
            return Err("alert_transport_missing_receipt");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn failed(id: &str) -> ResidentObservation {
        ResidentObservation {
            id: id.into(),
            binding: "binding-1".into(),
            unhealthy: true,
            reason: "observed_health_failure",
            inference_verified: false,
            inference_observed_at_unix_millis: 0,
            present: true,
        }
    }
    #[test]
    fn simultaneous_failures_and_offline_shepherd_are_independently_owned_and_deduplicated() {
        let mut s = ResidentHealthSupervisor::default();
        let observations = [failed("harbor"), failed("quill"), failed("beacon")];
        for n in 1..20 {
            s.observe(&observations, false, n * 5_000).unwrap();
        }
        assert_eq!(s.snapshot().len(), 3);
        assert!(s
            .snapshot()
            .iter()
            .all(|i| i.escalated && i.response_attempts == 0));
        let mut ids = std::collections::BTreeSet::new();
        for _ in 0..3 {
            ids.insert(s.reserve_alert(100_000).unwrap().unwrap().resident_id);
        }
        assert_eq!(ids.len(), 3);
        assert!(s.reserve_alert(100_000).unwrap().is_none());
    }
    #[test]
    fn unknown_and_metadata_success_cannot_resolve_an_incident() {
        let mut s = ResidentHealthSupervisor::default();
        let mut o = failed("resident");
        s.observe(std::slice::from_ref(&o), true, 100).unwrap();
        o.unhealthy = false;
        s.observe(std::slice::from_ref(&o), true, 200).unwrap();
        assert_eq!(s.snapshot()[0].state, IncidentState::Open);
        o.inference_verified = true;
        o.inference_observed_at_unix_millis = 50;
        s.observe(std::slice::from_ref(&o), true, 300).unwrap();
        assert_eq!(s.snapshot()[0].state, IncidentState::Open);
        o.inference_observed_at_unix_millis = 301;
        s.observe(&[o], true, 302).unwrap();
        assert_eq!(s.snapshot()[0].state, IncidentState::Recovered);
    }
    #[test]
    fn replaced_binding_fences_old_response_and_retains_incident() {
        let mut s = ResidentHealthSupervisor::default();
        let mut o = failed("resident");
        s.observe(std::slice::from_ref(&o), true, 100).unwrap();
        let old = s.reserve_response(100).unwrap().unwrap();
        o.binding = "binding-2".into();
        s.observe(&[o], true, 200).unwrap();
        s.finish_response(&old, true, 300).unwrap();
        assert_eq!(s.snapshot()[0].response_status, "binding_changed");
        assert_eq!(s.snapshot()[0].incident_id, old.incident_id);
    }
    #[test]
    fn timeout_escalates_and_response_budget_is_bounded() {
        let mut s = ResidentHealthSupervisor::default();
        let o = failed("resident");
        s.observe(std::slice::from_ref(&o), true, 100).unwrap();
        for n in 0..3 {
            let now = 100 + n * 1_000_000;
            let response = s.reserve_response(now).unwrap().unwrap();
            s.observe(
                std::slice::from_ref(&o),
                true,
                now + RESPONSE_TIMEOUT_MILLIS,
            )
            .unwrap();
            s.finish_response(&response, true, now + RESPONSE_TIMEOUT_MILLIS + 1)
                .unwrap();
            assert_eq!(s.snapshot()[0].response_status, "response_timeout");
        }
        assert!(s.reserve_response(10_000_000).unwrap().is_none());
        assert!(s.snapshot()[0].escalated);
    }
    #[test]
    fn restart_keeps_reserved_delivery_and_retries_after_backoff() {
        let temp = tempfile::tempdir().unwrap();
        let path = temp.path().join("incidents.json");
        let mut s = ResidentHealthSupervisor::open(path.clone()).unwrap();
        s.observe(&[failed("resident")], false, 100).unwrap();
        let original = s.reserve_alert(100).unwrap().unwrap();
        drop(s);
        let mut s = ResidentHealthSupervisor::open(path).unwrap();
        assert!(s.reserve_alert(101).unwrap().is_none());
        let retry = s
            .reserve_alert(original.next_alert_at_unix_millis)
            .unwrap()
            .unwrap();
        assert_eq!(retry.incident_id, original.incident_id);
        s.finish_alert(&retry, true).unwrap();
        assert!(s.reserve_alert(u64::MAX).unwrap().is_none());
        assert_eq!(s.snapshot()[0].alert_status, "sns_accepted");
    }
    #[test]
    fn removal_preserves_pending_alert_and_readdition_creates_new_incident() {
        let mut s = ResidentHealthSupervisor::default();
        s.observe(&[failed("resident")], false, 100).unwrap();
        s.observe(&[], false, 200).unwrap();
        assert_eq!(s.snapshot()[0].state, IncidentState::Retired);
        assert!(s.reserve_alert(200).unwrap().is_some());
        s.observe(&[failed("resident")], false, 300).unwrap();
        assert_eq!(s.snapshot().len(), 2);
    }
    #[test]
    fn explicit_help_deduplicates_and_escalates_without_failed_model() {
        let mut s = ResidentHealthSupervisor::default();
        let o = failed("beacon");
        s.request_help(&o, 100).unwrap();
        s.request_help(&o, 200).unwrap();
        assert_eq!(s.snapshot().len(), 1);
        assert_eq!(s.snapshot()[0].reason, "explicit_help_request");
        assert!(s.reserve_alert(200).unwrap().is_some());
    }
    fn alert_config() -> ResidentAlertConfig {
        ResidentAlertConfig {
            profile: "test-shepherd".into(),
            region: "us-west-2".into(),
            topic_arn: "arn:aws:sns:us-west-2:111122223333:health".into(),
            role_arn: "arn:aws:iam::111122223333:role/shepherd".into(),
        }
    }
    #[test]
    fn alert_transport_rejects_admin_wrong_account_and_wrong_role() {
        let config = alert_config();
        assert!(config.validate().is_ok());
        for arn in [
            "arn:aws:iam::111122223333:user/admin",
            "arn:aws:sts::111122223333:assumed-role/admin/session",
            "arn:aws:sts::444455556666:assumed-role/shepherd/session",
        ] {
            assert!(
                !config.accepts_identity(&serde_json::json!({"Account":"111122223333","Arn":arn}))
            );
        }
        assert!(config.accepts_identity(&serde_json::json!({"Account":"111122223333","Arn":"arn:aws:sts::111122223333:assumed-role/shepherd/session"})));
        let mut wrong = config.clone();
        wrong.topic_arn = "arn:aws:sns:us-west-2:444455556666:health".into();
        assert!(wrong.validate().is_err());
    }
    #[test]
    fn verification_only_incidents_do_not_trigger_periodic_inference() {
        let mut s = ResidentHealthSupervisor::default();
        let mut o = failed("beacon");
        o.reason = "inference_evidence_stale";
        s.observe(std::slice::from_ref(&o), true, 100).unwrap();
        for now in [200, 40_000, 400_000] {
            s.observe(std::slice::from_ref(&o), true, now).unwrap();
            assert!(s.reserve_response(now).unwrap().is_none());
        }
        assert_eq!(s.snapshot().len(), 1);
        assert!(s.snapshot()[0].escalated);
    }

    #[test]
    fn worsening_health_rearms_alert_and_rejects_old_delivery_receipt() {
        let mut s = ResidentHealthSupervisor::default();
        let mut o = failed("resident");
        o.reason = "inference_unverified";
        s.observe(std::slice::from_ref(&o), false, 100).unwrap();
        let first = s.reserve_alert(100).unwrap().unwrap();
        s.finish_alert(&first, true).unwrap();
        o.reason = "observed_health_failure";
        s.observe(&[o], false, 200).unwrap();
        s.finish_alert(&first, true).unwrap();
        assert!(!s.snapshot()[0].alert_delivered);
        let next = s.reserve_alert(200).unwrap().unwrap();
        assert_eq!(next.reason, "observed_health_failure");
        assert_eq!(next.revision, first.revision + 1);
        s.finish_alert(&next, true).unwrap();
        assert!(s.snapshot()[0].alert_delivered);
    }

    #[test]
    fn queued_incidents_escalate_without_waiting_for_response_slots() {
        let mut s = ResidentHealthSupervisor::default();
        let observations = (0..25)
            .map(|n| failed(&format!("resident-{n}")))
            .collect::<Vec<_>>();
        s.observe(&observations, true, 100).unwrap();
        s.reserve_response(100).unwrap().unwrap();
        s.observe(&observations, true, 100 + RESPONSE_TIMEOUT_MILLIS)
            .unwrap();
        assert_eq!(s.snapshot().iter().filter(|i| i.escalated).count(), 25);
    }

    #[cfg(unix)]
    #[tokio::test]
    async fn sns_transport_requires_receipt_and_retries_failure_without_model() {
        use std::os::unix::fs::PermissionsExt;
        let temp = tempfile::tempdir().unwrap();
        let program = temp.path().join("aws-fixture");
        let mut s = ResidentHealthSupervisor::open(temp.path().join("journal.json")).unwrap();
        s.observe(&[failed("beacon")], false, 100).unwrap();
        let incident = s.reserve_alert(100).unwrap().unwrap();
        let config = alert_config();
        for (receipt, expected) in [("{}", false), (r#"{"MessageId":"fixture-accepted"}"#, true)] {
            let script = format!(
                r#"#!/bin/sh
case "$*" in
 *"sts get-caller-identity"*) printf '%s' '{{"Account":"111122223333","Arn":"arn:aws:sts::111122223333:assumed-role/shepherd/test"}}' ;;
 *"sns publish --topic-arn arn:aws:sns:us-west-2:111122223333:health"*) printf '%s' '{}' ;;
 *) exit 91 ;;
esac
"#,
                receipt
            );
            fs::write(&program, script).unwrap();
            fs::set_permissions(&program, fs::Permissions::from_mode(0o700)).unwrap();
            let accepted = config.publish_using(&program, &incident).await.is_ok();
            assert_eq!(accepted, expected);
            s.finish_alert(&incident, accepted).unwrap();
            assert_eq!(s.snapshot()[0].alert_delivered, expected);
        }
        let script = r#"#!/bin/sh
case "$*" in
 *"sts get-caller-identity"*) printf '%s' '{"Account":"111122223333","Arn":"arn:aws:iam::111122223333:user/admin"}' ;;
 *) exit 92 ;;
esac
"#;
        fs::write(&program, script).unwrap();
        assert_eq!(
            config.publish_using(&program, &incident).await.unwrap_err(),
            "alert_role_mismatch"
        );
    }

    #[test]
    fn failed_persistence_does_not_publish_memory_only_incident() {
        let temp = tempfile::tempdir().unwrap();
        let parent = temp.path().join("parent");
        fs::write(&parent, b"file").unwrap();
        let mut s = ResidentHealthSupervisor {
            journal: Journal::default(),
            path: Some(parent.join("state.json")),
        };
        assert!(s.observe(&[failed("resident")], false, 100).is_err());
        assert!(s.snapshot().is_empty());
    }
}
