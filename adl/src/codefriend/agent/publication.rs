//! Paired local publication relay with separate immutable stage receipts.
//! Native decisions and rendering retain independent no-replay reservations.
use super::*;
use crate::codefriend::{
    evidence::{
        contracts::{Publication, ReviewRecord},
        valid_digest,
    },
    integration::{
        prepare_publication_bundle_for_format_v2, PublicationChallenge, PublicationFormat,
    },
    publication::{self as native, DecisionKind, DecisionRecord},
};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::Value;
use sha2::{Digest, Sha256};
const BINDING_SCHEMA: &str = "codefriend.agent_publication.v1";
const STAGE_SCHEMA: &str = "codefriend.agent_publication_stage.v1";
const LIMIT: usize = 4 * 1024 * 1024;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub schema: String,
    pub job_id: String,
    pub subject: String,
    pub agent_id: String,
    pub run_id: String,
    pub report_digest: String,
    pub received_digest: String,
    pub consent_digest: String,
    pub format: PublicationFormat,
    pub expires_at: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Decision {
    pub challenge_digest: String,
    pub binding_digest: String,
    pub expected_decision_digest: Option<String>,
    pub decision: DecisionKind,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub binding: Binding,
    pub status: String,
    pub decision: Option<Decision>,
    pub prepared_digest: Option<String>,
    pub terminal_digest: Option<String>,
    pub effect_unresolved: bool,
    pub agent_candidate_revision: Option<String>,
    pub prepared: Option<Prepared>,
    pub terminal: Option<TerminalSummary>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalSummary {
    pub status: String,
    pub decision_digest: Option<String>,
    pub reason: String,
    pub exports: Vec<ExportSummary>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExportSummary {
    pub index: usize,
    pub name: String,
    pub media_type: String,
    pub digest: String,
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Poll {
    job: Option<Job>,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stage {
    pub schema: String,
    pub stage: String,
    pub binding: Binding,
    pub agent_candidate_revision: String,
    pub payload: Value,
    pub exports: Vec<Export>,
    pub digest: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Export {
    pub name: String,
    pub media_type: String,
    pub digest: String,
    pub bytes_base64: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Prepared {
    pub challenge_digest: String,
    pub binding_digest: String,
    pub expected_decision_digest: Option<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub native: PreparedNative,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PreparedNative {
    pub schema: String,
    pub challenge: PublicationChallenge,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Terminal {
    pub status: String,
    pub decision_digest: Option<String>,
    pub reason: String,
    pub native: TerminalNative,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TerminalNative {
    pub schema: String,
    pub decision: Option<DecisionRecord>,
    pub render: Option<Value>,
    pub manifest: Option<Value>,
}
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct VerificationContext {
    pub schema: String,
    pub binding: Binding,
    pub report: RunReport,
    pub decision: Option<Decision>,
    pub prepared: Option<Stage>,
    pub now: u64,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct LocalProvenance {
    pub schema: String,
    pub subject: String,
    pub agent_id: String,
    pub run_id: String,
    pub job_digest: String,
    pub report_digest: String,
    pub received_digest: String,
    pub consent_digest: String,
    pub format: PublicationFormat,
    pub agent_candidate_revision: String,
    pub challenge_digest: String,
    pub publication_binding_digest: String,
}
fn private_dirs(path: &Path) -> Result<()> {
    use std::os::unix::fs::DirBuilderExt;
    native::manifest::reject_symlink_components(path)?;
    fs::DirBuilder::new()
        .recursive(true)
        .mode(0o700)
        .create(path)?;
    ensure!(
        fs::metadata(path)?.permissions().mode() & 0o077 == 0,
        "publication_private_directory"
    );
    Ok(())
}
pub(super) fn read<T: serde::de::DeserializeOwned>(path: &Path, limit: usize) -> Result<T> {
    use std::io::Read;
    native::manifest::reject_symlink_components(path)?;
    let meta = fs::symlink_metadata(path)?;
    ensure!(
        meta.is_file() && meta.permissions().mode() & 0o077 == 0 && meta.len() <= limit as u64,
        "publication_private_file"
    );
    let mut bytes = Vec::new();
    File::open(path)?
        .take(limit as u64 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= limit, "publication_file_limit");
    Ok(serde_json::from_slice(&bytes)?)
}
fn canonical(value: &Value) -> Result<Value> {
    Ok(match value {
        Value::Object(map) => {
            // Wire records use fixed ASCII non-index keys only. This excludes JS
            // integer-key enumeration and UTF16-vs-UTF8 ordering ambiguity.
            let mut out = serde_json::Map::new();
            let mut keys = map.keys().collect::<Vec<_>>();
            keys.sort();
            for key in keys {
                ensure!(
                    key.is_ascii() && key.parse::<u32>().is_err(),
                    "publication_noncanonical_key"
                );
                out.insert(key.clone(), canonical(&map[key])?);
            }
            Value::Object(out)
        }
        Value::Array(xs) => Value::Array(xs.iter().map(canonical).collect::<Result<_>>()?),
        Value::Number(n) => {
            ensure!(
                n.as_u64().is_some_and(|x| x <= 9_007_199_254_740_991),
                "publication_noncanonical_number"
            );
            value.clone()
        }
        _ => value.clone(),
    })
}
fn stage_digest(stage: &Stage) -> Result<String> {
    let mut value = serde_json::to_value(stage)?;
    value["digest"] = Value::String(String::new());
    Ok(format!(
        "{:x}",
        Sha256::digest(serde_json::to_vec(&canonical(&value)?)?)
    ))
}
fn sealed_stage(
    binding: &Binding,
    stage: &str,
    payload: impl Serialize,
    exports: Vec<Export>,
) -> Result<Stage> {
    let mut value = Stage {
        schema: STAGE_SCHEMA.into(),
        stage: stage.into(),
        binding: binding.clone(),
        agent_candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
        payload: serde_json::to_value(payload)?,
        exports,
        digest: String::new(),
    };
    value.digest = stage_digest(&value)?;
    ensure!(
        serde_json::to_vec(&value)?.len() <= LIMIT,
        "publication_output_limit"
    );
    Ok(value)
}
impl Binding {
    fn validate_report(&self, report: &RunReport, now: u64) -> Result<()> {
        report.validate(now)?;
        ensure!(
            self.schema == BINDING_SCHEMA
                && identifier(&self.job_id)
                && valid_digest(&self.received_digest),
            "publication_binding"
        );
        ensure!(
            self.subject == report.subject
                && self.agent_id == report.agent_id
                && self.run_id == report.run_id
                && self.report_digest == report.digest
                && self.consent_digest == report.consent_digest
                && self.expires_at == report.expires_at
                && report.status == "complete"
                && report.selected_review().is_ok(),
            "publication_report_binding"
        );
        Ok(())
    }
}
struct LocalContext {
    pairing: Pairing,
    command: Command,
    report: RunReport,
    root: PathBuf,
}
impl Transport {
    fn publication_context(
        &self,
        journal: &Journal,
        consent_path: &Path,
        binding: &Binding,
    ) -> Result<LocalContext> {
        let now = (self.clock)();
        let pairing = journal.pairing(now)?;
        self.paired(&pairing)?;
        ensure!(identifier(&binding.run_id), "publication_run_id");
        let root = journal.root.join(format!("run-{}", binding.run_id));
        let command: Command = read(&root.join("command.json"), 8192)?;
        command.validate(&pairing, &read_consent(consent_path, now)?, now)?;
        let report: RunReport = read(&root.join("report.json"), LIMIT)?;
        binding.validate_report(&report, now)?;
        let expiry: u64 = read(&root.join("expires.json"), 64)?;
        ensure!(
            expiry == binding.expires_at
                && command.run_id == binding.run_id
                && command.consent_digest == binding.consent_digest
                && pairing.agent_id == binding.agent_id
                && pairing.subject == binding.subject,
            "publication_local_identity"
        );
        let receipt = self.publication_receipt(journal, &binding.run_id, consent_path)?;
        ensure!(
            receipt.received_digest == binding.received_digest,
            "publication_received_digest"
        );
        // Recheck after authenticated receipt observation, not just before it.
        let after = (self.clock)();
        let current = journal.pairing(after)?;
        ensure!(
            hash(&current)? == hash(&pairing)?,
            "publication_pairing_changed"
        );
        command.validate(&current, &read_consent(consent_path, after)?, after)?;
        binding.validate_report(&report, after)?;
        report.check_original_cycle(&root, after)?;
        Ok(LocalContext {
            pairing,
            command,
            report,
            root,
        })
    }
    fn recheck_publication_context(
        &self,
        journal: &Journal,
        consent_path: &Path,
        binding: &Binding,
        context: &LocalContext,
    ) -> Result<()> {
        let now = (self.clock)();
        let pairing = journal.pairing(now)?;
        ensure!(
            hash(&pairing)? == hash(&context.pairing)?,
            "publication_pairing_changed"
        );
        context
            .command
            .validate(&pairing, &read_consent(consent_path, now)?, now)?;
        binding.validate_report(&context.report, now)?;
        let expiry: u64 = read(&context.root.join("expires.json"), 64)?;
        ensure!(
            expiry == binding.expires_at && expiry > now,
            "publication_retention_changed"
        );
        Ok(())
    }
    fn job(&self, pairing: &Pairing, binding: &Binding) -> Result<Job> {
        let job: Job = self.request(
            Method::GET,
            &format!("/v1/agent/publications/{}", binding.job_id),
            Some(&pairing.agent_token),
            None,
        )?;
        ensure!(job.binding == *binding, "publication_job_changed");
        Ok(job)
    }
    /// Called before the review poll; Some means this invocation cannot reach a model.
    pub(super) fn poll_publication(
        &self,
        journal: &Journal,
        consent_path: &Path,
    ) -> Result<Option<String>> {
        let pairing = journal.pairing((self.clock)())?;
        self.paired(&pairing)?;
        let poll: Poll = self.request(
            Method::GET,
            "/v1/agent/publications",
            Some(&pairing.agent_token),
            None,
        )?;
        let Some(job) = poll.job else { return Ok(None) };
        ensure!(
            env!("CODEFRIEND_BUILD_CLEAN") == "true",
            "publication_dirty_candidate"
        );
        let context = self.publication_context(journal, consent_path, &job.binding)?;
        let root = context
            .root
            .join("work/publications")
            .join(&job.binding.job_id);
        native::manifest::reject_symlink_components(&root)?;
        for stage in ["terminal", "prepared"] {
            let path = root.join(format!("{stage}-stage.json"));
            if path.exists() {
                let saved: Stage = read(&path, LIMIT)?;
                // A prepared stage alone must not prevent a pending decision.
                if stage == "terminal" || job.status == "awaiting_agent" {
                    self.forward_stage(journal, consent_path, &saved)?;
                    return Ok(Some(job.binding.run_id));
                }
            }
        }
        match job.status.as_str() {
            "awaiting_agent" => self.prepare_job(journal, consent_path, &job, &root)?,
            "decision_pending" => self.decide_job(journal, consent_path, &job, &root)?,
            _ => anyhow::bail!("publication_job_not_actionable"),
        }
        Ok(Some(job.binding.run_id))
    }
    fn prepare_job(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
        root: &Path,
    ) -> Result<()> {
        let context = self.publication_context(journal, consent_path, &job.binding)?;
        let current = self.job(&context.pairing, &job.binding)?;
        ensure!(
            current.status == "awaiting_agent" && current.decision.is_none(),
            "publication_not_preparable"
        );
        let reservation = context
            .root
            .join(format!("publication-{}.json", job.binding.job_id));
        if reservation.exists() {
            let saved: String = read(&reservation, 256)?;
            ensure!(
                saved == hash(&job.binding)?,
                "publication_reservation_changed"
            );
            return self.fail_job(journal, consent_path, job, root, "effect_unresolved", None);
        }
        let count = fs::read_dir(&context.root)?
            .filter_map(|x| x.ok())
            .filter(|x| x.file_name().to_string_lossy().starts_with("publication-"))
            .count();
        ensure!(count < 12, "publication_capacity");
        save_private(&reservation, &hash(&job.binding)?)?;
        private_dirs(root)?;
        let destination = root.join("exports");
        private_dirs(&destination)?;
        let review = &context.report.selected_review()?.review_record;
        save_private(&root.join("review-record.json"), review)?;
        let bound = prepare_publication_bundle_for_format_v2(
            &root.join("review-record.json"),
            &root.join("bundle"),
            &destination,
            job.binding.format,
        )?;
        let context = self.publication_context(journal, consent_path, &job.binding)?;
        let current = self.job(&context.pairing, &job.binding)?;
        ensure!(
            current.status == "awaiting_agent" && current.decision.is_none(),
            "publication_job_changed"
        );
        let now = (self.clock)();
        let challenge = PublicationChallenge::prepare(
            &job.binding.subject,
            &job.binding.job_id,
            env!("CODEFRIEND_BUILD_REVISION"),
            review,
            &bound,
            &root.join("bundle/artifacts"),
            None,
            now,
            job.binding.expires_at.min(now.saturating_add(300)),
        )?;
        let p = Prepared {
            challenge_digest: challenge.digest().into(),
            binding_digest: bound.binding_digest()?,
            expected_decision_digest: None,
            issued_at: now,
            expires_at: job.binding.expires_at.min(now.saturating_add(300)),
            native: PreparedNative {
                schema: "codefriend.agent_publication_prepared.v1".into(),
                challenge,
            },
        };
        let stage = sealed_stage(&job.binding, "prepared", p, vec![])?;
        save_private(&root.join("prepared-stage.json"), &stage)?;
        self.forward_stage(journal, consent_path, &stage)
    }
    fn forward_stage(&self, journal: &Journal, consent_path: &Path, stage: &Stage) -> Result<()> {
        ensure!(
            stage.digest == stage_digest(stage)? && serde_json::to_vec(stage)?.len() <= LIMIT,
            "publication_stage_corrupt"
        );
        let context = self.publication_context(journal, consent_path, &stage.binding)?;
        let observed = self.job(&context.pairing, &stage.binding)?;
        self.recheck_publication_context(journal, consent_path, &stage.binding, &context)?;
        let head = if stage.stage == "prepared" {
            &observed.prepared_digest
        } else {
            &observed.terminal_digest
        };
        if let Some(head) = head {
            ensure!(head == &stage.digest, "publication_stage_conflict");
            return Ok(());
        }
        // Authenticated absence permits only this exact saved upload, no native effects.
        let _: Job = self.request(
            Method::POST,
            &format!(
                "/v1/agent/publications/{}/{}",
                stage.binding.job_id, stage.stage
            ),
            Some(&context.pairing.agent_token),
            Some(&serde_json::to_value(stage)?),
        )?;
        let observed = self.job(&context.pairing, &stage.binding)?;
        self.recheck_publication_context(journal, consent_path, &stage.binding, &context)?;
        let head = if stage.stage == "prepared" {
            observed.prepared_digest
        } else {
            observed.terminal_digest
        };
        ensure!(
            head.as_deref() == Some(stage.digest.as_str()),
            "publication_upload_unconfirmed"
        );
        Ok(())
    }
    fn fail_job(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
        root: &Path,
        reason: &str,
        decision: Option<DecisionRecord>,
    ) -> Result<()> {
        private_dirs(root)?;
        let terminal = Terminal {
            status: "failed".into(),
            decision_digest: decision.as_ref().map(|x| x.digest.clone()),
            reason: reason.into(),
            native: TerminalNative {
                schema: "codefriend.agent_publication_terminal.v1".into(),
                decision,
                render: None,
                manifest: None,
            },
        };
        let stage = sealed_stage(&job.binding, "terminal", terminal, vec![])?;
        save_private(&root.join("terminal-stage.json"), &stage)?;
        self.forward_stage(journal, consent_path, &stage)
    }
    fn decide_job(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
        root: &Path,
    ) -> Result<()> {
        let context = self.publication_context(journal, consent_path, &job.binding)?;
        let prepared_stage: Stage = read(&root.join("prepared-stage.json"), LIMIT)?;
        ensure!(
            Some(&prepared_stage.digest) == job.prepared_digest.as_ref()
                && prepared_stage.digest == stage_digest(&prepared_stage)?,
            "publication_prepared_changed"
        );
        let prepared: Prepared = serde_json::from_value(prepared_stage.payload.clone())?;
        let requested = job
            .decision
            .as_ref()
            .ok_or_else(|| anyhow::anyhow!("publication_decision_missing"))?;
        let review = &context.report.selected_review()?.review_record;
        let bound = native::read_publication(&root.join("bundle/publication.json"))?;
        let authority = LocalDecisionAuthority {
            transport: self,
            journal,
            consent_path,
            binding: &job.binding,
            requested,
            challenge: &prepared.native.challenge,
            pairing_digest: hash(&context.pairing)?,
            root,
        };
        let decision_reservation = root.join("decision-reserved.json");
        let decision = if decision_reservation.exists() {
            let expected: String = read(&decision_reservation, 256)?;
            ensure!(
                expected == hash(requested)?,
                "publication_decision_reservation_changed"
            );
            // A missing or incomplete journal cannot prove append non-effect.
            let Some(record) = native::read_decision_head(&root.join("approvals"), review, &bound)?
            else {
                return self.fail_job(journal, consent_path, job, root, "effect_unresolved", None);
            };
            let (provenance, _) = authority.verify_existing(review, &bound, &record)?;
            ensure!(
                record.local_agent.as_ref() == Some(&provenance)
                    && record.decision == requested.decision,
                "publication_decision_recovery_mismatch"
            );
            record
        } else {
            save_private(&decision_reservation, &hash(requested)?)?;
            native::approval::append_authenticated_local_agent_decision(
                &root.join("approvals"),
                review,
                &bound,
                requested.decision.clone(),
                &authority,
            )?
        };
        if decision.decision == DecisionKind::Withheld {
            let stage = sealed_stage(
                &job.binding,
                "terminal",
                Terminal {
                    status: "withheld".into(),
                    decision_digest: Some(decision.digest.clone()),
                    reason: "withheld".into(),
                    native: TerminalNative {
                        schema: "codefriend.agent_publication_terminal.v1".into(),
                        decision: Some(decision),
                        render: None,
                        manifest: None,
                    },
                },
                vec![],
            )?;
            save_private(&root.join("terminal-stage.json"), &stage)?;
            return self.forward_stage(journal, consent_path, &stage);
        }
        let reservation = root.join("render-reserved.json");
        if reservation.exists() {
            // Never retry rendering. Companion verifier admits only exact committed output.
            return self.reconcile_render(journal, consent_path, job, root, &decision);
        }
        authority.verify_existing(review, &bound, &decision)?;
        save_private(&reservation, &decision.digest)?;
        match render(root, job.binding.format) {
            Ok(result) => {
                save_private(&root.join("render-result.json"), &result)?;
                self.reconcile_render(journal, consent_path, job, root, &decision)
            }
            Err(_) => self.fail_job(
                journal,
                consent_path,
                job,
                root,
                "render_failed",
                Some(decision),
            ),
        }
    }
    fn reconcile_render(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
        root: &Path,
        decision: &DecisionRecord,
    ) -> Result<()> {
        let context = self.publication_context(journal, consent_path, &job.binding)?;
        let current = self.job(&context.pairing, &job.binding)?;
        ensure!(
            current.status == "decision_pending" && current.decision == job.decision,
            "publication_authority_changed"
        );
        let prepared: Stage = read(&root.join("prepared-stage.json"), LIMIT)?;
        let (target, name, media) = names(job.binding.format);
        let directory = root.join("exports").join(target);
        let manifest: Value = match read(&directory.join("manifest.json"), LIMIT) {
            Ok(value) => value,
            Err(_) => {
                return self.fail_job(
                    journal,
                    consent_path,
                    job,
                    root,
                    "effect_unresolved",
                    Some(decision.clone()),
                )
            }
        };
        // Committed manifest may exist even if the process died before saving
        // render-result. Recover metadata, then verify all bindings and bytes below.
        let result: Value = if root.join("render-result.json").exists() {
            read(&root.join("render-result.json"), LIMIT)?
        } else {
            native::relay::result_from_manifest(job.binding.format, &manifest)?
        };
        native::manifest::reject_symlink_components(&directory.join(name))?;
        let file = File::open(directory.join(name))?;
        use std::io::Read;
        let mut bytes = Vec::new();
        file.take(LIMIT as u64 + 1).read_to_end(&mut bytes)?;
        if bytes.len() > LIMIT {
            return self.fail_job(
                journal,
                consent_path,
                job,
                root,
                "output_limit",
                Some(decision.clone()),
            );
        }
        let terminal = Terminal {
            status: "complete".into(),
            decision_digest: Some(decision.digest.clone()),
            reason: "rendered".into(),
            native: TerminalNative {
                schema: "codefriend.agent_publication_terminal.v1".into(),
                decision: Some(decision.clone()),
                render: Some(result),
                manifest: Some(manifest),
            },
        };
        let overhead = sealed_stage(
            &job.binding,
            "terminal",
            &terminal,
            vec![Export {
                name: name.into(),
                media_type: media.into(),
                digest: "0".repeat(64),
                bytes_base64: String::new(),
            }],
        )?;
        let encoded_len = bytes
            .len()
            .checked_add(2)
            .and_then(|n| n.checked_div(3))
            .and_then(|n| n.checked_mul(4))
            .ok_or_else(|| anyhow::anyhow!("publication_output_limit"))?;
        if serde_json::to_vec(&overhead)?
            .len()
            .checked_add(encoded_len)
            .is_none_or(|n| n > LIMIT)
        {
            return self.fail_job(
                journal,
                consent_path,
                job,
                root,
                "output_limit",
                Some(decision.clone()),
            );
        }
        let stage = match sealed_stage(
            &job.binding,
            "terminal",
            terminal,
            vec![Export {
                name: name.into(),
                media_type: media.into(),
                digest: crate::codefriend::ingestion::digest(&bytes),
                bytes_base64: STANDARD.encode(&bytes),
            }],
        ) {
            Ok(stage) => stage,
            Err(_) => {
                return self.fail_job(
                    journal,
                    consent_path,
                    job,
                    root,
                    "output_limit",
                    Some(decision.clone()),
                )
            }
        };
        verify_stage(
            &stage,
            &VerificationContext {
                schema: "codefriend.agent_publication_verifier_context.v1".into(),
                binding: job.binding.clone(),
                report: context.report,
                decision: job.decision.clone(),
                prepared: Some(prepared),
                now: (self.clock)(),
            },
            (self.clock)(),
        )?;
        save_private(&root.join("terminal-stage.json"), &stage)?;
        self.forward_stage(journal, consent_path, &stage)
    }
}

pub(crate) struct LocalDecisionAuthority<'a> {
    transport: &'a Transport,
    journal: &'a Journal,
    consent_path: &'a Path,
    binding: &'a Binding,
    requested: &'a Decision,
    challenge: &'a PublicationChallenge,
    pairing_digest: String,
    root: &'a Path,
}
impl LocalDecisionAuthority<'_> {
    pub(crate) fn requested_decision(&self) -> &DecisionKind {
        &self.requested.decision
    }
    pub(crate) fn verify(
        &self,
        review: &ReviewRecord,
        publication: &Publication,
        head: Option<&str>,
    ) -> Result<(LocalProvenance, u64)> {
        self.verify_inner(review, publication, head, None)
    }
    fn verify_existing(
        &self,
        review: &ReviewRecord,
        publication: &Publication,
        record: &DecisionRecord,
    ) -> Result<(LocalProvenance, u64)> {
        record.validate(review)?;
        let verified = self.verify_inner(
            review,
            publication,
            record.previous_decision_digest.as_deref(),
            Some(record.decided_at),
        )?;
        ensure!(
            record.schema == "codefriend.publication_decision.v3"
                && record.local_agent.as_ref() == Some(&verified.0)
                && record.decision == self.requested.decision,
            "publication_historic_decision"
        );
        Ok(verified)
    }
    fn verify_inner(
        &self,
        review: &ReviewRecord,
        publication: &Publication,
        head: Option<&str>,
        historic: Option<u64>,
    ) -> Result<(LocalProvenance, u64)> {
        let context =
            self.transport
                .publication_context(self.journal, self.consent_path, self.binding)?;
        ensure!(
            hash(&context.pairing)? == self.pairing_digest
                && context.report.selected_review()?.review_record == *review,
            "publication_authority_identity"
        );
        let job = self.transport.job(&context.pairing, self.binding)?;
        ensure!(
            job.status == "decision_pending"
                && job.decision.as_ref() == Some(self.requested)
                && matches!(
                    self.requested.decision,
                    DecisionKind::Approved | DecisionKind::Withheld
                ),
            "publication_authority_decision"
        );
        let now = (self.transport.clock)();
        let pairing = self.journal.pairing(now)?;
        ensure!(
            hash(&pairing)? == self.pairing_digest,
            "publication_pairing_changed"
        );
        context
            .command
            .validate(&pairing, &read_consent(self.consent_path, now)?, now)?;
        self.binding.validate_report(&context.report, now)?;
        self.challenge.verify_response(
            &self.binding.subject,
            &self.binding.job_id,
            env!("CODEFRIEND_BUILD_REVISION"),
            &self.requested.challenge_digest,
            &self.requested.binding_digest,
            head,
            review,
            publication,
            &self.root.join("bundle/artifacts"),
            historic.unwrap_or(now),
        )?;
        ensure!(
            head == self.requested.expected_decision_digest.as_deref(),
            "publication_head_changed"
        );
        Ok((
            LocalProvenance {
                schema: "codefriend.local_agent_decision.v1".into(),
                subject: self.binding.subject.clone(),
                agent_id: self.binding.agent_id.clone(),
                run_id: self.binding.run_id.clone(),
                job_digest: hash(self.binding)?,
                report_digest: self.binding.report_digest.clone(),
                received_digest: self.binding.received_digest.clone(),
                consent_digest: self.binding.consent_digest.clone(),
                format: self.binding.format,
                agent_candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
                challenge_digest: self.requested.challenge_digest.clone(),
                publication_binding_digest: self.requested.binding_digest.clone(),
            },
            now,
        ))
    }
}
fn names(format: PublicationFormat) -> (&'static str, &'static str, &'static str) {
    match format {
        PublicationFormat::Markdown => ("report-md", "report.md", "text/markdown"),
        PublicationFormat::Html => ("report-html", "report.html", "text/html"),
        PublicationFormat::Pdf => ("report-pdf", "report.pdf", "application/pdf"),
    }
}
fn render(root: &Path, format: PublicationFormat) -> Result<Value> {
    let review_record = root.join("review-record.json");
    let publication = root.join("bundle/publication.json");
    let approval_store = root.join("approvals");
    let artifact_root = root.join("bundle/artifacts");
    let synthesis = "synthesis/synthesis.json".into();
    let remediation_plan = "remediation/remediation-plan.json".into();
    let test_plan = "tests/test-plan.json".into();
    let destination_root = root.join("exports");
    let out = destination_root.join(names(format).0);
    Ok(match format {
        PublicationFormat::Markdown => {
            serde_json::to_value(native::render_markdown(native::MarkdownRenderOptions {
                review_record,
                publication,
                approval_store,
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root,
                out,
            })?)?
        }
        PublicationFormat::Html => {
            serde_json::to_value(native::render_html(native::HtmlRenderOptions {
                review_record,
                publication,
                approval_store,
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root,
                out,
            })?)?
        }
        PublicationFormat::Pdf => {
            let font = [
                "/usr/share/fonts/truetype/dejavu/DejaVuSans.ttf",
                "/System/Library/Fonts/Supplemental/Arial.ttf",
            ]
            .into_iter()
            .map(PathBuf::from)
            .find(|p| p.is_file())
            .ok_or_else(|| anyhow::anyhow!("publication_pdf_font_required"))?;
            serde_json::to_value(native::render_pdf(native::PdfRenderOptions {
                review_record,
                publication,
                approval_store,
                artifact_root,
                synthesis,
                remediation_plan,
                test_plan,
                destination_root,
                out,
                font,
            })?)?
        }
    })
}
/// No network or dispatch. Context is supplied independently from owned website state.
pub fn verify_stage(stage: &Stage, context: &VerificationContext, now: u64) -> Result<()> {
    ensure!(
        context.schema == "codefriend.agent_publication_verifier_context.v1"
            && context.now <= now
            && context.binding == stage.binding,
        "publication_verifier_context"
    );
    stage.binding.validate_report(&context.report, now)?;
    ensure!(
        stage.schema == STAGE_SCHEMA
            && stage.digest == stage_digest(stage)?
            && serde_json::to_vec(stage)?.len() <= LIMIT,
        "publication_stage_integrity"
    );
    ensure!(
        stage.agent_candidate_revision.len() == 40
            && stage.agent_candidate_revision != "0".repeat(40)
            && stage
                .agent_candidate_revision
                .bytes()
                .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)),
        "publication_candidate"
    );
    let review = &context.report.selected_review()?.review_record;
    match stage.stage.as_str() {
        "prepared" => {
            let prepared: Prepared = serde_json::from_value(stage.payload.clone())?;
            ensure!(
                stage.exports.is_empty()
                    && prepared.native.schema == "codefriend.agent_publication_prepared.v1",
                "publication_prepared_shape"
            );
            // Companion verifier rebuilds deterministic source-derived bundle inputs
            // without granting approval or executing any renderer.
            native::relay::verify_prepared(
                &prepared,
                &stage.binding,
                &stage.agent_candidate_revision,
                review,
                now,
            )?;
        }
        "terminal" => {
            let terminal: Terminal = serde_json::from_value(stage.payload.clone())?;
            ensure!(
                terminal.native.schema == "codefriend.agent_publication_terminal.v1",
                "publication_terminal_shape"
            );
            if terminal.status == "failed" {
                ensure!(
                    stage.exports.is_empty()
                        && terminal.native.render.is_none()
                        && terminal.native.manifest.is_none()
                        && ["render_failed", "effect_unresolved", "output_limit"]
                            .contains(&terminal.reason.as_str()),
                    "publication_failed_shape"
                );
                if terminal.native.decision.is_none() {
                    ensure!(
                        terminal.decision_digest.is_none(),
                        "publication_failure_decision"
                    );
                    return Ok(());
                }
            }
            let accepted = context
                .prepared
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("publication_prepared_required"))?;
            ensure!(
                accepted.binding == stage.binding
                    && accepted.agent_candidate_revision == stage.agent_candidate_revision
                    && accepted.stage == "prepared"
                    && accepted.digest == stage_digest(accepted)?,
                "publication_prepared_binding"
            );
            let prepared: Prepared = serde_json::from_value(accepted.payload.clone())?;
            // Validate historic challenge at issue time; decision timestamp below must
            // be inside it. Delivery itself is bounded by current run retention.
            native::relay::verify_prepared(
                &prepared,
                &stage.binding,
                &stage.agent_candidate_revision,
                review,
                prepared.issued_at,
            )?;
            let requested = context
                .decision
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("publication_decision_required"))?;
            let decision = terminal
                .native
                .decision
                .as_ref()
                .ok_or_else(|| anyhow::anyhow!("publication_native_decision_required"))?;
            native::relay::verify_local_decision(
                decision,
                requested,
                &prepared,
                &stage.binding,
                &stage.agent_candidate_revision,
                review,
            )?;
            ensure!(
                terminal.decision_digest.as_deref() == Some(decision.digest.as_str()),
                "publication_decision_digest"
            );
            match terminal.status.as_str() {
                "complete" => {
                    ensure!(
                        decision.decision == DecisionKind::Approved
                            && terminal.reason == "rendered"
                            && stage.exports.len() == 1,
                        "publication_complete_shape"
                    );
                    let file = &stage.exports[0];
                    let (_, name, media) = names(stage.binding.format);
                    let bytes = STANDARD.decode(&file.bytes_base64)?;
                    ensure!(
                        !bytes.is_empty()
                            && STANDARD.encode(&bytes) == file.bytes_base64
                            && file.name == name
                            && file.media_type == media
                            && crate::codefriend::ingestion::digest(&bytes) == file.digest,
                        "publication_export_integrity"
                    );
                    native::relay::verify_rendered(
                        review,
                        decision,
                        stage.binding.format,
                        terminal
                            .native
                            .render
                            .as_ref()
                            .ok_or_else(|| anyhow::anyhow!("publication_render_missing"))?,
                        terminal
                            .native
                            .manifest
                            .as_ref()
                            .ok_or_else(|| anyhow::anyhow!("publication_manifest_missing"))?,
                        &bytes,
                    )?;
                }
                "withheld" => ensure!(
                    decision.decision == DecisionKind::Withheld
                        && terminal.reason == "withheld"
                        && stage.exports.is_empty()
                        && terminal.native.render.is_none()
                        && terminal.native.manifest.is_none(),
                    "publication_withheld_shape"
                ),
                "failed" => {}
                _ => anyhow::bail!("publication_terminal_status"),
            }
        }
        _ => anyhow::bail!("publication_stage_name"),
    }
    Ok(())
}
