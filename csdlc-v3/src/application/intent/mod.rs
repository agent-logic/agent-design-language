//! Issue intent adapters. Native command owners remain the only lifecycle writers.
mod context;
mod local;
mod remote;
mod terminal;
pub use context::Context;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{collections::BTreeMap, fs, path::PathBuf};

pub const INTENTS: [&str; 11] = [
    "status", "prepare", "bind", "edit", "validate", "proof", "review", "publish", "finish",
    "clean", "recover",
];
pub fn read_metrics() -> Value {
    serde_json::json!({"application_git_reads":context::application_git_reads(),"canonical_authority_file_reads":crate::authority::canonical_file_reads(),"scope":"actual application Git calls and canonical authority file reads, including repeated freshness checks; complete native Git subprocess counts are measured separately by installed fixture instrumentation"})
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Validator {
    pub id: String,
    pub program: String,
    pub args: Vec<String>,
    pub success_marker: String,
    #[serde(default = "default_validator_timeout")]
    pub timeout_seconds: u64,
}

fn default_validator_timeout() -> u64 {
    300
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Publication {
    pub base: String,
    pub title: String,
    pub body: String,
    pub draft: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntentPlan {
    pub schema: String,
    pub slug: String,
    pub cards: BTreeMap<String, Value>,
    pub validators: Vec<Validator>,
    pub publication: Publication,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AuthorityVersion {
    pub selector_digest: String,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IssueVersion {
    pub generation: Option<u64>,
    pub digest: Option<String>,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CheckoutIdentity {
    pub branch: String,
    pub head: String,
    pub root: PathBuf,
}
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Snapshot {
    pub repository: String,
    pub issue: u64,
    pub platform: String,
    pub authority: AuthorityVersion,
    pub version: IssueVersion,
    pub checkout: CheckoutIdentity,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IntentRequest {
    pub schema: String,
    pub command: String,
    pub content: Value,
    pub execute: bool,
    pub preview: Option<String>,
    pub snapshot: Snapshot,
}

/// Match only explicit positional intents or explicitly versioned advanced requests.
pub fn selected(command: &str, args: &[String]) -> bool {
    (INTENTS.contains(&command) || matches!(command, "github-issue" | "github-pr" | "pr-state"))
        && (args.first().is_some_and(|arg| arg.parse::<u64>().is_ok())
            || args.iter().any(|arg| arg == "--intent-request"))
}

pub fn run(command: &str, args: &[String]) -> Result<Value, String> {
    let mut root = std::env::current_dir().map_err(|_| "intent_cwd_unavailable")?;
    let mut issue = None;
    let mut content = Value::Null;
    let mut execute = false;
    let mut preview = None;
    let mut advanced = None;
    let mut emit = false;
    let mut seen = std::collections::BTreeSet::new();
    let mut iter = args.iter();
    if args.first().is_some_and(|arg| arg.parse::<u64>().is_ok()) {
        issue = iter.next().and_then(|arg| arg.parse::<u64>().ok());
    }
    while let Some(flag) = iter.next() {
        if !seen.insert(flag.as_str()) {
            return Err("intent_duplicate_argument".into());
        }
        match flag.as_str() {
            "--repo-root" => {
                root = PathBuf::from(iter.next().ok_or("intent_argument_value_missing")?)
            }
            "--plan" | "--changes" | "--evidence" | "--operation" | "--decisions"
            | "--disposition" => {
                let expected = match command {
                    "prepare" => "--plan",
                    "edit" => "--changes",
                    "review" => "--evidence",
                    "status" => "--decisions",
                    "finish" => "--disposition",
                    "github-issue" | "github-pr" => "--operation",
                    _ => "",
                };
                if flag != expected {
                    return Err("intent_content_argument_mismatch".into());
                }
                content = read_json(&PathBuf::from(
                    iter.next().ok_or("intent_argument_value_missing")?,
                ))?;
            }
            "--intent-request" => {
                advanced = Some(read_json(&PathBuf::from(
                    iter.next().ok_or("intent_argument_value_missing")?,
                ))?)
            }
            "--preview" => {
                preview = Some(iter.next().ok_or("intent_argument_value_missing")?.clone())
            }
            "--execute"
                if matches!(command, "clean" | "recover" | "github-issue" | "github-pr") =>
            {
                execute = true
            }
            "--emit-request" => emit = true,
            "--json" => {}
            _ => return Err("intent_unknown_argument".into()),
        }
    }
    if advanced.is_some() && (issue.is_some() || !content.is_null() || execute || preview.is_some())
    {
        return Err("intent_advanced_mixed_inputs".into());
    }
    let advanced = advanced
        .map(|value| {
            if value["schema"] == "csdlc.v3.generated_intent_request.v1" {
                if !value.as_object().is_some_and(|object| {
                    object.keys().all(|key| {
                        [
                            "schema",
                            "request",
                            "read_only",
                            "status",
                            "envelope",
                            "resolution_metrics",
                        ]
                        .contains(&key.as_str())
                    })
                }) {
                    return Err("intent_generated_request_wrapper_invalid");
                }
                Ok(value["request"].clone())
            } else {
                Ok(value)
            }
        })
        .transpose()?;
    let supplied: Option<IntentRequest> = advanced.map(serde_json::from_value).transpose()
        .map_err(|_| "intent_request_invalid_schema: regenerate through --emit-request; old internal request schemas are not intent snapshots")?;
    let issue = issue
        .or_else(|| supplied.as_ref().map(|request| request.snapshot.issue))
        .filter(|issue| *issue > 0)
        .ok_or("intent_issue_required")?;
    let context = Context::load(&root, issue)?;
    if context.cleanup_pending && command != "clean" {
        return Err("cleanup_archive_recovery_required: archived terminal evidence only permits explicit cleanup continuation".into());
    }
    let request = if let Some(request) = supplied {
        if request.schema != "csdlc.v3.intent_request.v1" || request.command != command {
            return Err("intent_request_schema_or_command_mismatch".into());
        }
        if request.snapshot.platform != std::env::consts::OS {
            return Err("intent_platform_not_admitted: serialized request platform must match the executing owner".into());
        }
        if request.snapshot != context.snapshot() {
            return Err("intent_snapshot_stale: regenerate and review the intended operation; stale inputs are never refreshed for execution".into());
        }
        request
    } else {
        IntentRequest {
            schema: "csdlc.v3.intent_request.v1".into(),
            command: command.into(),
            content,
            execute,
            preview,
            snapshot: context.snapshot(),
        }
    };
    if request.execute && !matches!(command, "clean" | "recover" | "github-issue" | "github-pr") {
        return Err("intent_execute_argument_not_supported".into());
    }
    if request.preview.is_some()
        && !matches!(
            command,
            "clean" | "recover" | "review" | "publish" | "finish" | "github-issue" | "github-pr"
        )
    {
        return Err("intent_preview_argument_not_supported".into());
    }
    if !matches!(command, "status" | "clean" | "recover") {
        context.fresh_integrity()?;
    }
    if emit {
        return Ok(
            serde_json::json!({"schema":"csdlc.v3.generated_intent_request.v1","request":request,"read_only":true,"status":"completed"}),
        );
    }
    context.fresh()?;
    let mut value = match command {
        "recover" => match remote::recover(&context, &request)? {
            Some(value) => value,
            None => local::run(&context, &request)?,
        },
        "prepare" | "status" | "bind" | "edit" | "validate" => local::run(&context, &request)?,
        "review" | "publish" | "github-issue" | "github-pr" | "pr-state" => {
            remote::run(&context, &request)?
        }
        "finish" | "clean" => terminal::run(&context, &request)?,
        "proof" => local::proof(&context, &request)?,
        _ => return Err("intent_unknown_command".into()),
    };
    value["request_issue"] = issue.into();
    value["request_expected_lifecycle_digest"] = request.snapshot.version.digest.clone().into();
    value["intent_snapshot"] = serde_json::to_value(&request.snapshot)
        .map_err(|_| "intent_snapshot_serialization_failed")?;
    value["resolution_metrics"] = read_metrics();
    if matches!(
        value["status"].as_str(),
        Some("failed" | "blocked" | "recovery_required")
    ) && command != "status"
        && (command != "recover" || request.execute)
    {
        return Err(value.to_string());
    }
    Ok(value)
}

pub fn read_json(path: &std::path::Path) -> Result<Value, String> {
    if path
        .symlink_metadata()
        .is_ok_and(|metadata| metadata.file_type().is_symlink())
    {
        return Err("intent_symlink_input_denied".into());
    }
    let bytes = fs::read(path).map_err(|_| "intent_input_unreadable")?;
    serde_json::from_slice(&bytes).map_err(|_| "intent_input_invalid_json".into())
}
