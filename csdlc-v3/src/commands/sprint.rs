//! Non-authoritative sprint-readiness verifier for pre-cutover v3 trials.
//!
//! This module consumes typed v2 GitHub issue readback artifacts and produces a
//! v3 sprint readiness classification. It never reads credentials, mutates
//! GitHub, advances lifecycle cards, binds worktrees, publishes PRs, finishes
//! issues, performs cleanup, or grants v3 operational authority.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Component, Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintReadinessRequest {
    pub repository: String,
    pub version: String,
    pub sprints: Vec<SprintReadinessTarget>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintReadinessTarget {
    pub sprint: u64,
    pub umbrella_issue: u64,
    pub title: String,
    pub execution_mode: SprintExecutionMode,
    pub serial_gates: Vec<String>,
    pub umbrella_readback_ref: String,
    pub child_readback_refs: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SprintExecutionMode {
    Sequential,
    Parallel,
    Hybrid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintReadinessReport {
    pub schema: &'static str,
    pub repository: String,
    pub version: String,
    pub read_only: bool,
    pub operational_authority: bool,
    pub status: SprintReadinessStatus,
    pub sprints: Vec<SprintReadiness>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SprintReadinessStatus {
    Ready,
    CompleteNotCutoverAuthority,
    Blocked,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintReadiness {
    pub sprint: u64,
    pub umbrella_issue: u64,
    pub title: String,
    pub execution_mode: SprintExecutionMode,
    pub membership_version: Option<u64>,
    pub declared_children: Vec<u64>,
    pub umbrella_state: SprintUmbrellaState,
    pub child_states: Vec<SprintChildState>,
    pub status: SprintReadinessStatus,
    pub findings: Vec<SprintReadinessFinding>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintUmbrellaState {
    pub issue: u64,
    pub title: String,
    pub state: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintChildState {
    pub issue: u64,
    pub title: String,
    pub state: String,
    pub closed_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SprintReadinessFinding {
    pub severity: SprintFindingSeverity,
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SprintFindingSeverity {
    Info,
    Blocking,
    Invalid,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SprintReadinessError {
    RequestInvalidJson(String),
    EvidenceRefEscapesRepo { field: &'static str, value: String },
    EvidenceRefMissing { field: &'static str, value: String },
    EvidenceInvalidJson { field: &'static str, value: String },
}

pub fn verify_sprint_readiness(
    repo_root: &Path,
    request: SprintReadinessRequest,
) -> Result<SprintReadinessReport, SprintReadinessError> {
    let repository = request.repository.clone();
    let version = request.version.clone();
    let mut sprints = Vec::new();
    for target in request.sprints {
        sprints.push(verify_one_sprint(repo_root, &repository, target)?);
    }
    let status = if sprints.iter().any(|sprint| {
        sprint
            .findings
            .iter()
            .any(|finding| finding.severity == SprintFindingSeverity::Invalid)
    }) {
        SprintReadinessStatus::Invalid
    } else if sprints
        .iter()
        .any(|sprint| sprint.status == SprintReadinessStatus::Blocked)
    {
        SprintReadinessStatus::Blocked
    } else if sprints
        .iter()
        .all(|sprint| sprint.status == SprintReadinessStatus::CompleteNotCutoverAuthority)
    {
        SprintReadinessStatus::CompleteNotCutoverAuthority
    } else {
        SprintReadinessStatus::Ready
    };
    Ok(SprintReadinessReport {
        schema: "csdlc.v3.sprint_readiness.v1",
        repository,
        version,
        read_only: true,
        operational_authority: false,
        status,
        sprints,
    })
}

pub fn parse_request(bytes: &[u8]) -> Result<SprintReadinessRequest, SprintReadinessError> {
    serde_json::from_slice(bytes)
        .map_err(|error| SprintReadinessError::RequestInvalidJson(error.to_string()))
}

fn verify_one_sprint(
    repo_root: &Path,
    repository: &str,
    target: SprintReadinessTarget,
) -> Result<SprintReadiness, SprintReadinessError> {
    let mut findings = Vec::new();
    if target.serial_gates.is_empty() {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "serial_gates_missing",
            "sprint readiness requires explicit serial gates, even for parallel-capable lanes",
        ));
    }
    let umbrella = read_issue_readback(
        repo_root,
        "umbrella_readback_ref",
        &target.umbrella_readback_ref,
    )?;
    if umbrella.repository != repository || umbrella.number != target.umbrella_issue {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "umbrella_identity_mismatch",
            "umbrella readback does not match the requested repository and issue",
        ));
    }
    if umbrella.state != "open" && umbrella.state != "closed" {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "umbrella_state_unknown",
            "umbrella readbacks must use open or closed GitHub state",
        ));
    }
    if !umbrella.title.contains(&target.title) {
        findings.push(finding(
            SprintFindingSeverity::Info,
            "umbrella_title_differs",
            "umbrella title does not contain the requested sprint title",
        ));
    }
    let declared_children = parse_child_membership(&umbrella.body);
    if declared_children.is_empty() {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "membership_missing",
            "sprint umbrella body must declare child issue membership",
        ));
    }
    let membership_version = parse_membership_version(&umbrella.body);
    if membership_version.is_none() {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "membership_version_missing",
            "sprint umbrella body must declare a membership version",
        ));
    }
    let requested_children = target
        .child_readback_refs
        .keys()
        .filter_map(|key| key.parse::<u64>().ok())
        .collect::<BTreeSet<_>>();
    let declared = declared_children.iter().copied().collect::<BTreeSet<_>>();
    if requested_children != declared {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "child_readback_denominator_mismatch",
            "child readback refs must exactly match the umbrella-declared membership",
        ));
    }
    let mut child_states = Vec::new();
    for child in &declared_children {
        let key = child.to_string();
        let Some(path) = target.child_readback_refs.get(&key) else {
            findings.push(finding(
                SprintFindingSeverity::Invalid,
                "child_readback_missing",
                &format!("missing child readback for #{child}"),
            ));
            continue;
        };
        let readback = read_issue_readback(repo_root, "child_readback_ref", path)?;
        if readback.repository != repository || readback.number != *child {
            findings.push(finding(
                SprintFindingSeverity::Invalid,
                "child_identity_mismatch",
                &format!("child readback does not match #{child}"),
            ));
            continue;
        }
        child_states.push(SprintChildState {
            issue: readback.number,
            title: readback.title,
            state: readback.state,
            closed_at: readback.closed_at,
        });
    }
    if child_states
        .iter()
        .any(|child| child.state != "open" && child.state != "closed")
    {
        findings.push(finding(
            SprintFindingSeverity::Invalid,
            "child_state_unknown",
            "child readbacks must use open or closed GitHub state",
        ));
    }
    if child_states.iter().any(|child| child.state == "open") {
        findings.push(finding(
            SprintFindingSeverity::Info,
            "open_children_ready_for_execution_planning",
            "open children remain in the sprint readiness denominator",
        ));
    }
    let terminal_sprint = umbrella.state == "closed"
        && !child_states.is_empty()
        && child_states.iter().all(|child| child.state == "closed");
    if umbrella.state == "closed" && !terminal_sprint {
        findings.push(finding(
            SprintFindingSeverity::Blocking,
            "umbrella_closed_before_children_terminal",
            "closed sprint umbrellas require every declared child readback to be closed",
        ));
    }
    let status = if findings
        .iter()
        .any(|finding| finding.severity == SprintFindingSeverity::Invalid)
    {
        SprintReadinessStatus::Invalid
    } else if findings
        .iter()
        .any(|finding| finding.severity == SprintFindingSeverity::Blocking)
    {
        SprintReadinessStatus::Blocked
    } else if terminal_sprint {
        SprintReadinessStatus::CompleteNotCutoverAuthority
    } else {
        SprintReadinessStatus::Ready
    };
    Ok(SprintReadiness {
        sprint: target.sprint,
        umbrella_issue: target.umbrella_issue,
        title: target.title,
        execution_mode: target.execution_mode,
        membership_version,
        declared_children,
        umbrella_state: SprintUmbrellaState {
            issue: umbrella.number,
            title: umbrella.title,
            state: umbrella.state,
            closed_at: umbrella.closed_at,
        },
        child_states,
        status,
        findings,
    })
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct IssueReadback {
    repository: String,
    number: u64,
    title: String,
    body: String,
    state: String,
    closed_at: Option<String>,
}

fn read_issue_readback(
    repo_root: &Path,
    field: &'static str,
    value: &str,
) -> Result<IssueReadback, SprintReadinessError> {
    let path = repo_local_path(repo_root, field, value)?;
    if !path.is_file() {
        return Err(SprintReadinessError::EvidenceRefMissing {
            field,
            value: value.to_owned(),
        });
    }
    let bytes = fs::read(&path).map_err(|_| SprintReadinessError::EvidenceRefMissing {
        field,
        value: value.to_owned(),
    })?;
    let value_json: Value =
        serde_json::from_slice(&bytes).map_err(|_| SprintReadinessError::EvidenceInvalidJson {
            field,
            value: value.to_owned(),
        })?;
    let issue = value_json
        .get("issue")
        .and_then(Value::as_object)
        .ok_or_else(|| SprintReadinessError::EvidenceInvalidJson {
            field,
            value: value.to_owned(),
        })?;
    Ok(IssueReadback {
        repository: string_field(issue, "repository").to_owned(),
        number: number_field(issue, "number"),
        title: string_field(issue, "title").to_owned(),
        body: string_field(issue, "body").to_owned(),
        state: string_field(issue, "state").to_owned(),
        closed_at: issue
            .get("closed_at")
            .and_then(Value::as_str)
            .map(str::to_owned),
    })
}

fn repo_local_path(
    repo_root: &Path,
    field: &'static str,
    value: &str,
) -> Result<PathBuf, SprintReadinessError> {
    let path = Path::new(value);
    if path.is_absolute()
        || path
            .components()
            .any(|component| matches!(component, Component::ParentDir))
        || value.trim().is_empty()
    {
        return Err(SprintReadinessError::EvidenceRefEscapesRepo {
            field,
            value: value.to_owned(),
        });
    }
    Ok(repo_root.join(path))
}

fn string_field<'a>(value: &'a serde_json::Map<String, Value>, field: &str) -> &'a str {
    value.get(field).and_then(Value::as_str).unwrap_or_default()
}

