use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

pub const DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION: &str =
    "dspark_speculative_decoding_evaluation.v1";
pub const DSPARK_SPECULATIVE_DECODING_EVALUATION_PROMPT_VERSION: &str =
    "v0917.provider_sprint.dspark_speculative_decoding.v1";
pub const DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH: &str =
    "docs/milestones/v0.91.7/review/provider/DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json";

#[cfg(test)]
const HOST_PATH_MARKER: &str = "/absolute/host/path/";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsparkSourceRecord {
    pub source_id: &'static str,
    pub title: &'static str,
    pub source_ref: &'static str,
    pub observed_date: &'static str,
    pub relevance: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DsparkCandidateDisposition {
    CandidateForBackendProbe,
    BlockedUntilBackendExists,
    RejectCrossFamilyPairing,
    RouteToLiveGpuSmoke,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsparkCandidateRow {
    pub row_id: &'static str,
    pub target_family: &'static str,
    pub draft_family: &'static str,
    pub proposed_models: Vec<&'static str>,
    pub disposition: DsparkCandidateDisposition,
    pub acceptance_condition: &'static str,
    pub reason: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DsparkEvaluationReport {
    pub schema_version: &'static str,
    pub prompt_version: &'static str,
    pub issue_number: u32,
    pub sprint_issue: u32,
    pub source_records: Vec<DsparkSourceRecord>,
    pub candidate_rows: Vec<DsparkCandidateRow>,
    pub accepted_for_v0917_provider_sprint: bool,
    pub recommendation: &'static str,
    pub required_next_proof: Vec<&'static str>,
    pub authority_rules: Vec<&'static str>,
    pub non_claims: Vec<&'static str>,
    pub validation_commands: Vec<&'static str>,
}

fn source_records() -> Vec<DsparkSourceRecord> {
    vec![
        DsparkSourceRecord {
            source_id: "arxiv_2607_05147",
            title: "DSpark: Confidence-Scheduled Speculative Decoding with Semi-Autoregressive Generation",
            source_ref: "https://arxiv.org/abs/2607.05147",
            observed_date: "2026-07-07",
            relevance: "Defines DSpark's semi-autoregressive draft and confidence-scheduled verification design; supports evaluating ADL only as a backend capability candidate, not as a prompt-level provider feature.",
        },
        DsparkSourceRecord {
            source_id: "adl_v0912_speculative_decoding_prototype",
            title: "ADL speculative decoding deterministic commit-boundary prototype",
            source_ref: "docs/milestones/v0.91.2/review/speculative_decoding/speculative_decoding_prototype_packet.md",
            observed_date: "2026-07-07",
            relevance: "Existing ADL proof requires target-verified token commit, explicit tokenizer mismatch rejection, and no expansion of tool or side-effect authority.",
        },
    ]
}

fn candidate_rows() -> Vec<DsparkCandidateRow> {
    vec![
        DsparkCandidateRow {
            row_id: "qwen_same_family_candidate",
            target_family: "qwen",
            draft_family: "qwen",
            proposed_models: vec!["qwen/qwen3-coder-next", "qwen/qwen3-6-flash"],
            disposition: DsparkCandidateDisposition::BlockedUntilBackendExists,
            acceptance_condition: "A serving backend must expose DSpark-style draft generation, target verification, accepted-token counts, fallback counts, and tokenizer compatibility for the same Qwen family.",
            reason: "Qwen is plausible as a same-family speculative-decoding candidate, but ADL currently has no live DSpark/Qwen draft-verify backend to prove accepted length or throughput.",
        },
        DsparkCandidateRow {
            row_id: "gemma_same_family_candidate",
            target_family: "gemma",
            draft_family: "gemma",
            proposed_models: vec!["google/gemma-4-31b-it", "gemma4:e4b"],
            disposition: DsparkCandidateDisposition::BlockedUntilBackendExists,
            acceptance_condition: "A serving backend must expose DSpark-style draft generation, target verification, accepted-token counts, fallback counts, and tokenizer compatibility for the same Gemma family.",
            reason: "Gemma is plausible as a same-family local or hosted candidate, but existing ADL Gemma evidence covers model usefulness, not DSpark-style speculative acceptance or throughput.",
        },
        DsparkCandidateRow {
            row_id: "qwen_gemma_cross_family_rejected",
            target_family: "qwen_or_gemma",
            draft_family: "gemma_or_qwen",
            proposed_models: vec!["qwen target with gemma draft", "gemma target with qwen draft"],
            disposition: DsparkCandidateDisposition::RejectCrossFamilyPairing,
            acceptance_condition: "None for v0.91.7; cross-family pairings must not be treated as accepted speculative acceleration evidence.",
            reason: "The ADL speculative-decoding prototype treats tokenizer mismatch as non-proving. Cross-family Qwen/Gemma pairings would widen that risk unless a backend proves tokenizer identity and target-verified commit behavior.",
        },
        DsparkCandidateRow {
            row_id: "deepseek_v4_flash_dspark_live_lane",
            target_family: "deepseek-v4",
            draft_family: "dspark",
            proposed_models: vec!["deepseek-v4-flash-dspark"],
            disposition: DsparkCandidateDisposition::RouteToLiveGpuSmoke,
            acceptance_condition: "Issue #4654 must run the bounded ephemeral 2xH100 AWS smoke, record teardown, and retain provider/model outcome evidence before this row can be accepted.",
            reason: "The external DSpark result is specifically tied to the DeepSeek-V4 serving system; ADL should prove that path in #4654 rather than infer it from Qwen/Gemma candidates.",
        },
    ]
}

pub fn run_dspark_speculative_decoding_evaluation() -> DsparkEvaluationReport {
    DsparkEvaluationReport {
        schema_version: DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION,
        prompt_version: DSPARK_SPECULATIVE_DECODING_EVALUATION_PROMPT_VERSION,
        issue_number: 4653,
        sprint_issue: 5027,
        source_records: source_records(),
        candidate_rows: candidate_rows(),
        accepted_for_v0917_provider_sprint: false,
        recommendation: "Do not claim Qwen or Gemma DSpark acceleration as accepted in v0.91.7 from planning evidence alone. Keep Qwen/Gemma as same-family candidates, reject cross-family Qwen/Gemma pairings, and route actual DeepSeek-V4 DSpark live proof to #4654.",
        required_next_proof: vec![
            "A same-family Qwen or Gemma backend must expose draft tokens, target verification, accepted-token counts, fallback counts, tokenizer compatibility, latency, and throughput before ADL can accept the row.",
            "Issue #4654 must prove or truthfully block the deepseek-v4-flash-dspark live GPU smoke with Agent Logic AWS account guard and teardown evidence.",
            "The shared provider proof #5026 must consume only rows that have live or accepted blocked dispositions.",
        ],
        authority_rules: vec![
            "Speculative draft tokens remain provisional until target verification accepts them.",
            "Accepted token counts and throughput claims must come from the backend, not prompt-level model text.",
            "Speculative decoding cannot grant tool, mutation, merge, or side-effect authority.",
            "Cross-family tokenizer mismatch is a fail-closed condition unless the backend proves compatibility.",
        ],
        non_claims: vec![
            "does not prove live Qwen DSpark acceleration",
            "does not prove live Gemma DSpark acceleration",
            "does not prove DeepSeek-V4 DSpark availability on AWS",
            "does not claim broad speculative decoding support in ADL provider routing",
            "does not replace #4654 live GPU smoke or #5026 shared provider acceptance proof",
        ],
        validation_commands: vec![
            "CARGO_INCREMENTAL=0 cargo test --manifest-path adl/Cargo.toml --lib dspark_speculative_decoding_evaluation -- --nocapture",
            "CARGO_INCREMENTAL=0 cargo test --manifest-path adl/Cargo.toml --bin demo_v0917_dspark_speculative_decoding_evaluation -- --nocapture",
            "CARGO_INCREMENTAL=0 cargo run --manifest-path adl/Cargo.toml --bin demo_v0917_dspark_speculative_decoding_evaluation -- docs/milestones/v0.91.7/review/provider/DSPARK_SPECULATIVE_DECODING_EVALUATION_4653.json",
            "git diff --check",
        ],
    }
}

pub fn write_dspark_speculative_decoding_evaluation_report(
    output_path: impl AsRef<Path>,
) -> Result<DsparkEvaluationReport> {
    let report = run_dspark_speculative_decoding_evaluation();
    let output_path = output_path.as_ref();
    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent).with_context(|| {
            format!(
                "create DSpark speculative decoding evaluation parent '{}'",
                parent.display()
            )
        })?;
    }
    let json = serde_json::to_string_pretty(&report)
        .context("serialize DSpark speculative decoding evaluation report")?;
    fs::write(output_path, json).with_context(|| {
        format!(
            "write DSpark speculative decoding evaluation report '{}'",
            output_path.display()
        )
    })?;
    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::{
        run_dspark_speculative_decoding_evaluation,
        write_dspark_speculative_decoding_evaluation_report, DsparkCandidateDisposition,
        DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH,
        DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION, HOST_PATH_MARKER,
    };
    use std::fs;
    use std::path::Path;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn unique_temp_path(prefix: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be valid")
            .as_nanos();
        std::env::temp_dir().join(format!("{prefix}-{nanos}.json"))
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_routes_rows_truthfully() {
        let report = run_dspark_speculative_decoding_evaluation();
        assert!(!report.accepted_for_v0917_provider_sprint);
        let qwen = report
            .candidate_rows
            .iter()
            .find(|row| row.row_id == "qwen_same_family_candidate")
            .expect("qwen row");
        assert_eq!(
            qwen.disposition,
            DsparkCandidateDisposition::BlockedUntilBackendExists
        );
        let cross = report
            .candidate_rows
            .iter()
            .find(|row| row.row_id == "qwen_gemma_cross_family_rejected")
            .expect("cross-family row");
        assert_eq!(
            cross.disposition,
            DsparkCandidateDisposition::RejectCrossFamilyPairing
        );
        let deepseek = report
            .candidate_rows
            .iter()
            .find(|row| row.row_id == "deepseek_v4_flash_dspark_live_lane")
            .expect("deepseek row");
        assert_eq!(
            deepseek.disposition,
            DsparkCandidateDisposition::RouteToLiveGpuSmoke
        );
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_serializes_portably() {
        let first = serde_json::to_string_pretty(&run_dspark_speculative_decoding_evaluation())
            .expect("serialize first report");
        let second = serde_json::to_string_pretty(&run_dspark_speculative_decoding_evaluation())
            .expect("serialize second report");
        assert_eq!(first, second);
        assert!(!first.contains(HOST_PATH_MARKER));
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_writer_emits_expected_json() {
        let path = unique_temp_path("dspark-speculative-decoding-evaluation");
        let report =
            write_dspark_speculative_decoding_evaluation_report(&path).expect("write report");
        let body = fs::read_to_string(&path).expect("read report");
        assert!(body.contains(DSPARK_SPECULATIVE_DECODING_EVALUATION_SCHEMA_VERSION));
        assert_eq!(report.candidate_rows.len(), 4);
        fs::remove_file(&path).expect("remove report");
    }

    #[test]
    fn dspark_speculative_decoding_evaluation_artifact_path_is_repo_relative() {
        assert!(
            !Path::new(DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH).is_absolute()
        );
        assert!(!DSPARK_SPECULATIVE_DECODING_EVALUATION_REPORT_ARTIFACT_PATH.contains(".."));
    }
}
