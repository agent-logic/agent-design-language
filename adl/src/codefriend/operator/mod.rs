//! Operator-facing CodeFriend review shell state.
//!
//! This module is deliberately a thin control layer over the isolated review
//! runner. It does not synthesize findings, publish results, or grant source
//! mutation authority.

use crate::{
    codefriend::review::runner::{
        review_run_summary, run_from_store, ReviewRunOptions, REVIEW_RUN_SCHEMA,
    },
    provider_communication::ProviderInvocationRequestV1,
};
use anyhow::{ensure, Context, Result};
use serde::{Deserialize, Serialize};
use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

pub const OPERATOR_STATE_SCHEMA: &str = "codefriend.operator_review_shell.v1";

#[derive(Debug, Clone)]
pub struct OperatorStartOptions {
    pub store: PathBuf,
    pub packet_id: String,
    pub provider_request: ProviderInvocationRequestV1,
    pub out: PathBuf,
    pub run_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum OperatorReviewStatus {
    Incomplete,
    Complete,
    Failed,
    Cancelled,
    WithheldPublication,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OperatorAttempt {
    pub attempt: u64,
    pub run_id: String,
    pub review_out: String,
    pub status: OperatorReviewStatus,
    pub summary_ref: Option<String>,
    pub failure: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(deny_unknown_fields)]
pub struct OperatorReviewState {
    pub schema: String,
    pub review_schema: String,
    pub packet_id: String,
    pub store_ref: String,
    pub provider_route: String,
    pub status: OperatorReviewStatus,
    pub active_attempt: u64,
    pub attempts: Vec<OperatorAttempt>,
    pub cancel_request_ref: Option<String>,
    pub publication_withheld_reason: Option<String>,
    pub artifact_navigation: Vec<String>,
    pub message: String,
}

pub fn start_review(options: OperatorStartOptions) -> Result<OperatorReviewState> {
    ensure!(
        options
            .run_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
        "invalid_review_run_id"
    );
    ensure!(
        !options.out.exists(),
        "review_shell_output_directory_already_exists"
    );
    fs::create_dir_all(&options.out)?;
    let mut state = new_state(&options, 1);
    state.attempts.push(OperatorAttempt {
        attempt: 1,
        run_id: options.run_id.clone(),
        review_out: attempt_review_ref(1),
        status: OperatorReviewStatus::Incomplete,
        summary_ref: None,
        failure: None,
    });
    write_state(&options.out, &state)?;
    run_attempt(
        &options.out,
        &mut state,
        options.provider_request,
        options.run_id,
        1,
    )?;
    Ok(state)
}

pub fn inspect_review(out: &Path) -> Result<OperatorReviewState> {
    read_state(out)
}

pub fn cancel_review(out: &Path, reason: &str) -> Result<OperatorReviewState> {
    ensure!(!reason.trim().is_empty(), "cancel_reason_required");
    let mut state = read_state(out)?;
    let cancel = CancelRequest {
        schema: "codefriend.operator_cancel_request.v1".to_string(),
        reason: reason.trim().to_string(),
    };
    write_json(&out.join("cancel-request.json"), &cancel)?;
    state.cancel_request_ref = Some("cancel-request.json".to_string());
    if state.status == OperatorReviewStatus::Incomplete {
        state.status = OperatorReviewStatus::Cancelled;
        if let Some(attempt) = state
            .attempts
            .iter_mut()
            .find(|attempt| attempt.attempt == state.active_attempt)
        {
            attempt.status = OperatorReviewStatus::Cancelled;
            attempt.failure = Some("operator_cancel_requested".to_string());
        }
        state.message =
            "cancelled; review runner will settle safely at the next cancellation checkpoint"
                .to_string();
        write_state(out, &state)?;
    }
    Ok(state)
}

pub fn retry_review(
    out: &Path,
    provider_request: ProviderInvocationRequestV1,
    run_id: &str,
) -> Result<OperatorReviewState> {
    ensure!(
        run_id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._-".contains(&b)),
        "invalid_review_run_id"
    );
    let mut state = read_state(out)?;
    ensure!(
        state.status != OperatorReviewStatus::Complete,
        "retry_requires_noncomplete_run"
    );
    archive_cancel_request_for_retry(out, &state)?;
    let attempt = state
        .attempts
        .iter()
        .map(|attempt| attempt.attempt)
        .max()
        .unwrap_or(0)
        + 1;
    state.status = OperatorReviewStatus::Incomplete;
    state.active_attempt = attempt;
    state.cancel_request_ref = None;
    state.publication_withheld_reason = None;
    state.message = "retry in progress".to_string();
    state.attempts.push(OperatorAttempt {
        attempt,
        run_id: run_id.to_string(),
        review_out: attempt_review_ref(attempt),
        status: OperatorReviewStatus::Incomplete,
        summary_ref: None,
        failure: None,
    });
    write_state(out, &state)?;
    run_attempt(
        out,
        &mut state,
        provider_request,
        run_id.to_string(),
        attempt,
    )?;
    Ok(state)
}

fn archive_cancel_request_for_retry(out: &Path, state: &OperatorReviewState) -> Result<()> {
    let cancel_path = out.join("cancel-request.json");
    if !cancel_path.exists() {
        return Ok(());
    }
    let archive_ref = format!(
        "attempts/{}/cancel-request.json",
        state.active_attempt.max(1)
    );
    let archive_path = out.join(&archive_ref);
    if let Some(parent) = archive_path.parent() {
        fs::create_dir_all(parent)?;
    }
    ensure!(
        !archive_path.exists(),
        "cancel_request_archive_already_exists"
    );
    fs::rename(&cancel_path, &archive_path).with_context(|| {
        format!(
            "archive {} to {}",
            cancel_path.display(),
            archive_path.display()
        )
    })?;
    Ok(())
}

pub fn withhold_publication(out: &Path, reason: &str) -> Result<OperatorReviewState> {
    ensure!(!reason.trim().is_empty(), "withhold_reason_required");
    let mut state = read_state(out)?;
    ensure!(
        state.status == OperatorReviewStatus::Complete,
        "withhold_publication_requires_complete_run"
    );
    state.status = OperatorReviewStatus::WithheldPublication;
    state.publication_withheld_reason = Some(reason.trim().to_string());
    state.message = "publication withheld by operator policy".to_string();
    write_state(out, &state)?;
    Ok(state)
}

fn new_state(options: &OperatorStartOptions, active_attempt: u64) -> OperatorReviewState {
    OperatorReviewState {
        schema: OPERATOR_STATE_SCHEMA.to_string(),
        review_schema: REVIEW_RUN_SCHEMA.to_string(),
        packet_id: options.packet_id.clone(),
        store_ref: options.store.display().to_string(),
        provider_route: provider_route_identity(&options.provider_request),
        status: OperatorReviewStatus::Incomplete,
        active_attempt,
        attempts: Vec::new(),
        cancel_request_ref: None,
        publication_withheld_reason: None,
        artifact_navigation: vec!["operator-state.json".to_string()],
        message: "review in progress".to_string(),
    }
}

fn run_attempt(
    out: &Path,
    state: &mut OperatorReviewState,
    provider_request: ProviderInvocationRequestV1,
    run_id: String,
    attempt: u64,
) -> Result<()> {
    let review_out = out.join(attempt_review_ref(attempt));
    let result = run_from_store(ReviewRunOptions {
        store: PathBuf::from(&state.store_ref),
        packet_id: state.packet_id.clone(),
        provider_request,
        out: review_out.clone(),
        run_id,
        cancel_file: Some(out.join("cancel-request.json")),
    });
    let summary_ref = if review_out.join("run.json").exists() {
        Some(format!("{}/run.json", attempt_review_ref(attempt)))
    } else {
        None
    };
    match result {
        Ok(output) => {
            let summary_path = out.join(format!("attempt-{attempt}-summary.json"));
            write_json(&summary_path, &review_run_summary(&output)?)?;
            set_attempt(
                state,
                attempt,
                OperatorReviewStatus::Complete,
                Some(format!("attempt-{attempt}-summary.json")),
                None,
            );
            state.status = OperatorReviewStatus::Complete;
            state.message =
                "review complete; publication is still a separate downstream authority".to_string();
        }
        Err(error) => {
            let cancelled = out.join("cancel-request.json").exists();
            let status = if cancelled {
                OperatorReviewStatus::Cancelled
            } else {
                OperatorReviewStatus::Failed
            };
            set_attempt(
                state,
                attempt,
                status.clone(),
                summary_ref,
                Some(sanitize_failure(&error.to_string())),
            );
            state.status = status;
            state.message = if cancelled {
                "review cancelled and settled safely".to_string()
            } else {
                "review failed; retry preserves prior attempt evidence".to_string()
            };
        }
    }
    state.artifact_navigation = artifact_navigation(out)?;
    write_state(out, state)?;
    Ok(())
}

fn set_attempt(
    state: &mut OperatorReviewState,
    attempt: u64,
    status: OperatorReviewStatus,
    summary_ref: Option<String>,
    failure: Option<String>,
) {
    if let Some(existing) = state
        .attempts
        .iter_mut()
        .find(|existing| existing.attempt == attempt)
    {
        existing.status = status;
        existing.summary_ref = summary_ref;
        existing.failure = failure;
    }
}

fn attempt_review_ref(attempt: u64) -> String {
    format!("attempts/{attempt}/review")
}

fn provider_route_identity(request: &ProviderInvocationRequestV1) -> String {
    format!(
        "{}:{:?}:{}",
        request.route.provider, request.route.runtime_surface, request.route.provider_model_id
    )
}

fn state_path(out: &Path) -> PathBuf {
    out.join("operator-state.json")
}

fn read_state(out: &Path) -> Result<OperatorReviewState> {
    let mut bytes = Vec::new();
    File::open(state_path(out))
        .with_context(|| format!("open {}", state_path(out).display()))?
        .take(256 * 1024 + 1)
        .read_to_end(&mut bytes)?;
    ensure!(bytes.len() <= 256 * 1024, "operator_state_too_large");
    serde_json::from_slice(&bytes).map_err(|_| anyhow::anyhow!("invalid_operator_state_json"))
}

fn write_state(out: &Path, state: &OperatorReviewState) -> Result<()> {
    write_json(&state_path(out), state)
}

fn write_json<T: Serialize>(path: &Path, value: &T) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    let bytes = serde_json::to_vec_pretty(value)?;
    let mut file = File::create(path).with_context(|| format!("write {}", path.display()))?;
    file.write_all(&bytes)?;
    file.write_all(b"\n")?;
    file.sync_all()?;
    Ok(())
}

fn artifact_navigation(out: &Path) -> Result<Vec<String>> {
    let mut artifacts = vec!["operator-state.json".to_string()];
    for entry in fs::read_dir(out.join("attempts")).or_else(|error| {
        if error.kind() == std::io::ErrorKind::NotFound {
            Ok(fs::read_dir(out)?)
        } else {
            Err(error)
        }
    })? {
        let entry = entry?;
        let file_type = entry.file_type()?;
        if file_type.is_dir() {
            let name = entry.file_name().to_string_lossy().to_string();
            let run = format!("attempts/{name}/review/run.json");
            if out.join(&run).exists() {
                artifacts.push(run);
            }
            let record = format!("attempts/{name}/review/review-record.json");
            if out.join(&record).exists() {
                artifacts.push(record);
            }
        }
    }
    artifacts.sort();
    artifacts.dedup();
    Ok(artifacts)
}

fn sanitize_failure(message: &str) -> String {
    message
        .chars()
        .filter(|c| c.is_ascii_graphic() || c.is_ascii_whitespace())
        .collect::<String>()
        .replace('\n', " ")
        .chars()
        .take(512)
        .collect()
}

#[derive(Debug, Clone, Serialize)]
#[serde(deny_unknown_fields)]
struct CancelRequest {
    schema: String,
    reason: String,
}
