use serde::{Deserialize, Serialize};

pub const CORPUS_SCHEMA: &str = "adl.characterization.corpus.v1";
pub const OBSERVATION_SCHEMA: &str = "adl.characterization.observation.v1";
pub const NORMALIZED_SCHEMA: &str = "adl.characterization.normalized.v1";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Corpus {
    pub schema: String,
    pub incumbent_revision: String,
    pub binary_sha256: String,
    pub repetitions: u32,
    pub schema_path: String,
    pub required_behaviors: Vec<String>,
    pub cases: Vec<Case>,
    #[serde(default)]
    pub equivalence_groups: Vec<ComparisonGroup>,
    #[serde(default)]
    pub difference_groups: Vec<ComparisonGroup>,
    pub coverage: Vec<CoverageEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub id: String,
    pub behaviors: Vec<String>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub normalization: Vec<NormalizationRule>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct Step {
    pub id: String,
    pub args: Vec<String>,
    pub expected_exit: i32,
    #[serde(default)]
    pub stdout_contains: Vec<String>,
    #[serde(default)]
    pub stderr_contains: Vec<String>,
    #[serde(default)]
    pub pre_actions: Vec<PreAction>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum PreAction {
    FixedEd25519Keypair {
        private_path: String,
        public_path: String,
        seed_byte: u8,
    },
    ReplaceText {
        path: String,
        from: String,
        to: String,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Stream {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum NormalizationRule {
    CanonicalJson {
        step: String,
        stream: Stream,
    },
    ReplaceRoot {
        step: String,
        stream: Stream,
    },
    ReplaceWorkdir {
        step: String,
        stream: Stream,
    },
    ReplaceJsonFields {
        step: String,
        stream: Stream,
        fields: Vec<String>,
    },
    RemoveExactLine {
        step: String,
        stream: Stream,
        line: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct ComparisonGroup {
    pub id: String,
    pub cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CoverageEntry {
    pub behavior: String,
    pub cases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct RawObservation {
    pub schema: String,
    pub case_id: String,
    pub repetition: u32,
    pub incumbent_revision: String,
    pub binary_sha256: String,
    pub corpus_root: String,
    pub workdir: String,
    pub commands: Vec<CommandObservation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct CommandObservation {
    pub step_id: String,
    pub declared_args: Vec<String>,
    pub expanded_args: Vec<String>,
    pub exit_code: i32,
    pub stdout: String,
    pub stderr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct NormalizedObservation {
    pub schema: String,
    pub case_id: String,
    pub repetition: u32,
    pub incumbent_revision: String,
    pub binary_sha256: String,
    pub commands: Vec<CommandObservation>,
}
