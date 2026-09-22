//! Paired transport for native Journey owners. No model or renderer dispatch.
use super::*;
use crate::codefriend::{
    architecture::artifact::{
        BoundaryPolicyArtifact, ChangeSetArtifact, ImpactArtifact, RationaleArtifact,
        RationaleSelectionArtifact, StructureArtifact,
    },
    evidence::valid_digest,
    governance::artifact::{FitnessArtifact, PolicyArtifact},
    integration::PublicationFormat,
};
use crate::codefriend::{evidence::store::Store, integration::journey as native};
use sha2::{Digest, Sha256};
mod delivery;
pub mod verification;

#[cfg(all(test, unix))]
#[path = "../../../tests/support/codefriend_agent_journey_tests.rs"]
mod tests;

const JOB_SCHEMA: &str = "codefriend.agent_journey_job.v1";
const JOB_SCHEMA_V2: &str = "codefriend.agent_journey_job.v2";
const MAX_REQUEST: usize = 128 * 1024;
const MAX_JOBS_PER_RUN: usize = 32;

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Binding {
    pub schema: String,
    pub job_id: String,
    pub subject: String,
    pub agent_id: String,
    pub run_id: String,
    pub consent_digest: String,
    pub report_digest: String,
    pub received_digest: String,
    pub request_digest: String,
    pub expires_at: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "stage", rename_all = "snake_case", deny_unknown_fields)]
// One bounded request is processed at a time; preserve the owned wire-contract API.
#[allow(clippy::large_enum_variant)]
pub enum Request {
    Prepare {
        boundary_policy: BoundaryPolicyArtifact,
        fitness_policy: PolicyArtifact,
    },
    Status,
    Graph,
    Artifact {
        artifact: Artifact,
    },
    Impact {
        changes: ChangeSetArtifact,
    },
    Rationale {
        selection: RationaleSelectionArtifact,
    },
    Drift {
        baseline_run: String,
    },
    PalaceComparison {
        baseline_run: String,
    },
    AttachPublication {
        publication_job: String,
        format: PublicationFormat,
    },
}

