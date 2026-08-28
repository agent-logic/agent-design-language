use crate::repository::RepositoryContext;
use std::collections::BTreeMap;
use std::fmt;
use std::fs;

/// Retained predecessor denominator for the V3-B foundation slice.
pub const FOUNDATION_PREDECESSORS: [u64; 4] = [164, 165, 166, 167];

/// Operator target for getting a single issue into an executable state.
pub const ISSUE_START_MINUTES_MAX: u64 = 3;

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
