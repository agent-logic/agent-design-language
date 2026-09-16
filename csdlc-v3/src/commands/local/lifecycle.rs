//! Local lifecycle-state observation and fixture initialization.

use std::{fs, path::Path};

use serde_json::Value;

use super::planning::plan_cards;
use super::storage::lifecycle_digest;
use super::{
    finding, DoctorFinding, LocalLifecycleStateObservation, LocalPreparationRequest, PlanStatus,
    PromptRegistry, REQUIRED_CARD_KINDS,
};

pub fn inspect_local_lifecycle_state(root: &Path, issue: u64) -> LocalLifecycleStateObservation {
    let issue_root = root.join(format!(".csdlc/issues/{issue}"));
    inspect_lifecycle_issue_root(&issue_root, issue, "local")
}

pub fn inspect_v3_local_state(root: &Path, issue: u64) -> LocalLifecycleStateObservation {
    let issue_root = root.join(format!("issues/{issue}"));
    inspect_lifecycle_issue_root(&issue_root, issue, "v3-local")
}

pub fn initialize_v3_local_state(
    root: &Path,
    request: &LocalPreparationRequest,
    registry: &PromptRegistry,
) -> Result<LocalLifecycleStateObservation, Vec<DoctorFinding>> {
    let cards = plan_cards(request.issue, &request.registry_version, registry)?;
    let issue_root = root.join(format!("issues/{}", request.issue));
    let cards_root = issue_root.join("cards");
    fs::create_dir_all(&cards_root).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "v3_local_state_create_failed",
            &format!("could not create v3 local state directory: {error}"),
        )]
    })?;
    let index = serde_json::json!({
        "schema": "csdlc.v3.local_state.v1",
        "issue": request.issue,
        "phase": "ready",
        "repository": request.repository,
        "branch": request.branch,
        "worktree": request.worktree,
        "template_registry_version": registry.version,
        "operational_authority": false
    });
    write_json(&issue_root.join("index.json"), &index)?;
    for card in cards.card_kinds {
        write_json(
            &cards_root.join(format!("{card}.values.json")),
            &serde_json::json!({
                "schema": "csdlc.v3.local_card_values.v1",
                "issue": request.issue,
                "card": card,
                "operational_authority": false
            }),
        )?;
        fs::write(
            cards_root.join(format!("{card}.md")),
            format!(
                "# {card}\n\nNon-authoritative v3 local preparation fixture for issue #{}.\n",
                request.issue
            ),
        )
        .map_err(|error| {
            vec![finding(
                PlanStatus::Failed,
                "v3_local_card_write_failed",
                &format!("could not write v3 local card fixture: {error}"),
            )]
        })?;
    }
    Ok(inspect_v3_local_state(root, request.issue))
}

