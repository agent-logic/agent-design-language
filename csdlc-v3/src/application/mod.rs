use crate::repository::RepositoryContext;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;

/// Retained predecessor denominator for the V3-B foundation slice.
pub const FOUNDATION_PREDECESSORS: [u64; 4] = [164, 165, 166, 167];

/// Operator target for getting a single issue into an executable state.
pub const ISSUE_START_MINUTES_MAX: u64 = 3;

/// Source-grounded behavior retained from a predecessor issue.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequirementProof {
    pub issue: u64,
    pub title: &'static str,
    pub source_scope: &'static str,
    pub foundation_behavior: &'static str,
}

pub const REQUIREMENT_PROOFS: [RequirementProof; 4] = [
    RequirementProof {
        issue: 164,
        title: "[v0.92.1][V3-03] Build The Single-Binary Foundation",
        source_scope: "root parser, dispatch, schemas, completion, generated help, output mode selection, typed top-level errors, and version provenance",
        foundation_behavior: "read-only csdlc-v3-foundation command requires explicit --repo-root and emits stable machine-readable schema csdlc.v3.foundation.v1",
    },
    RequirementProof {
        issue: 165,
        title: "[v0.92.1][V3-04] Implement Application Context And Shared Services",
        source_scope: "invocation-scoped dependency container, common I/O, configuration, typed errors, cancellation, observability, redaction, operation IDs, and test constructors",
        foundation_behavior: "FoundationState::load accepts explicit RepositoryContext data and returns typed errors without hidden lifecycle services or ambient authority",
    },
    RequirementProof {
        issue: 166,
        title: "[v0.92.1][V3-05] Implement Repository Context And Read-Only V2 Import",
        source_scope: "root discovery, canonical repository identity, issue selection precedence, symlink-safe paths, read-only v2 record/card parsing, unsupported-field reporting, and normalized read-only projections",
        foundation_behavior: "RepositoryContext::discover canonicalizes an explicit root, verifies required v2/v3 contract files, and exposes normalized read-only projection paths",
    },
    RequirementProof {
        issue: 167,
        title: "[v0.92.1][V3-06] Implement Canonical State And Card Projections",
        source_scope: "state.json, typed audit events, schema evolution, canonical serialization, card AST values, digest rules, projection manifests, and drift detection",
        foundation_behavior: "FoundationState::projections replays a deterministic BTreeMap-backed projection set and renders byte-stable machine JSON",
    },
];

/// Read-only C-SDLC v3 foundation state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FoundationState {
    repository_root: String,
    operational_authority: String,
    contract_path: String,
    predecessor_coverage_path: String,
    proportional_lifecycle_path: String,
    foundation_predecessors: Vec<u64>,
    issue_start_minutes_max: u64,
    requirement_proofs: Vec<RequirementProof>,
}

impl FoundationState {
    pub fn load(context: &RepositoryContext) -> Result<Self, FoundationError> {
        let contract = fs::read_to_string(context.contract_path()).map_err(|source| {
            FoundationError::ReadFailed {
                label: "v3 contract",
                source,
            }
        })?;
        let coverage =
            fs::read_to_string(context.predecessor_coverage_path()).map_err(|source| {
                FoundationError::ReadFailed {
                    label: "predecessor coverage",
                    source,
                }
            })?;
        let lifecycle =
            fs::read_to_string(context.proportional_lifecycle_path()).map_err(|source| {
                FoundationError::ReadFailed {
                    label: "proportional lifecycle",
                    source,
                }
            })?;
        require_contains(
            &contract,
            "v2 remains the sole operational authority",
            "contract authority boundary",
        )?;
        require_contains(
            &coverage,
            "\"denominator\": [161, 162, 163]",
            "V3-A denominator",
        )?;
        require_contains(
            &lifecycle,
            "\"three_issue_ready_minutes_max\": 3",
            "issue-start simplification budget",
        )?;
        Ok(Self {
            repository_root: context.root().to_string_lossy().into_owned(),
            operational_authority: crate::operational_authority().to_owned(),
            contract_path: context.relative_display(context.contract_path()),
            predecessor_coverage_path: context
                .relative_display(context.predecessor_coverage_path()),
            proportional_lifecycle_path: context
                .relative_display(context.proportional_lifecycle_path()),
            foundation_predecessors: FOUNDATION_PREDECESSORS.to_vec(),
            issue_start_minutes_max: ISSUE_START_MINUTES_MAX,
            requirement_proofs: REQUIREMENT_PROOFS.to_vec(),
        })
    }

    pub fn repository_root(&self) -> &str {
        &self.repository_root
    }