fn number_field(value: &serde_json::Map<String, Value>, field: &str) -> u64 {
    value.get(field).and_then(Value::as_u64).unwrap_or_default()
}

fn parse_child_membership(body: &str) -> Vec<u64> {
    let mut children = Vec::new();
    let mut in_membership = false;
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("## ") {
            in_membership = trimmed.contains("child membership")
                || trimmed.contains("Initial child membership")
                || trimmed.contains("Exact child membership")
                || trimmed == "## Child issues";
            continue;
        }
        if !in_membership {
            continue;
        }
        let issue_ref = if let Some(rest) = trimmed.strip_prefix("- #") {
            rest
        } else if let Some((_, rest)) = trimmed.split_once(". #") {
            rest
        } else {
            continue;
        };
        let number = issue_ref
            .chars()
            .take_while(|value| value.is_ascii_digit())
            .collect::<String>();
        if let Ok(issue) = number.parse::<u64>() {
            children.push(issue);
        }
    }
    children
}

fn parse_membership_version(body: &str) -> Option<u64> {
    body.lines().find_map(|line| {
        let trimmed = line.trim();
        if !trimmed.starts_with("- Membership version:") {
            return None;
        }
        trimmed
            .split('`')
            .nth(1)
            .and_then(|value| value.parse::<u64>().ok())
    })
}