impl Request {
    fn matches_schema(&self, schema: &str) -> bool {
        let v2 = schema == JOB_SCHEMA_V2;
        match self {
            Self::Prepare {
                boundary_policy,
                fitness_policy,
            } => {
                matches!(boundary_policy, BoundaryPolicyArtifact::V2(_)) == v2
                    && matches!(fitness_policy, PolicyArtifact::V2(_)) == v2
            }
            Self::Impact { changes } => matches!(changes, ChangeSetArtifact::V2(_)) == v2,
            Self::Rationale { selection } => {
                matches!(selection, RationaleSelectionArtifact::V2(_)) == v2
            }
            _ => true,
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Artifact {
    Structure,
    Fitness,
    Impact,
    Rationale,
    Drift,
    PalaceComparison,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub binding: Binding,
    pub request: Request,
    // Website policy is the accepted installed agent candidate, never gateway
    // model provenance or a first-response claim from an arbitrary agent.
    pub permitted_agent_candidate: String,
}

impl Request {
    pub fn digest(&self) -> Result<String> {
        let value = serde_json::to_value(self)?;
        let bytes = canonical_request_bytes(&value)?;
        ensure!(bytes.len() <= MAX_REQUEST, "agent_journey_request_limit");
        Ok(format!("{:x}", Sha256::digest(bytes)))
    }
    fn baseline(&self) -> Option<&str> {
        match self {
            Self::Drift { baseline_run } | Self::PalaceComparison { baseline_run } => {
                Some(baseline_run)
            }
            _ => None,
        }
    }
    fn is_observation(&self) -> bool {
        matches!(self, Self::Status | Self::Graph | Self::Artifact { .. })
    }
}

// Request policies contain source-derived Unicode path keys. Match JavaScript
// string ordering explicitly; serde_json::Map ordering is not the wire contract.
fn canonical_request_bytes(value: &serde_json::Value) -> Result<Vec<u8>> {
    fn append(value: &serde_json::Value, out: &mut Vec<u8>) -> Result<()> {
        match value {
            serde_json::Value::Object(map) => {
                let mut keys = map.keys().collect::<Vec<_>>();
                keys.sort_by(|a, b| a.encode_utf16().cmp(b.encode_utf16()));
                out.push(b'{');
                for (index, key) in keys.into_iter().enumerate() {
                    if index != 0 {
                        out.push(b',');
                    }
                    out.extend(serde_json::to_vec(key)?);
                    out.push(b':');
                    append(&map[key], out)?;
                }
                out.push(b'}');
            }
            serde_json::Value::Array(values) => {
                out.push(b'[');
                for (index, value) in values.iter().enumerate() {
                    if index != 0 {
                        out.push(b',');
                    }
                    append(value, out)?;
                }
                out.push(b']');
            }
            serde_json::Value::Number(number) => {
                ensure!(
                    number.as_u64().is_some_and(|n| n <= 9_007_199_254_740_991),
                    "agent_journey_request_number"
                );
                out.extend(serde_json::to_vec(value)?);
            }
            _ => out.extend(serde_json::to_vec(value)?),
        }
        ensure!(out.len() <= MAX_REQUEST, "agent_journey_request_limit");
        Ok(())
    }
    let mut bytes = Vec::new();
    append(value, &mut bytes)?;
    Ok(bytes)
}

impl Job {
    // Local authority must separately recheck current pairing/consent/control
    // before and after native effects and forwarding. This is exact binding only.
    fn validate_binding(
        &self,
        pairing: &Pairing,
        report: &RunReport,
        receipt: &ForwardReceipt,
        now: u64,
    ) -> Result<()> {
        report.validate(now)?;
        let b = &self.binding;
        ensure!(
            matches!(b.schema.as_str(), JOB_SCHEMA | JOB_SCHEMA_V2)
                && self.request.matches_schema(&b.schema)
                && identifier(&b.job_id)
                && identifier(&b.run_id)
                && valid_digest(&b.request_digest)
                && valid_digest(&b.received_digest)
                && b.request_digest == self.request.digest()?
                && b.subject == pairing.subject
                && b.agent_id == pairing.agent_id
                && b.subject == report.subject
                && b.agent_id == report.agent_id
                && b.run_id == report.run_id
                && b.consent_digest == report.consent_digest
                && b.report_digest == report.digest
                && report.status == "complete"
                && b.expires_at == report.expires_at
                && b.expires_at <= pairing.expires_at
                && now < b.expires_at,
            "agent_journey_binding"
        );
        ensure!(
            receipt.schema == "codefriend.agent_report_receipt.v1"
                && receipt.subject == b.subject
                && receipt.agent_id == b.agent_id
                && receipt.run_id == b.run_id
                && receipt.report_digest == b.report_digest
                && receipt.consent_digest == b.consent_digest
                && receipt.received_digest == b.received_digest
                && receipt.expires_at == b.expires_at,
            "agent_journey_receipt_changed"
        );
        ensure!(
            self.permitted_agent_candidate == env!("CODEFRIEND_BUILD_REVISION")
                && env!("CODEFRIEND_BUILD_CLEAN") == "true",
            "agent_journey_candidate"
        );
        if let Some(baseline) = self.request.baseline() {
            ensure!(
                identifier(baseline) && baseline != b.run_id,
                "agent_journey_baseline_identity"
            );
        }
        if let Request::AttachPublication {
            publication_job, ..
        } = &self.request
        {
            ensure!(
                identifier(publication_job),
                "agent_journey_publication_identity"
            );
        }
        Ok(())
    }
}

struct LocalOwner {
    pairing: Pairing,
    command: Command,
    report: RunReport,
    root: PathBuf,
    consent_binding: LocalConsentBinding,
}

impl Transport {
    // Requires the existing bounded private publication::read helper to become
    // pub(super). It rejects symlink components and broad file permissions.
    fn journey_owner(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
    ) -> Result<LocalOwner> {
        let now = (self.clock)();
        let pairing = journal.pairing(now)?;
        self.paired(&pairing)?;
        ensure!(identifier(&job.binding.run_id), "agent_journey_run_id");
        let root = journal.root.join(format!("run-{}", job.binding.run_id));
        let command: Command = publication::read(&root.join("command.json"), 8192)?;
        command.validate(&pairing, &read_consent(consent_path, now)?, now)?;
        let consent_binding: LocalConsentBinding =
            publication::read(&root.join("local-consent.json"), 8192)?;
        ensure!(
            consent_binding.path == fs::canonicalize(consent_path)?
                && consent_binding.digest == command.consent_digest,
            "agent_journey_consent_binding_changed"
        );
        let report: RunReport =
            publication::read(&root.join("report.json"), MAX_RESPONSE as usize)?;
        let receipt = self.publication_receipt(journal, &job.binding.run_id, consent_path)?;
        job.validate_binding(&pairing, &report, &receipt, (self.clock)())?;
        let owner = LocalOwner {
            pairing,
            command,
            report,
            root,
            consent_binding,
        };
        self.recheck_journey_owner(journal, consent_path, job, &owner)?;
        // Validate the original Store then release its lock before native Journey
        // opens it. Holding this guard across prepare/resume would deadlock.
        let store_path = owner.root.join("evidence");
        ensure!(store_path.is_dir(), "agent_journey_original_store_missing");
        let clock = self.clock.clone();
        let store = super::super::evidence::store::Store::open(&store_path, move || clock())?;
        let review = owner.report.selected_review()?;
        let original = if owner.report.cycle_result.is_some() {
            publication::read(&owner.root.join("admission.json"), MAX_RESPONSE as usize)?
        } else {
            review.review_record.admission.clone()
        };
        ensure!(
            store.get(&review.review_record.admission.packet.packet_id)? == original
                && original.packet == review.review_record.admission.packet
                && (owner.report.cycle_result.is_some()
                    || original == review.review_record.admission),
            "agent_journey_original_admission_changed"
        );
        drop(store);
        // Original native run.json may be0644 beneath the private owner. The
        // mandatory native prepare/resume immediately following this loader
        // validates its bounded bytes and complete original review inventory.
        // Do not apply transport-record0600 policy to native runner artifacts.
        Ok(owner)
    }

    fn recheck_journey_owner(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
        owner: &LocalOwner,
    ) -> Result<()> {
        let now = (self.clock)();
        let pairing = journal.pairing(now)?;
        ensure!(
            hash(&pairing)? == hash(&owner.pairing)?,
            "agent_journey_pairing_changed"
        );
        let current_command: Command = publication::read(&owner.root.join("command.json"), 8192)?;
        let binding: LocalConsentBinding =
            publication::read(&owner.root.join("local-consent.json"), 8192)?;
        ensure!(
            binding == owner.consent_binding
                && binding.path == fs::canonicalize(consent_path)?
                && binding.digest == current_command.consent_digest,
            "agent_journey_consent_binding_changed"
        );
        ensure!(
            hash(&current_command)? == hash(&owner.command)?,
            "agent_journey_command_changed"
        );
        current_command.validate(&pairing, &read_consent(consent_path, now)?, now)?;
        let report: RunReport =
            publication::read(&owner.root.join("report.json"), MAX_RESPONSE as usize)?;
        report.validate(now)?;
        ensure!(
            hash(&report)? == hash(&owner.report)?,
            "agent_journey_report_changed"
        );
        let expiry: u64 = publication::read(&owner.root.join("expires.json"), 64)?;
        ensure!(
            expiry == job.binding.expires_at && expiry > now,
            "agent_journey_expired"
        );
        ensure!(
            !self.control(&pairing, &current_command)?,
            "agent_cancelled"
        );
        // Network observation can outlive consent or pairing: recheck afterward.
        let now = (self.clock)();
        let after = journal.pairing(now)?;
        ensure!(
            hash(&after)? == hash(&pairing)?,
            "agent_journey_pairing_changed"
        );
        current_command.validate(&after, &read_consent(consent_path, now)?, now)?;
        let command_after: Command = publication::read(&owner.root.join("command.json"), 8192)?;
        let report_after: RunReport =
            publication::read(&owner.root.join("report.json"), MAX_RESPONSE as usize)?;
        let expiry_after: u64 = publication::read(&owner.root.join("expires.json"), 64)?;
        let binding_after: LocalConsentBinding =
            publication::read(&owner.root.join("local-consent.json"), 8192)?;
        ensure!(
            hash(&command_after)? == hash(&owner.command)?
                && hash(&report_after)? == hash(&owner.report)?
                && binding_after == owner.consent_binding
                && expiry_after == expiry,
            "agent_journey_authority_changed"
        );
        report_after.validate(now)?;
        report_after.check_original_cycle(&owner.root, now)?;
        ensure!(now < expiry_after, "agent_journey_expired");
        Ok(())
    }
}

// Transport/execution wiring follows the owner checks above.
struct BaselineOwner {
    owner: LocalOwner,
    binding: Job,
    store: Store,
    graph: StructureArtifact,
    baseline_root: PathBuf,
}

impl BaselineOwner {
    fn context(&self) -> native::owned_baseline::OwnedBaseline<'_> {
        native::owned_baseline::OwnedBaseline {
            operation: &self.owner.command.run_id,
            store: &self.store,
            graph: &self.graph,
            review: &self
                .owner
                .report
                .selected_review()
                .expect("validated complete report")
                .review_record,
            baseline_root: &self.baseline_root,
            expires_at: self.owner.report.expires_at,
        }
    }
}

impl Transport {
    fn journey_baseline(
        &self,
        journal: &Journal,
        current: &Job,
        baseline: &str,
    ) -> Result<BaselineOwner> {
        ensure!(
            identifier(baseline) && baseline != current.binding.run_id,
            "agent_journey_baseline_identity"
        );
        let root = journal.root.join(format!("run-{baseline}"));
        let consent: LocalConsentBinding =
            publication::read(&root.join("local-consent.json"), 8192)?;
        ensure!(consent.path.is_absolute(), "agent_journey_consent_path");
        let report: RunReport =
            publication::read(&root.join("report.json"), MAX_RESPONSE as usize)?;
        report.validate((self.clock)())?;
        let receipt = self.publication_receipt(journal, baseline, &consent.path)?;
        // This local observation binding is derived from the authenticated
        // original receipt. It is never a new browser job or effect authority.
        let request = Request::Status;
        let binding = Job {
            binding: Binding {
                schema: current.binding.schema.clone(),
                job_id: current.binding.job_id.clone(),
                subject: current.binding.subject.clone(),
                agent_id: current.binding.agent_id.clone(),
                run_id: baseline.into(),
                consent_digest: consent.digest,
                report_digest: report.digest,
                received_digest: receipt.received_digest,
                request_digest: request.digest()?,
                expires_at: report.expires_at,
            },
            request,
            permitted_agent_candidate: current.permitted_agent_candidate.clone(),
        };
        let owner = self.journey_owner(journal, &consent.path, &binding)?;
        let output = root.join("journey");
        let graph: StructureArtifact =
            publication::read(&output.join("structure.json"), MAX_RESPONSE as usize)?;
        native::owned_baseline::validate_graph_source(&output, &graph)?;
        ensure!(
            native::owned_baseline::original_review(&output)?
                == owner
                    .report
                    .selected_review()
                    .expect("validated complete report")
                    .review_record,
            "agent_journey_baseline_review_changed"
        );
        let clock = self.clock.clone();
        let store = Store::open(&owner.report.review_store(&root), move || clock())?;
        let baseline = BaselineOwner {
            owner,
            binding,
            store,
            graph,
            baseline_root: root.join("baseline-owners"),
        };
        baseline.context().validate()?;
        Ok(baseline)
    }

    fn selected_journey_baseline(
        &self,
        journal: &Journal,
        job: &Job,
        owner: &LocalOwner,
    ) -> Result<Option<BaselineOwner>> {
        let output = owner.root.join("journey");
        let (drift, palace) = if output.exists() {
            (
                native::owned_baseline::operation(&output)?,
                native::owned_palace::operation(&output)?,
            )
        } else {
            (None, None)
        };
        ensure!(
            drift.is_none() || palace.is_none() || drift == palace,
            "agent_journey_baseline_changed"
        );
        let saved = drift.or(palace);
        if let (Some(selected), Some(saved)) = (job.request.baseline(), saved.as_deref()) {
            ensure!(selected == saved, "agent_journey_baseline_changed");
        }
        job.request
            .baseline()
            .or(saved.as_deref())
            .map(|id| self.journey_baseline(journal, job, id))
            .transpose()
    }

    fn recheck_journey_baseline(&self, journal: &Journal, baseline: &BaselineOwner) -> Result<()> {
        self.recheck_journey_owner(
            journal,
            &baseline.owner.consent_binding.path,
            &baseline.binding,
            &baseline.owner,
        )?;
        baseline.context().validate()?;
        native::owned_baseline::validate_graph_source(
            &baseline.owner.root.join("journey"),
            &baseline.graph,
        )?;
        Ok(())
    }
}

const RESULT_SCHEMA: &str = "codefriend.agent_journey_result.v1";
const RESULT_SCHEMA_V2: &str = "codefriend.agent_journey_result.v2";

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StageResult {
    pub schema: String,
    pub binding: Binding,
    pub agent_candidate_revision: String,
    pub checkpoint_sequence: usize,
    pub manifest: native::JourneyManifest,
    pub payload: Option<serde_json::Value>,
    pub digest: String,
}

impl Artifact {
    fn key(&self) -> &'static str {
        match self {
            Self::Structure => "structure",
            Self::Fitness => "fitness",
            Self::Impact => "impact",
            Self::Rationale => "rationale",
            Self::Drift => "drift",
            Self::PalaceComparison => "palace_comparison",
        }
    }
}

fn artifact_digest(artifact: &Artifact, value: &serde_json::Value) -> Result<String> {
    // Reconstruct the shared native type: hashing a generic JSON map would
    // change field order and would not verify the original stage's digest.
    match artifact {
        Artifact::Structure => hash(&serde_json::from_value::<StructureArtifact>(value.clone())?),
        Artifact::Fitness => hash(&serde_json::from_value::<FitnessArtifact>(value.clone())?),
        Artifact::Impact => hash(&serde_json::from_value::<ImpactArtifact>(value.clone())?),
        Artifact::Rationale => hash(&serde_json::from_value::<RationaleArtifact>(value.clone())?),
        Artifact::Drift => hash(&serde_json::from_value::<
            native::owned_baseline::OwnedDriftReport,
        >(value.clone())?),
        Artifact::PalaceComparison => hash(&serde_json::from_value::<
            crate::codefriend::memory::palace::RetrievedComparison,
        >(value.clone())?),
    }
}

impl Transport {
    fn journey_remote_job(&self, pairing: &Pairing, expected: &Job) -> Result<()> {
        let current: Job = self.request(
            Method::GET,
            &format!("/v1/agent/journeys/{}", expected.binding.job_id),
            Some(&pairing.agent_token),
            None,
        )?;
        ensure!(
            hash(&current)? == hash(expected)?,
            "agent_journey_job_changed"
        );
        Ok(())
    }

