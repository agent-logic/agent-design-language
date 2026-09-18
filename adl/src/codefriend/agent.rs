//! Installed agent authority. Website commands select locally approved evidence;
//! they never provide paths, executable commands, or provider credentials.
use super::{
    evidence::hash,
    ingestion::{self, Scope},
};
use anyhow::{ensure, Result};
use reqwest::{blocking::Client, Method, Url};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Path, PathBuf},
    time::Duration,
};

pub const PROTOCOL: &str = "codefriend.local_agent.v1";
const MAX_RESPONSE: u64 = 4 * 1024 * 1024;
fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 80
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}
fn credential(s: &str) -> bool {
    (43..=512).contains(&s.len())
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Consent {
    pub schema: String,
    pub repository_path: PathBuf,
    pub repository: String,
    pub revision: String,
    pub scope: Scope,
    pub expires_at: u64,
    pub retention_seconds: u64,
    /// Selected evidence goes to Agent Logic and its configured model provider.
    pub allow_model_egress: bool,
    /// Upload the validated review record, including its selected admitted evidence,
    /// to the paired website for artifact inspection and approval.
    pub allow_result_upload: bool,
}
impl Consent {
    pub fn validate(&self, now: u64) -> Result<()> {
        ensure!(self.schema == PROTOCOL, "agent_consent_schema");
        self.scope.validate()?;
        ingestion::validate_repository(&self.repository)?;
        ensure!(
            self.repository_path.is_absolute() && self.repository_path.is_dir(),
            "agent_repository_path"
        );
        ensure!(
            matches!(self.revision.len(), 40 | 64)
                && self
                    .revision
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
            "agent_revision"
        );
        ensure!(
            self.expires_at > now && (60..=86400).contains(&self.retention_seconds),
            "agent_consent_expired_or_retention"
        );
        ensure!(
            self.allow_model_egress && self.allow_result_upload,
            "agent_consent_required"
        );
        Ok(())
    }
    /// Digest includes the private local path; only the digest is advertised.
    pub fn digest(&self) -> Result<String> {
        hash(self)
    }
}

/// Secrets deliberately do not implement Debug and never enter status reports.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pairing {
    pub schema: String,
    pub origin: String,
    pub agent_id: String,
    pub subject: String,
    pub agent_token: String,
    pub model_token: String,
    pub expires_at: u64,
}
impl Pairing {
    pub fn validate(&self, now: u64) -> Result<()> {
        ensure!(
            self.schema == PROTOCOL && identifier(&self.agent_id) && identifier(&self.subject),
            "agent_pairing_identity"
        );
        ensure!(
            credential(&self.agent_token)
                && credential(&self.model_token)
                && self.agent_token != self.model_token,
            "agent_pairing_credentials"
        );
        ensure!(self.expires_at > now, "agent_pairing_expired");
        Ok(())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Command {
    pub schema: String,
    pub agent_id: String,
    pub subject: String,
    pub run_id: String,
    pub consent_digest: String,
    pub expires_at: u64,
}
impl Command {
    pub fn validate(&self, pairing: &Pairing, consent: &Consent, now: u64) -> Result<()> {
        pairing.validate(now)?;
        consent.validate(now)?;
        ensure!(
            self.schema == PROTOCOL && identifier(&self.run_id),
            "agent_command_schema"
        );
        ensure!(
            self.agent_id == pairing.agent_id && self.subject == pairing.subject,
            "agent_command_identity"
        );
        ensure!(
            self.consent_digest == consent.digest()?,
            "agent_command_consent"
        );
        ensure!(
            self.expires_at > now
                && self.expires_at <= pairing.expires_at
                && self.expires_at <= consent.expires_at,
            "agent_command_expired"
        );
        Ok(())
    }
}

/// Fixed-origin, bounded HTTP transport. Redirects and ambient proxy settings are
/// disabled so scoped credentials cannot migrate to another host.
#[derive(Clone)]
pub struct Transport {
    client: Client,
    origin: Url,
    clock: std::sync::Arc<dyn Fn() -> u64 + Send + Sync>,
}
impl Transport {
    pub fn new(origin: &str) -> Result<Self> {
        Self::build(origin, false)
    }
    pub fn loopback_fixture(origin: &str) -> Result<Self> {
        Self::build(origin, true)
    }
    fn build(origin: &str, fixture: bool) -> Result<Self> {
        let url = Url::parse(origin).map_err(|_| anyhow::anyhow!("agent_origin_invalid"))?;
        ensure!(
            url.username().is_empty()
                && url.password().is_none()
                && url.query().is_none()
                && url.fragment().is_none()
                && url.path() == "/",
            "agent_origin_invalid"
        );
        let loopback = url.host_str().is_some_and(|h| {
            h.parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
        });
        ensure!(
            if fixture {
                url.scheme() == "http" && loopback
            } else {
                url.scheme() == "https" && url.host_str().is_some()
            },
            "agent_https_required"
        );
        let client = Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .no_proxy()
            .connect_timeout(Duration::from_secs(5))
            .timeout(Duration::from_secs(15))
            .build()?;
        Ok(Self {
            client,
            origin: url,
            clock: std::sync::Arc::new(clock),
        })
    }
    pub fn request<T: DeserializeOwned>(
        &self,
        method: Method,
        path: &str,
        token: Option<&str>,
        body: Option<&serde_json::Value>,
    ) -> Result<T> {
        ensure!(
            path.starts_with("/v1/")
                && !path.contains(['?', '#', '\\'])
                && !path.split('/').any(|s| s == "." || s == ".."),
            "agent_protocol_path"
        );
        let url = self.origin.join(path)?;
        ensure!(url.origin() == self.origin.origin(), "agent_origin_changed");
        let mut request = self.client.request(method, url);
        if let Some(token) = token {
            ensure!(credential(token), "agent_credential_invalid");
            request = request.bearer_auth(token);
        }
        if let Some(body) = body {
            request = request.json(body);
        }
        let response = request
            .send()
            .map_err(|_| anyhow::anyhow!("agent_transport_unknown_effect"))?;
        ensure!(response.status().is_success(), "agent_service_rejected");
        ensure!(
            response.content_length().unwrap_or(0) <= MAX_RESPONSE,
            "agent_response_limit"
        );
        let mut bytes = Vec::new();
        response
            .take(MAX_RESPONSE + 1)
            .read_to_end(&mut bytes)
            .map_err(|_| anyhow::anyhow!("agent_response_incomplete"))?;
        ensure!(bytes.len() as u64 <= MAX_RESPONSE, "agent_response_limit");
        serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("agent_response_invalid"))
    }
    /// A failed/uncertain exchange must not be automatically repeated. Request a
    /// new website pairing code after inspecting/revoking the earlier pairing.
    pub fn pair(&self, code: &str, now: u64) -> Result<Pairing> {
        ensure!(credential(code), "agent_pairing_code_invalid");
        let pairing: Pairing = self.request(
            Method::POST,
            "/v1/agent/pair",
            None,
            Some(&serde_json::json!({"schema":PROTOCOL,"code":code})),
        )?;
        pairing.validate(now)?;
        ensure!(
            pairing.origin == self.origin.as_str(),
            "agent_pairing_origin"
        );
        Ok(pairing)
    }
}

/// Exclusive local ownership and create-only run reservation. Presence of a run
/// directory forbids replaying dispatched operations, including after a crash.
pub struct Journal {
    root: PathBuf,
    _lock: File,
}
impl Journal {
    pub fn open(root: &Path) -> Result<Self> {
        ensure!(
            root.is_absolute() && root.parent().is_some_and(Path::is_dir),
            "agent_store_parent"
        );
        if !root.exists() {
            fs::create_dir(root)?;
            fs::set_permissions(root, fs::Permissions::from_mode(0o700))?;
        }
        let metadata = fs::symlink_metadata(root)?;
        ensure!(
            metadata.is_dir() && metadata.permissions().mode() & 0o077 == 0,
            "agent_store_permissions"
        );
        let lock = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(false)
            .mode(0o600)
            .open(root.join("lock"))?;
        fs2::FileExt::try_lock_exclusive(&lock)
            .map_err(|_| anyhow::anyhow!("agent_store_in_use"))?;
        File::open(root.parent().unwrap())?.sync_all()?;
        Ok(Self {
            root: root.into(),
            _lock: lock,
        })
    }
    pub fn save_pairing(&self, pairing: &Pairing, now: u64) -> Result<()> {
        pairing.validate(now)?;
        self.create_private("pairing.json", &serde_json::to_vec(pairing)?)
    }
    pub fn pairing(&self, now: u64) -> Result<Pairing> {
        let path = self.root.join("pairing.json");
        let metadata = fs::symlink_metadata(&path)?;
        ensure!(
            metadata.is_file()
                && metadata.permissions().mode() & 0o077 == 0
                && metadata.len() <= 8192,
            "agent_pairing_file_permissions"
        );
        let value: Pairing = serde_json::from_slice(&fs::read(path)?)
            .map_err(|_| anyhow::anyhow!("agent_pairing_invalid"))?;
        value.validate(now)?;
        Ok(value)
    }
    fn create_private(&self, name: &str, bytes: &[u8]) -> Result<()> {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(self.root.join(name))?;
        file.write_all(bytes)?;
        file.sync_all()?;
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }
    pub fn reserve(
        &self,
        command: &Command,
        pairing: &Pairing,
        consent: &Consent,
        now: u64,
    ) -> Result<PathBuf> {
        command.validate(pairing, consent, now)?;
        let dir = self.root.join(format!("run-{}", command.run_id));
        fs::create_dir(&dir).map_err(|_| anyhow::anyhow!("agent_run_already_reserved"))?;
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700))?;
        File::open(&self.root)?.sync_all()?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .mode(0o600)
            .open(dir.join("command.json"))?;
        file.write_all(&serde_json::to_vec(command)?)?;
        file.sync_all()?;
        File::open(&dir)?.sync_all()?;
        Ok(dir)
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PollReply {
    schema: String,
    command: Option<Command>,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Control {
    schema: String,
    agent_id: String,
    subject: String,
    run_id: String,
    cancelled: bool,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatewayLaneIdentity {
    pub lane: String,
    pub candidate_revision: String,
    pub model_identity: crate::model_identity::ModelIdentityV1,
}
impl GatewayLaneIdentity {
    fn validate(&self) -> Result<()> {
        ensure!(
            self.candidate_revision.len() == 40
                && self
                    .candidate_revision
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
                && self.candidate_revision.bytes().any(|b| b != b'0'),
            "agent_gateway_candidate"
        );
        ensure!(
            super::review::lanes::ReviewLane::ALL
                .iter()
                .any(|lane| lane.id() == self.lane),
            "agent_gateway_lane"
        );
        crate::model_identity::validate_model_identity_v1(&self.model_identity)?;
        ensure!(
            !matches!(
                self.model_identity.identity_strength,
                crate::model_identity::ModelIdentityStrengthV1::Unknown
            ),
            "agent_gateway_model_unknown"
        );
        Ok(())
    }
    fn same_execution(&self, other: &Self) -> bool {
        let a = &self.model_identity;
        let b = &other.model_identity;
        self.candidate_revision == other.candidate_revision
            && a.provider_kind == b.provider_kind
            && a.provider == b.provider
            && a.model_ref == b.model_ref
            && a.provider_model_id == b.provider_model_id
            && a.runtime_surface == b.runtime_surface
            && a.identity_strength == b.identity_strength
            && a.resolved_digest == b.resolved_digest
    }
    fn route(&self) -> Result<String> {
        // Contract identifiers are bounded; bind the full retained identity tuple
        // by digest rather than truncating model IDs or losing their attribution.
        Ok(format!(
            "agent_logic_gateway:{}",
            hash(&(
                &self.candidate_revision,
                &self.model_identity.provider_kind,
                &self.model_identity.provider,
                &self.model_identity.runtime_surface,
                &self.model_identity.model_ref,
                &self.model_identity.provider_model_id,
                &self.model_identity.identity_strength,
                &self.model_identity.resolved_digest
            ))?
        ))
    }
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RunReport {
    pub schema: String,
    pub agent_id: String,
    pub subject: String,
    pub run_id: String,
    pub consent_digest: String,
    pub execution_location: String,
    pub gateway_lanes: Vec<GatewayLaneIdentity>,
    pub status: String,
    pub expires_at: u64,
    pub result: Option<super::review::runner::FourPerspectiveReviewRun>,
    pub digest: String,
}
impl RunReport {
    /// Validate received website artifacts using their native typed serialization.
    /// This proves contract integrity, not independent provider execution.
    pub fn validate(&self, now: u64) -> Result<()> {
        use super::evidence::{contracts::Completion, valid_digest};
        use super::review::{
            lanes::{ReviewLane, LANE_CONTRACT_VERSION},
            runner,
        };
        ensure!(
            self.schema == PROTOCOL
                && identifier(&self.agent_id)
                && identifier(&self.subject)
                && identifier(&self.run_id)
                && valid_digest(&self.consent_digest)
                && self.execution_location == "local_agent"
                && self.expires_at > now,
            "agent_report_identity"
        );
        ensure!(
            serde_json::to_vec(self)?.len() as u64 <= MAX_RESPONSE,
            "agent_report_limit"
        );
        let mut unsigned: serde_json::Value = serde_json::to_value(self)?;
        unsigned["digest"] = serde_json::Value::String(String::new());
        let unsigned: RunReport = serde_json::from_value(unsigned)?;
        ensure!(self.digest == hash(&unsigned)?, "agent_report_digest");
        for identity in &self.gateway_lanes {
            identity.validate()?;
        }
        if let Some(first) = self.gateway_lanes.first() {
            ensure!(
                self.gateway_lanes
                    .iter()
                    .all(|identity| first.same_execution(identity)),
                "agent_gateway_identity_changed"
            );
        }
        match (&self.result, self.status.as_str()) {
            (Some(result), "complete") => {
                ensure!(
                    result.schema == runner::REVIEW_RUN_SCHEMA
                        && result.run_id == self.run_id
                        && result.completion == Completion::Complete
                        && result.failures.is_empty(),
                    "agent_report_completion"
                );
                result.review_record.validate()?;
                ensure!(
                    self.gateway_lanes.len() == ReviewLane::ALL.len(),
                    "agent_gateway_identities_missing"
                );
                let first = &self.gateway_lanes[0];
                ensure!(
                    result.review_record.run.provider_route == first.route()?,
                    "agent_gateway_route"
                );
                for lane in ReviewLane::ALL {
                    ensure!(
                        self.gateway_lanes
                            .iter()
                            .filter(|identity| identity.lane == lane.id())
                            .count()
                            == 1,
                        "agent_gateway_lane_identity"
                    );
                }
                let record = &result.review_record;
                ensure!(
                    record.run.completion == Completion::Complete
                        && record.run.failures.is_empty()
                        && record.admission.expires_at == self.expires_at
                        && record.admission.expires_at > now
                        && result.lane_results.len() == ReviewLane::ALL.len(),
                    "agent_report_review"
                );
                ensure!(
                    record.run.lane_versions.len() == ReviewLane::ALL.len()
                        && record.findings.iter().all(|finding| ReviewLane::ALL
                            .iter()
                            .any(|lane| lane.id() == finding.perspective)),
                    "agent_report_perspectives"
                );
                for lane in ReviewLane::ALL {
                    ensure!(
                        record
                            .run
                            .lane_versions
                            .get(lane.id())
                            .is_some_and(|version| version == LANE_CONTRACT_VERSION),
                        "agent_report_lane_version"
                    );
                    let found: Vec<_> = result
                        .lane_results
                        .iter()
                        .filter(|item| item.lane == lane.id())
                        .collect();
                    ensure!(found.len() == 1, "agent_report_lane");
                    let item = found[0];
                    let (manifest, _) =
                        runner::lane_input_manifest(&self.run_id, lane, &record.admission)?;
                    let mut findings: Vec<_> = record
                        .findings
                        .iter()
                        .filter(|finding| finding.perspective == lane.id())
                        .map(|finding| finding.id.clone())
                        .collect();
                    findings.sort();
                    ensure!(item.schema == runner::LANE_RESULT_SCHEMA && item.run_id == self.run_id
                        && item.lane_contract == LANE_CONTRACT_VERSION && item.input_digest == manifest.input_digest
                            && item.input_manifest_ref == format!("lanes/{}/input.json", lane.id())
                            && item.provider_route == record.run.provider_route
                        && item.provider_status == crate::provider_communication::ProviderInvocationFinalStatusV1::Ok
                        && item.failure.is_none() && item.finding_ids == findings
                        && item.output_digest.as_deref().is_some_and(valid_digest), "agent_report_lane_integrity");
                }
            }
            (None, "failed_or_interrupted" | "interrupted") => {}
            _ => anyhow::bail!("agent_report_completion"),
        }
        Ok(())
    }
}

fn clock() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn save_private(path: &Path, value: &impl Serialize) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .mode(0o600)
        .open(path)?;
    file.write_all(&serde_json::to_vec(value)?)?;
    file.sync_all()?;
    File::open(
        path.parent()
            .ok_or_else(|| anyhow::anyhow!("agent_storage_parent"))?,
    )?
    .sync_all()?;
    Ok(())
}
#[derive(Debug)]
struct ObservationPending;
impl std::fmt::Display for ObservationPending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("agent_known_operation_observation_pending")
    }
}
impl std::error::Error for ObservationPending {}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct AcknowledgedOperation {
    operation: super::server::Operation,
    observation_deadline: u64,
}
impl Transport {
    pub fn loopback_fixture_with_clock(
        origin: &str,
        clock: std::sync::Arc<dyn Fn() -> u64 + Send + Sync>,
    ) -> Result<Self> {
        let mut transport = Self::loopback_fixture(origin)?;
        transport.clock = clock;
        Ok(transport)
    }
    fn paired(&self, pairing: &Pairing) -> Result<()> {
        pairing.validate((self.clock)())?;
        ensure!(
            pairing.origin == self.origin.as_str(),
            "agent_pairing_origin"
        );
        Ok(())
    }
    fn control(&self, pairing: &Pairing, command: &Command) -> Result<bool> {
        self.paired(pairing)?;
        let response: Control = self.request(
            Method::GET,
            &format!("/v1/agent/runs/{}/control", command.run_id),
            Some(&pairing.agent_token),
            None,
        )?;
        ensure!(
            response.schema == PROTOCOL
                && response.agent_id == command.agent_id
                && response.subject == command.subject
                && response.run_id == command.run_id,
            "agent_control_identity"
        );
        Ok(response.cancelled)
    }
    fn model_lane(
        &self,
        authority: &RunAuthority<'_>,
        packet: &ingestion::Packet,
        lane: super::review::lanes::ReviewLane,
        dir: &Path,
    ) -> Result<(super::review::runner::LaneExecution, GatewayLaneIdentity)> {
        use super::server::{Mode, Operation, Status, Submit};
        let pairing = authority.pairing;
        let command = authority.command;
        authority.check((self.clock)())?;
        ensure!(
            !self
                .control(pairing, command)
                .map_err(|_| ObservationPending)?,
            "agent_cancelled"
        );
        authority.check((self.clock)())?;
        // Includes agent identity to avoid collisions between a user's paired machines.
        let operation_id = hash(&(PROTOCOL, &pairing.agent_id, &command.run_id, lane.id()))?;
        fs::create_dir_all(dir)?;
        // The reservation must survive losing this process after POST. Persist
        // every new directory entry, not just the file inside the lane directory.
        let gateway = dir
            .parent()
            .ok_or_else(|| anyhow::anyhow!("agent_gateway_parent"))?;
        let run_root = gateway
            .parent()
            .ok_or_else(|| anyhow::anyhow!("agent_run_parent"))?;
        fs::set_permissions(gateway, fs::Permissions::from_mode(0o700))?;
        fs::set_permissions(dir, fs::Permissions::from_mode(0o700))?;
        for directory in [dir, gateway, run_root] {
            File::open(directory)?.sync_all()?;
        }
        let path = format!("/v1/operations/{operation_id}");
        let submit = Submit {
            operation_id: operation_id.clone(),
            packet: packet.clone(),
            mode: Mode::LocalModel,
            lane: Some(lane),
        };
        let validate_operation = |operation: &Operation| -> Result<()> {
            ensure!(
                operation.operation_id == operation_id
                    && operation.subject == pairing.subject
                    && operation.mode == Mode::LocalModel
                    && operation.packet_id == packet.packet_id
                    && operation.source_revision == packet.revision
                    && operation.request_digest == hash(&submit)?,
                "agent_gateway_identity"
            );
            Ok(())
        };
        let acknowledged = dir.join("acknowledged-operation.json");
        let known: AcknowledgedOperation = if acknowledged.exists() {
            serde_json::from_slice(&fs::read(&acknowledged)?)?
        } else {
            // Create-only reservation is the no-replay barrier, including lost POST replies.
            save_private(&dir.join("gateway-reservation.json"), &submit.operation_id)?;
            let operation: Operation = self.request(
                Method::POST,
                "/v1/operations",
                Some(&pairing.model_token),
                Some(&serde_json::to_value(&submit)?),
            )?;
            validate_operation(&operation)?;
            let known = AcknowledgedOperation {
                operation,
                observation_deadline: (self.clock)().saturating_add(120).min(authority.expires_at),
            };
            save_private(&acknowledged, &known)?;
            known
        };
        validate_operation(&known.operation)?;
        let mut operation = known.operation;
        let output_path = dir.join("gateway-result.json");
        let mut stopping = false;
        loop {
            validate_operation(&operation)?;
            let allowed = authority.check((self.clock)()).is_ok();
            let cancelled = if allowed {
                self.control(pairing, command)
                    .map_err(|_| ObservationPending)?
            } else {
                true
            };
            if !allowed
                || cancelled
                || (!output_path.exists() && (self.clock)() >= known.observation_deadline)
            {
                // Cancellation is best effort; stopping local work is not a claim
                // that an in-flight remote provider effect was undone.
                let _: Result<Operation> = self.request(
                    Method::POST,
                    &format!("{path}/cancel"),
                    Some(&pairing.model_token),
                    None,
                );
                stopping = true;
            }
            if stopping {
                anyhow::bail!("agent_stopped_remote_effect_may_continue");
            }
            if output_path.exists() {
                break;
            }
            match operation.status {
                Status::Complete => break,
                Status::Running => {}
                _ => anyhow::bail!("agent_gateway_not_complete"),
            }
            std::thread::sleep(Duration::from_millis(250));
            operation = self
                .request(Method::GET, &path, Some(&pairing.model_token), None)
                .map_err(|_| ObservationPending)?;
        }
        #[derive(Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        struct ModelResult {
            schema: String,
            execution_location: String,
            model_execution_location: String,
            candidate_revision: String,
            model_identity: crate::model_identity::ModelIdentityV1,
            input_manifest: super::review::runner::LaneInputManifest,
            output: super::review::runner::ProviderLaneOutput,
        }
        let result: ModelResult = if output_path.exists() {
            serde_json::from_slice(&fs::read(&output_path)?)?
        } else {
            self.request(
                Method::GET,
                &format!("{path}/result"),
                Some(&pairing.model_token),
                None,
            )
            .map_err(|_| ObservationPending)?
        };
        ensure!(
            result.schema == "codefriend.local_model_result.v1"
                && result.execution_location == "local_agent"
                && result.model_execution_location == "agent_logic_provider"
                && result.input_manifest.run_id == operation_id
                && result.input_manifest.packet_id == packet.packet_id
                && result.input_manifest.lane == lane.id()
                && result.input_manifest.revision == packet.revision
                && result.input_manifest.repository == packet.repository
                && result.input_manifest.scope_digest == packet.scope_digest,
            "agent_gateway_result_identity"
        );
        let identity = GatewayLaneIdentity {
            lane: lane.id().into(),
            candidate_revision: result.candidate_revision.clone(),
            model_identity: result.model_identity.clone(),
        };
        identity.validate()?;
        if operation.status == Status::Complete {
            let observed = operation
                .model_identity
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("agent_gateway_operation_model_missing"))?;
            let operation_identity = GatewayLaneIdentity {
                lane: lane.id().into(),
                candidate_revision: operation.candidate_revision.clone(),
                model_identity: observed.clone(),
            };
            operation_identity.validate()?;
            ensure!(
                identity.same_execution(&operation_identity),
                "agent_gateway_operation_model_changed"
            );
        }
        ensure!(
            identity.candidate_revision == operation.candidate_revision,
            "agent_gateway_candidate_changed"
        );
        if !output_path.exists() {
            save_private(&output_path, &result)?;
        }
        Ok((
            super::review::runner::LaneExecution {
                final_status: crate::provider_communication::ProviderInvocationFinalStatusV1::Ok,
                output_text: Some(serde_json::to_string(&result.output)?),
            },
            identity,
        ))
    }
    /// Executes or resumes one review. Reconnection observes acknowledged operations
    /// and reuses completed lanes; no dispatched POST is replayed.
    pub fn poll_once(&self, journal: &Journal, consent_path: &Path) -> Result<Option<String>> {
        journal.expire((self.clock)())?;
        let result = self.poll_inner(journal, consent_path);
        journal.expire((self.clock)())?;
        result
    }
    fn poll_inner(&self, journal: &Journal, consent_path: &Path) -> Result<Option<String>> {
        let consent = read_consent(consent_path, (self.clock)())?;
        let pairing = journal.pairing((self.clock)())?;
        self.paired(&pairing)?;
        consent.validate((self.clock)())?;
        let poll: PollReply = self.request(Method::POST,"/v1/agent/poll",Some(&pairing.agent_token),Some(&serde_json::json!({"schema":PROTOCOL,"agent_id":pairing.agent_id,"consent_digest":consent.digest()?})))?;
        ensure!(poll.schema == PROTOCOL, "agent_poll_schema");
        let Some(command) = poll.command else {
            return Ok(None);
        };
        command.validate(&pairing, &consent, (self.clock)())?;
        let existing = journal.root.join(format!("run-{}", command.run_id));
        let resuming = existing.exists()
            && !existing.join("report.json").exists()
            && existing.join("admission.json").exists()
            && super::review::lanes::ReviewLane::ALL.iter().any(|lane| {
                existing
                    .join("gateway")
                    .join(lane.id())
                    .join("acknowledged-operation.json")
                    .exists()
            });
        if existing.exists() && !resuming {
            self.forward(journal, &pairing, &command, consent_path)?;
            return Ok(Some(command.run_id));
        }
        let dir = if resuming {
            let saved: Command = serde_json::from_slice(&fs::read(existing.join("command.json"))?)?;
            ensure!(
                hash(&saved)? == hash(&command)?,
                "agent_duplicate_command_changed"
            );
            existing
        } else {
            journal.reserve(&command, &pairing, &consent, (self.clock)())?
        };
        let expires_at = if resuming {
            serde_json::from_slice(&fs::read(dir.join("expires.json"))?)?
        } else {
            let expiry = (self.clock)()
                .saturating_add(consent.retention_seconds)
                .min(consent.expires_at)
                .min(pairing.expires_at)
                .min(command.expires_at);
            save_private(&dir.join("expires.json"), &expiry)?;
            expiry
        };
        let authority = RunAuthority {
            pairing: &pairing,
            command: &command,
            consent_path,
            expires_at,
        };
        let mut gateway_lanes = Vec::new();
        let run = (|| {
            authority.check((self.clock)())?;
            ensure!(
                !self
                    .control(&pairing, &command)
                    .map_err(|_| ObservationPending)?,
                "agent_cancelled"
            );
            let admission: super::evidence::Admission = if resuming {
                serde_json::from_slice(&fs::read(dir.join("admission.json"))?)?
            } else {
                let packet = ingestion::local::acquire(
                    &consent.repository_path,
                    &consent.repository,
                    &consent.revision,
                    consent.scope.clone(),
                )?;
                let saved = super::evidence::Admission::new(
                    packet,
                    super::evidence::Retention {
                        seconds: expires_at.saturating_sub((self.clock)()),
                    },
                    (self.clock)(),
                )?;
                save_private(&dir.join("admission.json"), &saved)?;
                saved
            };
            admission.validate()?;
            ensure!(
                admission.packet.completeness == "complete_scoped_acquisition",
                "review_requires_complete_scoped_acquisition"
            );
            ensure!(
                admission.expires_at == expires_at
                    && admission.packet.revision == consent.revision
                    && admission.packet.repository == consent.repository,
                "agent_resume_admission"
            );
            let packet = admission.packet.clone();
            // Rebuild derived orchestration output only. Gateway acknowledgements and
            // completed outputs remain durable and are never dispatched a second time.
            if resuming && dir.join("work").exists() {
                fs::remove_dir_all(dir.join("work"))?;
            }
            let first_lane = super::review::lanes::ReviewLane::ALL[0];
            let (first_output, first_identity) = self.model_lane(
                &authority,
                &packet,
                first_lane,
                &dir.join("gateway").join(first_lane.id()),
            )?;
            let route = first_identity.route()?;
            let mut first_output = Some(first_output);
            gateway_lanes.push(first_identity.clone());
            super::review::runner::run_with_executor(
                super::review::runner::ExecutionOptions {
                    out: dir.join("work"),
                    run_id: command.run_id.clone(),
                    cancel_file: None,
                },
                admission,
                route,
                |lane, _prompt, _lane_dir| {
                    if lane == first_lane {
                        return Ok(first_output.take().expect("first lane executes once"));
                    }
                    let (output, identity) = self.model_lane(
                        &authority,
                        &packet,
                        lane,
                        &dir.join("gateway").join(lane.id()),
                    )?;
                    ensure!(
                        first_identity.same_execution(&identity),
                        "agent_gateway_identity_changed"
                    );
                    gateway_lanes.push(identity);
                    Ok(output)
                },
            )
        })();
        if run
            .as_ref()
            .err()
            .is_some_and(|error| error.is::<ObservationPending>())
        {
            return Err(ObservationPending.into());
        }
        let still_allowed = authority.check((self.clock)()).is_ok()
            && !self
                .control(&pairing, &command)
                .map_err(|_| ObservationPending)?;
        ensure!(expires_at > (self.clock)(), "agent_retention_expired");
        let mut report = RunReport {
            schema: PROTOCOL.into(),
            agent_id: pairing.agent_id.clone(),
            subject: pairing.subject.clone(),
            run_id: command.run_id.clone(),
            consent_digest: command.consent_digest.clone(),
            execution_location: "local_agent".into(),
            gateway_lanes,
            status: if run.is_ok() && still_allowed {
                "complete"
            } else {
                "failed_or_interrupted"
            }
            .into(),
            expires_at,
            result: if still_allowed { run.ok() } else { None },
            digest: String::new(),
        };
        report.digest = hash(&report)?;
        // Individual lane caps do not bound the aggregate plus admitted evidence.
        // Never persist or announce an unforwardable successful report.
        if serde_json::to_vec(&report)?.len() as u64 > MAX_RESPONSE {
            report.result = None;
            report.status = "failed_or_interrupted".into();
            report.digest.clear();
            report.digest = hash(&report)?;
            scrub_run_payloads(&dir)?;
        }
        ensure!(
            serde_json::to_vec(&report)?.len() as u64 <= MAX_RESPONSE,
            "agent_report_limit"
        );
        save_private(&dir.join("report.json"), &report)?;
        // Never claim remote acceptance until exact result digest is acknowledged.
        self.forward(journal, &pairing, &command, consent_path)?;
        Ok(Some(command.run_id))
    }
    fn forward(
        &self,
        journal: &Journal,
        pairing: &Pairing,
        command: &Command,
        consent_path: &Path,
    ) -> Result<()> {
        self.paired(pairing)?;
        let path = journal.root.join(format!("run-{}", command.run_id));
        let saved: Command = serde_json::from_slice(&fs::read(path.join("command.json"))?)?;
        ensure!(
            hash(&saved)? == hash(command)?,
            "agent_duplicate_command_changed"
        );
        if !path.join("report.json").exists() {
            let expires_at: u64 = serde_json::from_slice(&fs::read(path.join("expires.json"))?)?;
            RunAuthority {
                pairing,
                command,
                consent_path,
                expires_at,
            }
            .check((self.clock)())?;
            // Known reservation with no terminal record is an interrupted run,
            // never evidence authorizing another model dispatch.
            let mut interrupted = RunReport {
                schema: PROTOCOL.into(),
                agent_id: command.agent_id.clone(),
                subject: command.subject.clone(),
                run_id: command.run_id.clone(),
                consent_digest: command.consent_digest.clone(),
                execution_location: "local_agent".into(),
                gateway_lanes: Vec::new(),
                status: "interrupted".into(),
                expires_at,
                result: None,
                digest: String::new(),
            };
            interrupted.digest = hash(&interrupted)?;
            save_private(&path.join("report.json"), &interrupted)?;
        }
        let bytes = fs::read(path.join("report.json"))?;
        ensure!(bytes.len() as u64 <= MAX_RESPONSE, "agent_report_limit");
        let mut report: RunReport = serde_json::from_slice(&bytes)?;
        let expected_digest = std::mem::take(&mut report.digest);
        ensure!(
            hash(&report)? == expected_digest
                && report.schema == PROTOCOL
                && report.agent_id == pairing.agent_id
                && report.subject == pairing.subject
                && report.run_id == command.run_id
                && report.consent_digest == command.consent_digest,
            "agent_report_integrity"
        );
        report.digest = expected_digest;
        ensure!(report.expires_at > (self.clock)(), "agent_report_expired");
        RunAuthority {
            pairing,
            command,
            consent_path,
            expires_at: report.expires_at,
        }
        .check((self.clock)())?;
        ensure!(!self.control(pairing, command)?, "agent_cancelled");
        RunAuthority {
            pairing,
            command,
            consent_path,
            expires_at: report.expires_at,
        }
        .check((self.clock)())?;
        #[derive(Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Ack {
            schema: String,
            run_id: String,
            digest: String,
        }
        let ack: Ack = self.request(
            Method::PUT,
            &format!("/v1/agent/runs/{}/result", command.run_id),
            Some(&pairing.agent_token),
            Some(&serde_json::from_slice(&bytes)?),
        )?;
        ensure!(
            ack.schema == PROTOCOL && ack.run_id == command.run_id && ack.digest == report.digest,
            "agent_result_ack_mismatch"
        );
        Ok(())
    }
    pub fn unpair(&self, journal: &Journal) -> Result<()> {
        let pairing = journal.pairing((self.clock)())?;
        self.paired(&pairing)?;
        let reply: serde_json::Value = self.request(
            Method::POST,
            "/v1/agent/revoke",
            Some(&pairing.agent_token),
            Some(&serde_json::json!({"schema":PROTOCOL,"agent_id":pairing.agent_id})),
        )?;
        ensure!(
            reply["revoked"] == true && reply["agent_id"] == pairing.agent_id,
            "agent_revoke_unconfirmed"
        );
        // Confirmed revocation may be the final invocation. Purge payloads now,
        // preserving command/expiry reservations that prevent a repeated dispatch.
        journal.scrub_payloads()?;
        fs::remove_file(journal.root.join("pairing.json"))?;
        File::open(&journal.root)?.sync_all()?;
        Ok(())
    }
}
pub fn read_consent(path: &Path, now: u64) -> Result<Consent> {
    let m = fs::symlink_metadata(path)?;
    ensure!(
        m.is_file() && m.permissions().mode() & 0o077 == 0 && m.len() <= 65536,
        "agent_consent_file_permissions"
    );
    let mut bytes = Vec::new();
    File::open(path)?.take(65537).read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 65536, "agent_consent_file_limit");
    let consent: Consent = serde_json::from_slice(&bytes)?;
    consent.validate(now)?;
    Ok(consent)
}
struct RunAuthority<'a> {
    pairing: &'a Pairing,
    command: &'a Command,
    consent_path: &'a Path,
    expires_at: u64,
}
impl RunAuthority<'_> {
    fn check(&self, now: u64) -> Result<()> {
        ensure!(self.expires_at > now, "agent_retention_expired");
        self.command
            .validate(self.pairing, &read_consent(self.consent_path, now)?, now)
    }
}
fn scrub_run_payloads(path: &Path) -> Result<()> {
    let report = path.join("report.json");
    if report.exists() {
        fs::remove_file(report)?;
    }
    for name in ["work", "gateway"] {
        let work = path.join(name);
        if work.is_dir() {
            fs::remove_dir_all(work)?;
        }
    }
    let admission = path.join("admission.json");
    if admission.exists() {
        fs::remove_file(admission)?;
    }
    File::open(path)?.sync_all()?;
    Ok(())
}
impl Journal {
    fn scrub_payloads(&self) -> Result<()> {
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            if entry.file_name().to_string_lossy().starts_with("run-")
                && entry.file_type()?.is_dir()
            {
                scrub_run_payloads(&entry.path())?;
            }
        }
        Ok(())
    }
    /// Removes local authority, including expired pairings, without claiming remote
    /// revocation. The user must revoke the old agent from the website separately.
    pub fn forget_pairing(&self) -> Result<()> {
        self.scrub_payloads()?;
        fs::remove_file(self.root.join("pairing.json"))?;
        File::open(&self.root)?.sync_all()?;
        Ok(())
    }
    pub fn expire(&self, now: u64) -> Result<()> {
        for entry in fs::read_dir(&self.root)? {
            let entry = entry?;
            let path = entry.path();
            if !entry.file_name().to_string_lossy().starts_with("run-")
                || !entry.file_type()?.is_dir()
            {
                continue;
            }
            let expiry = path.join("expires.json");
            if !expiry.is_file() {
                continue;
            }
            let deadline: u64 = serde_json::from_slice(&fs::read(expiry)?)?;
            if deadline > now {
                continue;
            }
            scrub_run_payloads(&path)?;
        }
        Ok(())
    }
}