pub(super) fn inspect_lifecycle_issue_root(
    issue_root: &Path,
    issue: u64,
    source: &str,
) -> LocalLifecycleStateObservation {
    let index_path = issue_root.join("index.json");
    if !index_path.is_file() {
        return LocalLifecycleStateObservation {
            issue,
            phase: None,
            generation: None,
            digest: None,
            status: PlanStatus::Blocked,
            code: "missing_local_lifecycle_state".into(),
            message: format!("{source} lifecycle state is missing; initialize or repair the issue record before executing local routes"),
            cards_present: Vec::new(),
            missing_cards: REQUIRED_CARD_KINDS.into_iter().map(str::to_string).collect(),
            ready_to_execute: false,
        };
    }
    let index = match read_local_lifecycle_index(&index_path) {
        Ok(index) => index,
        Err(message) => {
            return LocalLifecycleStateObservation {
                issue,
                phase: None,
                generation: None,
                digest: None,
                status: PlanStatus::Blocked,
                code: "invalid_local_lifecycle_state".into(),
                message,
                cards_present: Vec::new(),
                missing_cards: REQUIRED_CARD_KINDS
                    .into_iter()
                    .map(str::to_string)
                    .collect(),
                ready_to_execute: false,
            };
        }
    };
    let phase = index.phase.clone();
    let mut cards_present = Vec::new();
    let mut missing_cards = Vec::new();
    for card in REQUIRED_CARD_KINDS {
        if issue_root
            .join("cards")
            .join(format!("{card}.values.json"))
            .is_file()
            && issue_root
                .join("cards")
                .join(format!("{card}.md"))
                .is_file()
        {
            cards_present.push(card.to_owned());
        } else {
            missing_cards.push(card.to_owned());
        }
    }
    if missing_cards.is_empty() && index.operational_authority {
        let index_value = match fs::read(&index_path)
            .ok()
            .and_then(|bytes| serde_json::from_slice::<Value>(&bytes).ok())
        {
            Some(value) => value,
            None => {
                return invalid_lifecycle_observation(
                    issue,
                    "local lifecycle index could not be recomputed",
                );
            }
        };
        let computed = match lifecycle_digest(issue_root, &index_value) {
            Ok(digest) => digest,
            Err(_) => {
                return invalid_lifecycle_observation(
                    issue,
                    "local lifecycle digest could not be recomputed from index and cards",
                );
            }
        };
        if index.digest.as_deref() != Some(computed.as_str()) {
            return invalid_lifecycle_observation(
                issue,
                "stored lifecycle digest does not match the current index and card bytes",
            );
        }
    }
    if !matches!(phase.as_str(), "ready" | "bound") {
        LocalLifecycleStateObservation {
            issue,
            phase: Some(phase.clone()),
            generation: index.generation,
            digest: index.digest,
            status: PlanStatus::Blocked,
            code: "unsupported_local_lifecycle_phase".into(),
            message: format!(
                "local lifecycle state is in phase {phase}; local execution readiness requires ready or bound"
            ),
            cards_present,
            missing_cards,
            ready_to_execute: false,
        }
    } else if missing_cards.is_empty() {
        LocalLifecycleStateObservation {
            issue,
            phase: Some(phase.clone()),
            generation: index.generation,
            digest: index.digest,
            status: PlanStatus::Ready,
            code: "local_lifecycle_state_ready".into(),
            message: format!(
                "local lifecycle state is {phase} and contains the six-card denominator"
            ),
            cards_present,
            missing_cards,
            ready_to_execute: true,
        }
    } else {
        LocalLifecycleStateObservation {
            issue,
            phase: Some(phase),
            generation: index.generation,
            digest: index.digest,
            status: PlanStatus::Blocked,
            code: "missing_lifecycle_cards".into(),
            message: "local lifecycle state exists but one or more lifecycle cards are missing"
                .into(),
            cards_present,
            missing_cards,
            ready_to_execute: false,
        }
    }
}

fn invalid_lifecycle_observation(issue: u64, message: &str) -> LocalLifecycleStateObservation {
    LocalLifecycleStateObservation {
        issue,
        phase: None,
        generation: None,
        digest: None,
        status: PlanStatus::Blocked,
        code: "invalid_local_lifecycle_digest".into(),
        message: message.into(),
        cards_present: Vec::new(),
        missing_cards: Vec::new(),
        ready_to_execute: false,
    }
}

fn write_json(path: &Path, value: &Value) -> Result<(), Vec<DoctorFinding>> {
    let bytes = serde_json::to_vec_pretty(value).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "v3_local_state_serialize_failed",
            &error.to_string(),
        )]
    })?;
    fs::write(path, bytes).map_err(|error| {
        vec![finding(
            PlanStatus::Failed,
            "v3_local_state_write_failed",
            &format!("could not write v3 local state: {error}"),
        )]
    })
}

struct LocalLifecycleIndex {
    phase: String,
    generation: Option<u64>,
    digest: Option<String>,
    operational_authority: bool,
}

fn read_local_lifecycle_index(index_path: &Path) -> Result<LocalLifecycleIndex, String> {
    let bytes = fs::read(index_path)
        .map_err(|err| format!("local lifecycle index could not be read: {err}"))?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|err| format!("local lifecycle index is not valid JSON: {err}"))?;
    let phase = value
        .get("phase")
        .and_then(Value::as_str)
        .filter(|phase| !phase.trim().is_empty())
        .map(str::to_string)
        .ok_or_else(|| "local lifecycle index is missing nonempty phase".to_string())?;
    Ok(LocalLifecycleIndex {
        phase,
        generation: value.get("generation").and_then(Value::as_u64),
        digest: value
            .get("digest")
            .and_then(Value::as_str)
            .filter(|digest| !digest.trim().is_empty())
            .map(str::to_string),
        operational_authority: value
            .get("operational_authority")
            .and_then(Value::as_bool)
            .unwrap_or(false),
    })
}