    fn journey_publication_context(
        &self,
        journal: &Journal,
        consent_path: &Path,
        owner: &LocalOwner,
        publication_job: &str,
        format: PublicationFormat,
    ) -> Result<publication::VerificationContext> {
        ensure!(
            identifier(publication_job),
            "agent_journey_publication_identity"
        );
        let job: publication::Job = self.request(
            Method::GET,
            &format!("/v1/agent/publications/{publication_job}"),
            Some(&owner.pairing.agent_token),
            None,
        )?;
        ensure!(
            job.status == "complete"
                && job.binding.job_id == publication_job
                && job.binding.format == format
                && job.binding.run_id == owner.command.run_id
                && job.binding.agent_id == owner.pairing.agent_id
                && job.binding.subject == owner.pairing.subject
                && job.binding.report_digest == owner.report.digest
                && job.binding.consent_digest == owner.command.consent_digest
                && job.binding.expires_at == owner.report.expires_at
                && job.agent_candidate_revision.as_deref()
                    == Some(env!("CODEFRIEND_BUILD_REVISION")),
            "agent_journey_publication_binding"
        );
        let receipt = self.publication_receipt(journal, &owner.command.run_id, consent_path)?;
        ensure!(
            receipt.received_digest == job.binding.received_digest,
            "agent_journey_publication_receipt"
        );
        let root = owner.root.join("work/publications").join(publication_job);
        let prepared: publication::Stage =
            publication::read(&root.join("prepared-stage.json"), MAX_RESPONSE as usize)?;
        let terminal: publication::Stage =
            publication::read(&root.join("terminal-stage.json"), MAX_RESPONSE as usize)?;
        ensure!(
            job.prepared_digest.as_deref() == Some(&prepared.digest)
                && job.terminal_digest.as_deref() == Some(&terminal.digest),
            "agent_journey_publication_stage_changed"
        );
        let context = publication::VerificationContext {
            schema: "codefriend.agent_publication_verifier_context.v1".into(),
            binding: job.binding,
            report: publication::read(&owner.root.join("report.json"), MAX_RESPONSE as usize)?,
            decision: job.decision,
            prepared: Some(prepared),
            now: (self.clock)(),
        };
        publication::verify_stage(&terminal, &context, (self.clock)())?;
        let terminal: publication::Terminal = serde_json::from_value(terminal.payload)?;
        ensure!(
            terminal.status == "complete",
            "agent_journey_publication_not_complete"
        );
        Ok(context)
    }