fn finding(severity: SprintFindingSeverity, code: &str, message: &str) -> SprintReadinessFinding {
    SprintReadinessFinding {
        severity,
        code: code.to_owned(),
        message: message.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn fixture_dir(name: &str) -> PathBuf {
        let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/sprint-readiness-fixtures")
            .join(name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).expect("fixture dir");
        dir
    }

    fn write_issue(dir: &Path, name: &str, number: u64, title: &str, body: &str, state: &str) {
        fs::write(
            dir.join(name),
            serde_json::json!({
                "schema": "csdlc.github_action_result.v1",
                "action": "issue_read",
                "reconciled": true,
                "repository": "agent-logic/agent-design-language",
                "issue": {
                    "schema": "csdlc.github_issue.v1",
                    "repository": "agent-logic/agent-design-language",
                    "number": number,
                    "title": title,
                    "body": body,
                    "state": state,
                    "closed_at": null
                }
            })
            .to_string(),
        )
        .expect("issue readback");
    }

    #[test]
    fn sprint_readiness_parses_live_membership_style() {
        let dir = fixture_dir("happy");
        let body = "## Outcome\n\nCoordinate Sprint 8.\n\n## Initial child membership baseline\n\n- #51\n- #511\n\n- Membership version: `4`\n";
        write_issue(
            &dir,
            "umbrella.json",
            536,
            "[Sprint 8] Product lanes",
            body,
            "open",
        );
        write_issue(&dir, "51.json", 51, "Podcast", "body", "closed");
        write_issue(&dir, "511.json", 511, "OBS-A", "body", "open");
        let request = SprintReadinessRequest {
            repository: "agent-logic/agent-design-language".into(),
            version: "v0.92.1".into(),
            sprints: vec![SprintReadinessTarget {
                sprint: 8,
                umbrella_issue: 536,
                title: "Sprint 8".into(),
                execution_mode: SprintExecutionMode::Hybrid,
                serial_gates: vec!["OBS-B waits for OBS-A".into()],
                umbrella_readback_ref: "umbrella.json".into(),
                child_readback_refs: BTreeMap::from([
                    ("51".into(), "51.json".into()),
                    ("511".into(), "511.json".into()),
                ]),
            }],
        };
        let report = verify_sprint_readiness(&dir, request).expect("report");
        assert_eq!(report.status, SprintReadinessStatus::Ready);
        assert_eq!(report.sprints[0].membership_version, Some(4));
        assert_eq!(report.sprints[0].declared_children, vec![51, 511]);
        assert!(report.sprints[0]
            .findings
            .iter()
            .any(|finding| finding.code == "open_children_ready_for_execution_planning"));
        assert!(!report.operational_authority);
    }

    #[test]
    fn sprint_readiness_rejects_missing_child_readback() {
        let dir = fixture_dir("missing-child");
        let body = "## Exact child membership\n\n- #515\n- #516\n\n- Membership version: `4`\n";
        write_issue(
            &dir,
            "umbrella.json",
            537,
            "[Sprint 9] Provider",
            body,
            "open",
        );
        write_issue(&dir, "515.json", 515, "PROV-A", "body", "open");
        let request = SprintReadinessRequest {
            repository: "agent-logic/agent-design-language".into(),
            version: "v0.92.1".into(),
            sprints: vec![SprintReadinessTarget {
                sprint: 9,
                umbrella_issue: 537,
                title: "Sprint 9".into(),
                execution_mode: SprintExecutionMode::Sequential,
                serial_gates: vec!["PROV-B follows PROV-A".into()],
                umbrella_readback_ref: "umbrella.json".into(),
                child_readback_refs: BTreeMap::from([("515".into(), "515.json".into())]),
            }],
        };
        let report = verify_sprint_readiness(&dir, request).expect("report");
        assert_eq!(report.status, SprintReadinessStatus::Invalid);
        assert!(report.sprints[0]
            .findings
            .iter()
            .any(|finding| finding.code == "child_readback_denominator_mismatch"));
    }

    #[test]
    fn sprint_readiness_parses_v3_h_child_issue_heading() {
        let dir = fixture_dir("v3-h-child-issues");
        let body = "## Outcome\n\nSet up V3-H.\n\n## Child issues\n\n1. #627 -- denominator.\n2. #628 -- local lifecycle.\n3. #629 -- GitHub routes.\n\n- Membership version: `1`\n";
        write_issue(
            &dir,
            "umbrella.json",
            625,
            "[v0.92.1][V3-H] C-SDLC v3 full command replacement sprint",
            body,
            "open",
        );
        write_issue(&dir, "627.json", 627, "V3-H.1", "body", "closed");
        write_issue(&dir, "628.json", 628, "V3-H.2", "body", "closed");
        write_issue(&dir, "629.json", 629, "V3-H.3", "body", "open");
        let request = SprintReadinessRequest {
            repository: "agent-logic/agent-design-language".into(),
            version: "v0.92.1".into(),
            sprints: vec![SprintReadinessTarget {
                sprint: 6,
                umbrella_issue: 625,
                title: "V3-H".into(),
                execution_mode: SprintExecutionMode::Hybrid,
                serial_gates: vec!["#629 consumes #628".into()],
                umbrella_readback_ref: "umbrella.json".into(),
                child_readback_refs: BTreeMap::from([
                    ("627".into(), "627.json".into()),
                    ("628".into(), "628.json".into()),
                    ("629".into(), "629.json".into()),
                ]),
            }],
        };
        let report = verify_sprint_readiness(&dir, request).expect("report");
        assert_eq!(report.sprints[0].declared_children, vec![627, 628, 629]);
        assert_eq!(report.sprints[0].membership_version, Some(1));
        assert_eq!(report.status, SprintReadinessStatus::Ready);
    }

    #[test]
    fn sprint_readiness_reports_terminal_v3_h_without_cutover_authority() {
        let dir = fixture_dir("terminal-v3-h");
        let body = "## Outcome\n\nSet up V3-H.\n\n## Child issues\n\n1. #627 -- denominator.\n2. #628 -- local lifecycle.\n3. #629 -- GitHub routes.\n\n- Membership version: `1`\n";
        write_issue(
            &dir,
            "umbrella.json",
            625,
            "[v0.92.1][V3-H] C-SDLC v3 full command replacement sprint",
            body,
            "closed",
        );
        write_issue(&dir, "627.json", 627, "V3-H.1", "body", "closed");
        write_issue(&dir, "628.json", 628, "V3-H.2", "body", "closed");
        write_issue(&dir, "629.json", 629, "V3-H.3", "body", "closed");
        let request = SprintReadinessRequest {
            repository: "agent-logic/agent-design-language".into(),
            version: "v0.92.1".into(),
            sprints: vec![SprintReadinessTarget {
                sprint: 6,
                umbrella_issue: 625,
                title: "V3-H".into(),
                execution_mode: SprintExecutionMode::Hybrid,
                serial_gates: vec!["#629 consumes #628".into()],
                umbrella_readback_ref: "umbrella.json".into(),
                child_readback_refs: BTreeMap::from([
                    ("627".into(), "627.json".into()),
                    ("628".into(), "628.json".into()),
                    ("629".into(), "629.json".into()),
                ]),
            }],
        };
        let report = verify_sprint_readiness(&dir, request).expect("report");
        assert_eq!(
            report.status,
            SprintReadinessStatus::CompleteNotCutoverAuthority
        );
        assert_eq!(
            report.sprints[0].status,
            SprintReadinessStatus::CompleteNotCutoverAuthority
        );
        assert_eq!(report.sprints[0].umbrella_state.state, "closed");
        assert!(report.sprints[0].findings.is_empty());
        assert!(!report.operational_authority);
    }

    #[test]
    fn sprint_readiness_rejects_escaping_evidence_ref() {
        let dir = fixture_dir("escape");
        let request = SprintReadinessRequest {
            repository: "agent-logic/agent-design-language".into(),
            version: "v0.92.1".into(),
            sprints: vec![SprintReadinessTarget {
                sprint: 8,
                umbrella_issue: 536,
                title: "Sprint 8".into(),
                execution_mode: SprintExecutionMode::Hybrid,
                serial_gates: vec!["gate".into()],
                umbrella_readback_ref: "../outside.json".into(),
                child_readback_refs: BTreeMap::new(),
            }],
        };
        assert!(matches!(
            verify_sprint_readiness(&dir, request),
            Err(SprintReadinessError::EvidenceRefEscapesRepo { .. })
        ));
    }
}