    pub fn operational_authority(&self) -> &str {
        &self.operational_authority
    }

    pub fn foundation_predecessors(&self) -> &[u64] {
        &self.foundation_predecessors
    }

    pub fn issue_start_minutes_max(&self) -> u64 {
        self.issue_start_minutes_max
    }

    pub fn requirement_proofs(&self) -> &[RequirementProof] {
        &self.requirement_proofs
    }

    /// Return projections in deterministic key order.
    pub fn projections(&self) -> Vec<Projection> {
        let mut values = BTreeMap::new();
        values.insert("contract_path", self.contract_path.clone());
        values.insert(
            "foundation_predecessors",
            format_u64_array(&self.foundation_predecessors),
        );
        values.insert(
            "issue_start_minutes_max",
            self.issue_start_minutes_max.to_string(),
        );
        values.insert("operational_authority", self.operational_authority.clone());
        values.insert(
            "predecessor_coverage_path",
            self.predecessor_coverage_path.clone(),
        );
        values.insert(
            "proportional_lifecycle_path",
            self.proportional_lifecycle_path.clone(),
        );
        values.insert(
            "requirement_proofs",
            format_requirement_proofs(&self.requirement_proofs),
        );
        values.insert("repository_root", self.repository_root.clone());
        values
            .into_iter()
            .map(|(key, value)| Projection {
                key: key.to_owned(),
                value,
            })
            .collect()
    }

    pub fn to_machine_json(&self) -> String {
        let projections = self
            .projections()
            .into_iter()
            .map(|projection| {
                format!(
                    "{{\"key\":\"{}\",\"value\":\"{}\"}}",
                    escape_json(&projection.key),
                    escape_json(&projection.value)
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        format!(
            "{{\"schema\":\"csdlc.v3.foundation.v1\",\"read_only\":true,\"operational_authority\":\"{}\",\"projection_count\":{},\"projections\":[{}]}}",
            escape_json(&self.operational_authority),
            self.projections().len(),
            projections
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IssueProjection {
    pub issue: u64,
    pub record_contains_phase: bool,
    pub card_count: usize,
    pub cards: Vec<Projection>,
}

impl IssueProjection {
    pub fn load(context: &RepositoryContext, issue: u64) -> Result<Self, FoundationError> {
        let record = context
            .issue_record_text(issue)
            .map_err(FoundationError::Repository)?;
        let mut cards = BTreeMap::new();
        for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            let text = context
                .card_text(issue, card)
                .map_err(FoundationError::Repository)?;
            cards.insert(card.to_owned(), text.len().to_string());
        }
        Ok(Self {
            issue,
            record_contains_phase: record.contains("\"phase\""),
            card_count: cards.len(),
            cards: cards
                .into_iter()
                .map(|(key, value)| Projection { key, value })
                .collect(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Projection {
    pub key: String,
    pub value: String,
}

#[derive(Debug)]
pub enum FoundationError {
    ReadFailed {
        label: &'static str,
        source: std::io::Error,
    },
    MissingRequiredText {
        label: &'static str,
        needle: &'static str,
    },
    Repository(crate::repository::RepositoryContextError),
}

impl fmt::Display for FoundationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ReadFailed { label, source } => {
                write!(formatter, "failed to read {label}: {source}")
            }
            Self::MissingRequiredText { label, needle } => {
                write!(formatter, "{label} is missing required text {needle:?}")
            }
            Self::Repository(source) => write!(formatter, "{source}"),
        }
    }
}

impl std::error::Error for FoundationError {}

fn require_contains(
    haystack: &str,
    needle: &'static str,
    label: &'static str,
) -> Result<(), FoundationError> {
    if haystack.contains(needle) {
        Ok(())
    } else {
        Err(FoundationError::MissingRequiredText { label, needle })
    }
}

fn format_u64_array(values: &[u64]) -> String {
    let body = values
        .iter()
        .map(u64::to_string)
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn format_requirement_proofs(proofs: &[RequirementProof]) -> String {
    let body = proofs
        .iter()
        .map(|proof| {
            format!(
                "{}:{}=>{}",
                proof.issue, proof.source_scope, proof.foundation_behavior
            )
        })
        .collect::<Vec<_>>()
        .join("|");
    format!("[{body}]")
}

fn escape_json(value: &str) -> String {
    let mut escaped = String::new();
    for character in value.chars() {
        match character {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            value if value.is_control() => escaped.push_str(&format!("\\u{:04x}", value as u32)),
            value => escaped.push(value),
        }
    }
    escaped
}