    fn execute_journey_job(
        &self,
        journal: &Journal,
        consent_path: &Path,
        job: &Job,
        first_effect: bool,
    ) -> Result<StageResult> {
        let owner = self.journey_owner(journal, consent_path, job)?;
        self.journey_remote_job(&owner.pairing, job)?;
        self.recheck_journey_owner(journal, consent_path, job, &owner)?;
        let baseline = self.selected_journey_baseline(journal, job, &owner)?;
        let baseline_context = baseline.as_ref().map(BaselineOwner::context);
        let output = owner.root.join("journey");
        let palace = native::owned_palace::AuthorityContext {
            root: journal.root.join("palace-authority"),
            palace_root: owner.root.join("palace"),
        };
        let mut journey = if first_effect {
            if let Request::Prepare {
                boundary_policy,
                fitness_policy,
            } = &job.request
            {
                let run = owner
                    .report
                    .selected_review()
                    .expect("validated complete report");
                native::prepare_owned_admission(native::OwnedAdmissionJourneyOptions {
                    store: owner.report.review_store(&owner.root),
                    output: output.clone(),
                    owner_root: owner.root.clone(),
                    review_root: owner.root.join("work/review"),
                    packet_id: run.review_record.admission.packet.packet_id.clone(),
                    admission_digest: run.review_record.admission.digest.clone(),
                    operation_id: run.run_id.clone(),
                    candidate_revision: job.permitted_agent_candidate.clone(),
                    expires_at: owner.report.expires_at,
                    completed_run: run.clone(),
                    boundary_policy: boundary_policy.clone(),
                    fitness_policy: fitness_policy.clone(),
                })?
            } else {
                native::resume_with_owners(&output, baseline_context.as_ref(), Some(&palace))?
            }
        } else {
            // A retained reservation only permits reconstruction/observation.
            native::resume_with_owners(&output, baseline_context.as_ref(), Some(&palace))?
        };
        ensure!(
            journey.manifest().schema
                == if job.binding.schema == JOB_SCHEMA_V2 {
                    "codefriend.journey.v2"
                } else {
                    "codefriend.journey.v1"
                },
            "agent_journey_version_mismatch"
        );
        if first_effect {
            match &job.request {
                Request::Impact { changes } => {
                    journey.continue_with(native::Continuation::Impact {
                        changes: changes.clone(),
                    })?
                }
                Request::Rationale { selection } => {
                    journey.continue_with(native::Continuation::Rationale {
                        selection: selection.clone(),
                    })?
                }
                Request::Drift { .. } => journey.continue_owned_drift(
                    baseline_context
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("agent_journey_baseline_missing"))?,
                )?,
                Request::PalaceComparison { .. } => journey.continue_owned_palace(
                    baseline_context
                        .as_ref()
                        .ok_or_else(|| anyhow::anyhow!("agent_journey_baseline_missing"))?,
                    &palace,
                )?,
                Request::AttachPublication {
                    publication_job,
                    format,
                } => {
                    let context = self.journey_publication_context(
                        journal,
                        consent_path,
                        &owner,
                        publication_job,
                        *format,
                    )?;
                    journey.attach_local_publication(&context)?;
                }
                Request::Prepare { .. }
                | Request::Status
                | Request::Graph
                | Request::Artifact { .. } => {}
            }
        }
        let required_stage = match &job.request {
            Request::Prepare { .. } => Some("fitness"),
            Request::Impact { .. } => Some("impact"),
            Request::Rationale { .. } => Some("rationale"),
            Request::Drift { .. } => Some("drift"),
            Request::PalaceComparison { .. } => Some("palace_comparison"),
            Request::AttachPublication { format, .. } => Some(format.key()),
            _ => None,
        };
        if let Some(key) = required_stage {
            ensure!(
                journey.manifest().stages[key].status != native::StageStatus::Pending,
                "agent_journey_reserved_effect_unresolved"
            );
        }
        let requested_artifact = match &job.request {
            Request::Graph => Some(Artifact::Structure),
            Request::Artifact { artifact } => Some(artifact.clone()),
            _ => None,
        };
        let payload = if let Some(artifact) = requested_artifact {
            let key = artifact.key();
            let stage = &journey.manifest().stages[key];
            ensure!(
                stage.status != native::StageStatus::Pending
                    && stage.artifact.as_deref() == Some(format!("{key}.json").as_str()),
                "agent_journey_artifact_unavailable"
            );
            let value: serde_json::Value =
                publication::read(&output.join(format!("{key}.json")), MAX_RESPONSE as usize)?;
            ensure!(
                stage.digest.as_deref() == Some(artifact_digest(&artifact, &value)?.as_str()),
                "agent_journey_artifact_changed"
            );
            Some(value)
        } else {
            None
        };
        self.journey_remote_job(&owner.pairing, job)?;
        self.recheck_journey_owner(journal, consent_path, job, &owner)?;
        if let Some(baseline) = &baseline {
            self.recheck_journey_baseline(journal, baseline)?;
        }
        if let Request::AttachPublication {
            publication_job,
            format,
        } = &job.request
        {
            ensure!(
                journey.manifest().stages[format.key()].status == native::StageStatus::Complete,
                "agent_journey_attachment_unresolved"
            );
            let context = self.journey_publication_context(
                journal,
                consent_path,
                &owner,
                publication_job,
                *format,
            )?;
            journey.attach_local_publication(&context)?;
            self.recheck_journey_owner(journal, consent_path, job, &owner)?;
        }
        journey.validate_owned_palace(baseline_context.as_ref(), &palace)?;
        journey.continue_with(native::Continuation::Status)?;
        let mut result = StageResult {
            schema: if job.binding.schema == JOB_SCHEMA_V2 {
                RESULT_SCHEMA_V2
            } else {
                RESULT_SCHEMA
            }
            .into(),
            binding: job.binding.clone(),
            agent_candidate_revision: env!("CODEFRIEND_BUILD_REVISION").into(),
            checkpoint_sequence: journey.checkpoint_sequence(),
            manifest: journey.manifest().clone(),
            payload,
            digest: String::new(),
        };
        result.digest = hash(&result)?;
        ensure!(
            serde_json::to_vec(&result)?.len() as u64 <= MAX_RESPONSE,
            "agent_journey_result_limit"
        );
        Ok(result)
    }
}
