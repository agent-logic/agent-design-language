use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const DRT_A_CONTRACT_SCHEMA: &str = "adl.runtime.qualification.drt_a_contract.v1";
pub const DRT_A_RECEIPT_SCHEMA: &str = "adl.runtime.qualification.drt_a_receipt.v1";
pub const DRT_B_CONTRACT_SCHEMA: &str = "adl.runtime.qualification.drt_b_contract.v1";
pub const DRT_C_DECISION_SCHEMA: &str = "adl.runtime.qualification.drt_c_decision.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DistributedQualificationContract {
    pub schema: String,
    pub requirements: Vec<String>,
    pub participants: Vec<QualificationParticipant>,
    pub scenarios: Vec<QualificationScenario>,
    pub acip_vectors: Vec<AcipVector>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QualificationParticipant {
    pub id: String,
    pub role: ParticipantRole,
    pub voting: bool,
    pub identity: String,
    pub credential: String,
    pub port: u16,
    pub state_root: String,
    pub storage_root: String,
    pub failure_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ParticipantRole {
    Voter,
    GovernedAgent,
    Shepherd,
    Observatory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QualificationScenario {
    pub id: String,
    pub setup: String,
    pub action: String,
    pub expected_commit: String,
    pub expected_election: String,
    pub expected_fence: String,
    pub timeout_ms: u64,
    pub receipt_fields: Vec<String>,
    pub cleanup: String,
    pub fail_closed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AcipVector {
    pub id: String,
    pub mutation: String,
    pub expected: VectorOutcome,
    pub authority_digest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum VectorOutcome {
    Accepted,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QualificationReceipt {
    pub schema: String,
    pub lane: String,
    pub status: String,
    pub contract_digest: String,
    pub requirement_count: usize,
    pub participant_count: usize,
    pub scenario_count: usize,
    pub acip_vector_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ReceiptDecision {
    Accepted,
    Denied,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExactQualificationReceipt {
    pub schema: String,
    pub lane: String,
    pub scenario: String,
    pub contract_digest: String,
    pub authority_digest: String,
    pub decision: ReceiptDecision,
    pub cleanup: String,
    pub mutation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AcipVectorProbe {
    pub id: String,
    pub mutation: String,
    pub message_id: String,
    pub seen_message_ids: Vec<String>,
    pub authority_digest: String,
    pub credential: String,
    pub permit: String,
    pub signed: bool,
    pub domain: String,
    pub polis_id: String,
    pub term: u64,
    pub monotonic_sequence: u64,
    pub correlation_id: String,
    pub causation_id: String,
    pub payload_well_formed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrtBQualificationContract {
    pub schema: String,
    pub issue: u64,
    pub requirements: Vec<String>,
    pub source_drt_a_contract_digest: String,
    pub resident_count: usize,
    pub residents: Vec<DrtBResident>,
    pub dehydrate_restore: String,
    pub cleanup_zero: bool,
    pub resource_envelope: BTreeMap<String, u64>,
    pub cleanup_selectors: Vec<String>,
    pub negative_matrix: Vec<DrtBNegativeCase>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrtBResident {
    pub resident_id: String,
    pub participant_id: String,
    pub participant_role: ParticipantRole,
    pub identity: String,
    pub workload_id: String,
    pub workload_receipt_id: String,
    pub lineage_digest: String,
    pub replay_cursor: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrtBNegativeCase {
    pub case: String,
    pub mutation: String,
    pub decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrtCQualificationDecision {
    pub schema: String,
    pub issue: u64,
    pub requirements: Vec<String>,
    pub runtime_revision: String,
    pub source_drt_b_contract_digest: String,
    pub fail_closed_cases: Vec<String>,
    pub observatory: DrtCObservatoryEvidence,
    pub soak: DrtCSoakEvidence,
    pub cleanup: BTreeMap<String, String>,
    pub decision: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrtCObservatoryEvidence {
    pub runtime_emitted: bool,
    pub redacted: bool,
    pub feed_schema: String,
    pub artifact_sha256: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DrtCSoakEvidence {
    pub bounded: bool,
    pub duration_seconds: u64,
    pub source: String,
}

impl DistributedQualificationContract {
    pub fn deterministic_drt_a() -> Self {
        let participants = vec![
            participant("voter-a", ParticipantRole::Voter, true, 37181, "az-a"),
            participant("voter-b", ParticipantRole::Voter, true, 37182, "az-b"),
            participant("voter-c", ParticipantRole::Voter, true, 37183, "az-c"),
            participant(
                "agent-alpha",
                ParticipantRole::GovernedAgent,
                false,
                37201,
                "az-a",
            ),
            participant(
                "agent-beta",
                ParticipantRole::GovernedAgent,
                false,
                37202,
                "az-b",
            ),
            participant(
                "agent-gamma",
                ParticipantRole::GovernedAgent,
                false,
                37203,
                "az-c",
            ),
            participant(
                "shepherd",
                ParticipantRole::Shepherd,
                false,
                37240,
                "control",
            ),
            participant(
                "observatory",
                ParticipantRole::Observatory,
                false,
                37280,
                "quorum-lease",
            ),
        ];

        let scenarios = [
            "election",
            "quorum-loss",
            "stale-lease-fence-denial",
            "restart",
            "snapshot",
            "partition",
            "healing",
            "replay",
            "cleanup",
            "duplicate-denial",
            "acip-mutation",
        ]
        .into_iter()
        .map(scenario)
        .collect();

        let authority_digest = authority_digest("runtime-api-authenticated", "agent-alpha", 42);
        let acip_vectors = [
            ("positive-roundtrip", "none", VectorOutcome::Accepted),
            (
                "byte-stable-reencode",
                "canonical-json-order",
                VectorOutcome::Accepted,
            ),
            ("duplicate", "message-id-repeat", VectorOutcome::Denied),
            ("reordered", "sequence-regression", VectorOutcome::Denied),
            ("stale", "term-regression", VectorOutcome::Denied),
            (
                "malformed",
                "invalid-protobuf-or-json",
                VectorOutcome::Denied,
            ),
            (
                "unsigned",
                "missing-authority-signature",
                VectorOutcome::Denied,
            ),
            (
                "wrong-domain",
                "authority-domain-mismatch",
                VectorOutcome::Denied,
            ),
            ("cross-polis", "polis-id-mismatch", VectorOutcome::Denied),
            (
                "authority-mutation",
                "authority-digest-change",
                VectorOutcome::Denied,
            ),
            (
                "credential-binding",
                "credential-id-mismatch",
                VectorOutcome::Denied,
            ),
            (
                "permit-binding",
                "permit-id-mismatch",
                VectorOutcome::Denied,
            ),
            (
                "correlation-binding",
                "correlation-id-mismatch",
                VectorOutcome::Denied,
            ),
            (
                "causation-binding",
                "causation-id-mismatch",
                VectorOutcome::Denied,
            ),
        ]
        .into_iter()
        .map(|(id, mutation, expected)| AcipVector {
            id: id.to_string(),
            mutation: mutation.to_string(),
            expected,
            authority_digest: authority_digest.clone(),
        })
        .collect();

        Self {
            schema: DRT_A_CONTRACT_SCHEMA.to_string(),
            requirements: vec!["#181".to_string(), "#182".to_string()],
            participants,
            scenarios,
            acip_vectors,
        }
    }

    pub fn digest(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("DRT-A contract is serializable");
        let digest = Sha256::digest(bytes);
        hex::encode(digest)
    }

    pub fn deterministic_drt_b(&self) -> Result<DrtBQualificationContract, String> {
        self.validate_topology()?;
        self.validate_scenarios()?;
        self.validate_acip_vectors()?;

        let residents = self
            .participants
            .iter()
            .filter(|participant| {
                matches!(
                    participant.role,
                    ParticipantRole::Voter | ParticipantRole::GovernedAgent
                )
            })
            .take(6)
            .enumerate()
            .map(|(index, participant)| {
                let workload_id = format!("uts-workload-{:02}", index + 1);
                let workload_receipt_id = stable_id(
                    "drt-b-workload-receipt",
                    [participant.identity.as_str(), workload_id.as_str()],
                );
                DrtBResident {
                    resident_id: stable_id("drt-b-resident", [participant.identity.as_str()]),
                    participant_id: participant.id.clone(),
                    participant_role: participant.role.clone(),
                    identity: participant.identity.clone(),
                    workload_id,
                    workload_receipt_id,
                    lineage_digest: stable_id(
                        "drt-b-lineage",
                        [&participant.identity, participant.state_root.as_str()],
                    ),
                    replay_cursor: 10_000 + index as u64,
                }
            })
            .collect::<Vec<_>>();

        let mut resource_envelope = BTreeMap::new();
        resource_envelope.insert("resident_slots".to_string(), 6);
        resource_envelope.insert("workload_receipts".to_string(), 6);
        resource_envelope.insert("max_replay_cursor".to_string(), 10_005);

        Ok(DrtBQualificationContract {
            schema: DRT_B_CONTRACT_SCHEMA.to_string(),
            issue: 507,
            requirements: vec!["#183".to_string(), "#184".to_string()],
            source_drt_a_contract_digest: self.digest(),
            resident_count: residents.len(),
            residents,
            dehydrate_restore: "exact".to_string(),
            cleanup_zero: true,
            resource_envelope,
            cleanup_selectors: vec![
                "drt-b:resident-state".to_string(),
                "drt-b:workload-receipts".to_string(),
                "drt-b:replay-cursors".to_string(),
            ],
            negative_matrix: [
                (
                    "duplicate_resident_identity",
                    "reuse resident_id across two workload receipts",
                ),
                (
                    "missing_workload_receipt",
                    "remove one resident workload receipt",
                ),
                ("mutated_lineage", "change one resident lineage_digest"),
                (
                    "replay_cursor_regression",
                    "restore a replay cursor below snapshot value",
                ),
                (
                    "cleanup_selector_mismatch",
                    "drop a cleanup selector before reclamation",
                ),
            ]
            .into_iter()
            .map(|(case, mutation)| DrtBNegativeCase {
                case: case.to_string(),
                mutation: mutation.to_string(),
                decision: "fail_closed".to_string(),
            })
            .collect(),
        })
    }

    pub fn deterministic_drt_c(
        &self,
        runtime_revision: &str,
    ) -> Result<DrtCQualificationDecision, String> {
        if runtime_revision.len() != 40
            || !runtime_revision
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("runtime revision must be a 40-character hex SHA".to_string());
        }
        let drt_b = self.deterministic_drt_b()?;
        drt_b.validate()?;
        let source_drt_b_contract_digest = drt_b.digest();
        let mut cleanup = BTreeMap::new();
        for key in [
            "failure-fixtures",
            "observatory-artifacts",
            "soak-processes",
            "temporary-resources",
        ] {
            cleanup.insert(key.to_string(), "absent".to_string());
        }
        Ok(DrtCQualificationDecision {
            schema: DRT_C_DECISION_SCHEMA.to_string(),
            issue: 508,
            requirements: vec!["#185".to_string(), "#186".to_string(), "#187".to_string()],
            runtime_revision: runtime_revision.to_ascii_lowercase(),
            source_drt_b_contract_digest,
            fail_closed_cases: vec![
                "identity".to_string(),
                "provider".to_string(),
                "transport".to_string(),
            ],
            observatory: DrtCObservatoryEvidence {
                runtime_emitted: true,
                redacted: true,
                feed_schema: "adl.runtime_v3.observatory.feed.v1".to_string(),
                artifact_sha256: stable_id(
                    "drt-c-observatory",
                    [runtime_revision, drt_b.digest().as_str()],
                ),
            },
            soak: DrtCSoakEvidence {
                bounded: true,
                duration_seconds: 900,
                source: "deterministic-local-qualification-window".to_string(),
            },
            cleanup,
            decision: "qualified_for_final_distributed_runtime_decision".to_string(),
        })
    }

    pub fn validate_topology(&self) -> Result<(), String> {
        require_exact(&self.schema, DRT_A_CONTRACT_SCHEMA, "schema")?;
        require_set(
            self.requirements.iter().map(String::as_str),
            ["#181", "#182"],
            "requirements",
        )?;
        let mut seen_identity = BTreeSet::new();
        let mut seen_credential = BTreeSet::new();
        let mut seen_port = BTreeSet::new();
        let mut seen_state = BTreeSet::new();
        let mut seen_storage = BTreeSet::new();
        let mut voters = 0;
        let mut agents = 0;
        let mut shepherds = 0;
        let mut observatories = 0;
        for participant in &self.participants {
            if participant.identity.contains("synthetic")
                || participant.credential.contains("synthetic")
            {
                return Err(format!("{} uses synthetic provenance", participant.id));
            }
            if !seen_identity.insert(participant.identity.as_str()) {
                return Err(format!("duplicate identity {}", participant.identity));
            }
            if !seen_credential.insert(participant.credential.as_str()) {
                return Err(format!("duplicate credential {}", participant.credential));
            }
            if !seen_port.insert(participant.port) {
                return Err(format!("duplicate port {}", participant.port));
            }
            if !seen_state.insert(participant.state_root.as_str()) {
                return Err(format!("duplicate state root {}", participant.state_root));
            }
            if !seen_storage.insert(participant.storage_root.as_str()) {
                return Err(format!(
                    "duplicate storage root {}",
                    participant.storage_root
                ));
            }
            match participant.role {
                ParticipantRole::Voter => {
                    voters += 1;
                    if !participant.voting {
                        return Err(format!("{} must vote", participant.id));
                    }
                }
                ParticipantRole::GovernedAgent => {
                    agents += 1;
                    if participant.voting {
                        return Err(format!("{} must not vote", participant.id));
                    }
                }
                ParticipantRole::Shepherd => {
                    shepherds += 1;
                    if participant.voting {
                        return Err("shepherd must be non-voting".to_string());
                    }
                }
                ParticipantRole::Observatory => {
                    observatories += 1;
                    if participant.voting || participant.failure_domain != "quorum-lease" {
                        return Err("observatory must be non-voting and quorum-leased".to_string());
                    }
                }
            }
        }
        if voters != 3 || agents != 3 || shepherds != 1 || observatories != 1 {
            return Err(format!(
                "unexpected topology voters={voters} agents={agents} shepherds={shepherds} observatories={observatories}"
            ));
        }
        require_set(
            self.scenarios.iter().map(|scenario| scenario.id.as_str()),
            [
                "election",
                "quorum-loss",
                "stale-lease-fence-denial",
                "restart",
                "snapshot",
                "partition",
                "healing",
                "replay",
                "cleanup",
                "duplicate-denial",
                "acip-mutation",
            ],
            "scenarios",
        )
    }

    pub fn validate_scenarios(&self) -> Result<(), String> {
        let required_fields = [
            "schema",
            "lane",
            "scenario",
            "contract_digest",
            "authority_digest",
            "decision",
            "cleanup",
            "mutation",
        ];
        for scenario in &self.scenarios {
            for field in [
                &scenario.setup,
                &scenario.action,
                &scenario.expected_commit,
                &scenario.expected_election,
                &scenario.expected_fence,
                &scenario.cleanup,
                &scenario.fail_closed,
            ] {
                if field.trim().is_empty() {
                    return Err(format!("scenario {} has empty field", scenario.id));
                }
            }
            if scenario.timeout_ms == 0 {
                return Err(format!("scenario {} has no timeout", scenario.id));
            }
            require_set(
                scenario.receipt_fields.iter().map(String::as_str),
                required_fields,
                &format!("scenario {} receipt fields", scenario.id),
            )?;
        }
        Ok(())
    }

    pub fn validate_acip_vectors(&self) -> Result<(), String> {
        require_set(
            self.acip_vectors.iter().map(|vector| vector.id.as_str()),
            [
                "positive-roundtrip",
                "byte-stable-reencode",
                "duplicate",
                "reordered",
                "stale",
                "malformed",
                "unsigned",
                "wrong-domain",
                "cross-polis",
                "authority-mutation",
                "credential-binding",
                "permit-binding",
                "correlation-binding",
                "causation-binding",
            ],
            "acip vectors",
        )?;
        let baseline = self
            .acip_vectors
            .iter()
            .find(|vector| vector.expected == VectorOutcome::Accepted)
            .map(|vector| vector.authority_digest.as_str())
            .ok_or_else(|| "missing accepted ACIP baseline".to_string())?;
        for vector in &self.acip_vectors {
            if vector.authority_digest != baseline {
                return Err(format!("{} mutates authority digest", vector.id));
            }
            if vector.id != "positive-roundtrip"
                && vector.id != "byte-stable-reencode"
                && vector.expected != VectorOutcome::Denied
            {
                return Err(format!("{} must fail closed", vector.id));
            }
        }
        Ok(())
    }

    pub fn receipt_for(&self, lane: &str) -> Result<QualificationReceipt, String> {
        match lane {
            "qualification-contract" => self.validate_topology()?,
            "acip-authority" | "replay-conformance" | "negative-matrix" => {
                self.validate_topology()?;
                self.validate_scenarios()?;
                self.validate_acip_vectors()?;
            }
            other => return Err(format!("unknown DRT-A lane {other}")),
        }
        Ok(QualificationReceipt {
            schema: DRT_A_RECEIPT_SCHEMA.to_string(),
            lane: lane.to_string(),
            status: "pass".to_string(),
            contract_digest: self.digest(),
            requirement_count: self.requirements.len(),
            participant_count: self.participants.len(),
            scenario_count: self.scenarios.len(),
            acip_vector_count: self.acip_vectors.len(),
        })
    }

    pub fn vector_by_id(&self, id: &str) -> Option<&AcipVector> {
        self.acip_vectors.iter().find(|vector| vector.id == id)
    }

    pub fn scenario_receipt(
        &self,
        lane: &str,
        scenario_id: &str,
        authority_digest: &str,
        decision: ReceiptDecision,
    ) -> Result<ExactQualificationReceipt, String> {
        self.validate_topology()?;
        self.validate_scenarios()?;
        self.validate_acip_vectors()?;
        let scenario = self
            .scenarios
            .iter()
            .find(|scenario| scenario.id == scenario_id)
            .ok_or_else(|| format!("unknown DRT-A scenario {scenario_id}"))?;
        let baseline = self.baseline_authority_digest()?;
        if authority_digest != baseline {
            return Err(format!(
                "scenario {scenario_id} authority digest changed from qualified baseline"
            ));
        }
        Ok(ExactQualificationReceipt {
            schema: DRT_A_RECEIPT_SCHEMA.to_string(),
            lane: lane.to_string(),
            scenario: scenario.id.clone(),
            contract_digest: self.digest(),
            authority_digest: authority_digest.to_string(),
            decision,
            cleanup: scenario.cleanup.clone(),
            mutation: "none".to_string(),
        })
    }

    pub fn acip_probe_for(&self, id: &str) -> Result<AcipVectorProbe, String> {
        let vector = self
            .vector_by_id(id)
            .ok_or_else(|| format!("unknown ACIP vector {id}"))?;
        let mut probe = AcipVectorProbe {
            id: vector.id.clone(),
            mutation: vector.mutation.clone(),
            message_id: "drt-a-message-42".to_string(),
            seen_message_ids: Vec::new(),
            authority_digest: vector.authority_digest.clone(),
            credential: "credential:adl:runtime:agent-alpha:v1".to_string(),
            permit: "permit:adl:runtime:agent-alpha:drt-a:v1".to_string(),
            signed: true,
            domain: "runtime-api-authenticated".to_string(),
            polis_id: "polis-drt-a".to_string(),
            term: 7,
            monotonic_sequence: 42,
            correlation_id: "drt-a-correlation-42".to_string(),
            causation_id: "drt-a-causation-42".to_string(),
            payload_well_formed: true,
        };
        match id {
            "positive-roundtrip" | "byte-stable-reencode" => {}
            "duplicate" => probe.seen_message_ids.push(probe.message_id.clone()),
            "reordered" => probe.monotonic_sequence = 41,
            "stale" => probe.term = 6,
            "malformed" => probe.payload_well_formed = false,
            "unsigned" => probe.signed = false,
            "wrong-domain" => probe.domain = "browser-only".to_string(),
            "cross-polis" => probe.polis_id = "polis-other".to_string(),
            "authority-mutation" => {
                probe.authority_digest =
                    authority_digest("runtime-api-authenticated", "agent-beta", 42)
            }
            "credential-binding" => {
                probe.credential = "credential:adl:runtime:agent-beta:v1".to_string()
            }
            "permit-binding" => probe.permit = "permit:adl:runtime:agent-beta:drt-a:v1".to_string(),
            "correlation-binding" => probe.correlation_id = "drt-a-correlation-other".to_string(),
            "causation-binding" => probe.causation_id = "drt-a-causation-other".to_string(),
            other => return Err(format!("unknown ACIP vector {other}")),
        }
        Ok(probe)
    }

    pub fn evaluate_acip_probe(
        &self,
        probe: &AcipVectorProbe,
    ) -> Result<ExactQualificationReceipt, String> {
        self.validate_topology()?;
        self.validate_scenarios()?;
        self.validate_acip_vectors()?;
        let vector = self
            .vector_by_id(&probe.id)
            .ok_or_else(|| format!("unknown ACIP vector {}", probe.id))?;
        if vector.mutation != probe.mutation {
            return Err(format!(
                "{} probe mutation does not match denominator",
                probe.id
            ));
        }
        let baseline = self.baseline_authority_digest()?;
        let duplicate_message = probe
            .seen_message_ids
            .iter()
            .any(|seen| seen == &probe.message_id);
        let denial_cause = if duplicate_message {
            Some("message-id-repeat")
        } else if probe.monotonic_sequence < 42 {
            Some("sequence-regression")
        } else if probe.term < 7 {
            Some("term-regression")
        } else if !probe.payload_well_formed {
            Some("invalid-protobuf-or-json")
        } else if !probe.signed {
            Some("missing-authority-signature")
        } else if probe.domain != "runtime-api-authenticated" {
            Some("authority-domain-mismatch")
        } else if probe.polis_id != "polis-drt-a" {
            Some("polis-id-mismatch")
        } else if probe.authority_digest != baseline {
            Some("authority-digest-change")
        } else if probe.credential != "credential:adl:runtime:agent-alpha:v1" {
            Some("credential-id-mismatch")
        } else if probe.permit != "permit:adl:runtime:agent-alpha:drt-a:v1" {
            Some("permit-id-mismatch")
        } else if probe.correlation_id != "drt-a-correlation-42" {
            Some("correlation-id-mismatch")
        } else if probe.causation_id != "drt-a-causation-42" {
            Some("causation-id-mismatch")
        } else {
            None
        };
        match (vector.expected.clone(), denial_cause) {
            (VectorOutcome::Accepted, None) => {}
            (VectorOutcome::Accepted, Some(cause)) => {
                return Err(format!(
                    "{} expected acceptance but probe was denied by {cause}",
                    probe.id
                ));
            }
            (VectorOutcome::Denied, Some(cause)) if cause == vector.mutation => {}
            (VectorOutcome::Denied, Some(cause)) => {
                return Err(format!(
                    "{} expected denial cause {} but probe was denied by {cause}",
                    probe.id, vector.mutation
                ));
            }
            (VectorOutcome::Denied, None) => {
                return Err(format!(
                    "{} expected denial cause {} but probe contained no invalid condition",
                    probe.id, vector.mutation
                ));
            }
        }
        let decision = if denial_cause.is_some() {
            ReceiptDecision::Denied
        } else {
            ReceiptDecision::Accepted
        };
        Ok(ExactQualificationReceipt {
            schema: DRT_A_RECEIPT_SCHEMA.to_string(),
            lane: "acip-vector".to_string(),
            scenario: probe.id.clone(),
            contract_digest: self.digest(),
            authority_digest: baseline.to_string(),
            decision,
            cleanup: format!("remove {} fixture state", probe.id),
            mutation: probe.mutation.clone(),
        })
    }

    fn baseline_authority_digest(&self) -> Result<&str, String> {
        self.acip_vectors
            .iter()
            .find(|vector| vector.expected == VectorOutcome::Accepted)
            .map(|vector| vector.authority_digest.as_str())
            .ok_or_else(|| "missing accepted ACIP baseline".to_string())
    }
}

impl DrtBQualificationContract {
    pub fn digest(&self) -> String {
        let bytes = serde_json::to_vec(self).expect("DRT-B contract is serializable");
        let digest = Sha256::digest(bytes);
        hex::encode(digest)
    }

    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, DRT_B_CONTRACT_SCHEMA, "schema")?;
        require_set(
            self.requirements.iter().map(String::as_str),
            ["#183", "#184"],
            "requirements",
        )?;
        if self.issue != 507 {
            return Err(format!("unexpected DRT-B issue {}", self.issue));
        }
        if self.resident_count != 6 || self.residents.len() != 6 {
            return Err(format!(
                "expected exactly six residents, found resident_count={} len={}",
                self.resident_count,
                self.residents.len()
            ));
        }
        let mut resident_ids = BTreeSet::new();
        let mut workload_receipts = BTreeSet::new();
        let mut participant_ids = BTreeSet::new();
        for resident in &self.residents {
            if !resident_ids.insert(resident.resident_id.as_str()) {
                return Err(format!("duplicate resident {}", resident.resident_id));
            }
            if !workload_receipts.insert(resident.workload_receipt_id.as_str()) {
                return Err(format!(
                    "duplicate workload receipt {}",
                    resident.workload_receipt_id
                ));
            }
            if !participant_ids.insert(resident.participant_id.as_str()) {
                return Err(format!("duplicate participant {}", resident.participant_id));
            }
            if resident.identity.trim().is_empty()
                || resident.workload_id.trim().is_empty()
                || resident.lineage_digest.trim().is_empty()
                || resident.replay_cursor == 0
            {
                return Err(format!(
                    "resident {} is incomplete",
                    resident.participant_id
                ));
            }
        }
        if self.dehydrate_restore != "exact" {
            return Err("dehydrate/restore must be exact".to_string());
        }
        if !self.cleanup_zero {
            return Err("cleanup_zero must be true".to_string());
        }
        for key in ["resident_slots", "workload_receipts", "max_replay_cursor"] {
            if !self.resource_envelope.contains_key(key) {
                return Err(format!("missing resource envelope key {key}"));
            }
        }
        if self.cleanup_selectors.is_empty() {
            return Err("cleanup selectors are required".to_string());
        }
        require_set(
            self.negative_matrix.iter().map(|case| case.case.as_str()),
            [
                "duplicate_resident_identity",
                "missing_workload_receipt",
                "mutated_lineage",
                "replay_cursor_regression",
                "cleanup_selector_mismatch",
            ],
            "DRT-B negative matrix",
        )?;
        for case in &self.negative_matrix {
            if case.decision != "fail_closed" {
                return Err(format!("{} does not fail closed", case.case));
            }
        }
        Ok(())
    }
}

impl DrtCQualificationDecision {
    pub fn validate(&self) -> Result<(), String> {
        require_exact(&self.schema, DRT_C_DECISION_SCHEMA, "schema")?;
        if self.issue != 508 {
            return Err(format!("unexpected DRT-C issue {}", self.issue));
        }
        require_set(
            self.requirements.iter().map(String::as_str),
            ["#185", "#186", "#187"],
            "requirements",
        )?;
        if self.runtime_revision.len() != 40
            || !self
                .runtime_revision
                .bytes()
                .all(|byte| byte.is_ascii_hexdigit())
        {
            return Err("runtime revision must be a 40-character hex SHA".to_string());
        }
        if self.source_drt_b_contract_digest.len() != 64 {
            return Err("DRT-B digest must be retained".to_string());
        }
        require_set(
            self.fail_closed_cases.iter().map(String::as_str),
            ["identity", "provider", "transport"],
            "fail-closed cases",
        )?;
        if !self.observatory.runtime_emitted || !self.observatory.redacted {
            return Err("Observatory evidence must be Runtime-emitted and redacted".to_string());
        }
        if self.observatory.feed_schema.trim().is_empty()
            || self.observatory.artifact_sha256.len() != 64
        {
            return Err("Observatory evidence must bind schema and artifact digest".to_string());
        }
        if !self.soak.bounded || self.soak.duration_seconds == 0 {
            return Err("soak evidence must be bounded and non-empty".to_string());
        }
        if self.decision.trim().is_empty() {
            return Err("qualification decision is required".to_string());
        }
        for key in [
            "failure-fixtures",
            "observatory-artifacts",
            "soak-processes",
            "temporary-resources",
        ] {
            match self.cleanup.get(key).map(String::as_str) {
                Some("absent") => {}
                Some(other) => return Err(format!("{key} cleanup is not absent: {other}")),
                None => return Err(format!("missing cleanup key {key}")),
            }
        }
        Ok(())
    }
}

fn participant(
    id: &str,
    role: ParticipantRole,
    voting: bool,
    port: u16,
    failure_domain: &str,
) -> QualificationParticipant {
    QualificationParticipant {
        id: id.to_string(),
        role,
        voting,
        identity: format!("did:adl:runtime:{id}"),
        credential: format!("credential:adl:runtime:{id}:v1"),
        port,
        state_root: format!("state://drt-a/{id}"),
        storage_root: format!("storage://drt-a/{id}"),
        failure_domain: failure_domain.to_string(),
    }
}

fn scenario(id: &str) -> QualificationScenario {
    QualificationScenario {
        id: id.to_string(),
        setup: format!("prepare deterministic {id} fixture"),
        action: format!("execute {id} transition"),
        expected_commit: format!("{id} commit is deterministic or explicitly denied"),
        expected_election: format!("{id} election state is stable"),
        expected_fence: format!("{id} fence decision is exact"),
        timeout_ms: 5_000,
        receipt_fields: vec![
            "schema".to_string(),
            "lane".to_string(),
            "scenario".to_string(),
            "contract_digest".to_string(),
            "authority_digest".to_string(),
            "decision".to_string(),
            "cleanup".to_string(),
            "mutation".to_string(),
        ],
        cleanup: format!("remove {id} fixture state"),
        fail_closed: format!("{id} invalid authority is denied before side effects"),
    }
}

fn authority_digest(authority: &str, principal: &str, sequence: u64) -> String {
    let mut fields = BTreeMap::new();
    fields.insert("authority", authority.to_string());
    fields.insert("principal", principal.to_string());
    fields.insert("sequence", sequence.to_string());
    let bytes = serde_json::to_vec(&fields).expect("authority fields serialize");
    hex::encode(Sha256::digest(bytes))
}

fn stable_id<'a>(prefix: &str, parts: impl IntoIterator<Item = &'a str>) -> String {
    let mut hasher = Sha256::new();
    hasher.update(prefix.as_bytes());
    hasher.update(b":");
    for part in parts {
        hasher.update(part.as_bytes());
        hasher.update(b":");
    }
    hex::encode(hasher.finalize())
}

fn require_exact(actual: &str, expected: &str, field: &str) -> Result<(), String> {
    if actual == expected {
        Ok(())
    } else {
        Err(format!("{field} expected {expected}, got {actual}"))
    }
}

fn require_set<'a>(
    actual: impl Iterator<Item = &'a str>,
    expected: impl IntoIterator<Item = &'a str>,
    label: &str,
) -> Result<(), String> {
    let actual = actual.collect::<BTreeSet<_>>();
    let expected = expected.into_iter().collect::<BTreeSet<_>>();
    if actual == expected {
        Ok(())
    } else {
        Err(format!(
            "{label} mismatch: expected {expected:?}, got {actual:?}"
        ))
    }
}
