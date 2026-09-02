use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub const DRT_A_CONTRACT_SCHEMA: &str = "adl.runtime.qualification.drt_a_contract.v1";
pub const DRT_A_RECEIPT_SCHEMA: &str = "adl.runtime.qualification.drt_a_receipt.v1";

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
