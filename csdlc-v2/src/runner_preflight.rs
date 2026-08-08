use std::collections::BTreeSet;

use octocrab::Octocrab;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use strum::Display;
use time::{format_description::well_known::Rfc3339, OffsetDateTime};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct RunnerPreflightRequest {
    pub repository: String,
    pub organization: String,
    pub runner_group_id: u64,
    pub expected_label: String,
    pub workflow_path: String,
    #[serde(default)]
    pub canary_job_id: Option<u64>,
    #[serde(default = "default_queue_timeout_seconds")]
    pub queue_timeout_seconds: u64,
    #[serde(default)]
    pub token_file: Option<String>,
}

fn default_queue_timeout_seconds() -> u64 {
    300
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum CapacityState {
    Ready,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum PolicyState {
    Eligible,
    Ineligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum DispatchState {
    Proven,
    Unproven,
    TimedOut,
    Mismatched,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum OverallState {
    Eligible,
    ConfigurationEligibleDispatchUnproven,
    DispatchUnavailable,
    CapacityUnavailable,
    PolicyIneligible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, JsonSchema, Display)]
#[serde(rename_all = "snake_case")]
#[strum(serialize_all = "snake_case")]
pub enum WorkflowRefState {
    Current,
    Stale,
    Foreign,
    Malformed,
    Unverified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct WorkflowRefObservation {
    pub selected_workflow: String,
    pub repository: Option<String>,
    pub workflow_path: Option<String>,
    pub git_ref: Option<String>,
    pub state: WorkflowRefState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct CanaryObservation {
    pub job_id: u64,
    pub status: String,
    pub conclusion: Option<String>,
    pub labels: Vec<String>,
    pub runner_name: Option<String>,
    pub runner_group_name: Option<String>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
pub struct RunnerPreflightPacket {
    pub schema: String,
    pub repository: String,
    pub organization: String,
    pub runner_group_id: u64,
    pub runner_group_name: String,
    pub runner_group_visibility: String,
    pub expected_label: String,
    pub runner_status: Option<String>,
    pub maximum_runners: Option<u64>,
    pub repository_selected: bool,
    pub restricted_to_workflows: bool,
    pub selected_workflows: Vec<String>,
    pub workflow_refs: Vec<WorkflowRefObservation>,
    pub capacity: CapacityState,
    pub policy: PolicyState,
    pub dispatchability: DispatchState,
    pub classification: OverallState,
    pub canary: Option<CanaryObservation>,
    pub diagnostics: Vec<String>,
}

impl RunnerPreflightPacket {
    pub fn is_dispatch_eligible(&self) -> bool {
        self.classification == OverallState::Eligible
    }
}

#[derive(Debug, Deserialize)]
struct HostedRunnerList {
    runners: Vec<HostedRunner>,
}

#[derive(Debug, Deserialize)]
struct HostedRunner {
    name: String,
    status: String,
    maximum_runners: u64,
    runner_group_id: u64,
}

#[derive(Debug, Deserialize)]
struct RunnerGroup {
    id: u64,
    name: String,
    visibility: String,
    restricted_to_workflows: bool,
    #[serde(default)]
    selected_workflows: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RepositoryList {
    repositories: Vec<RepositoryIdentity>,
}

#[derive(Debug, Deserialize)]
struct RepositoryIdentity {
    full_name: String,
}

#[derive(Debug, Deserialize)]
struct ActionsJob {
    id: u64,
    status: String,
    conclusion: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    runner_name: Option<String>,
    runner_group_name: Option<String>,
    created_at: Option<String>,
    started_at: Option<String>,
}

pub async fn inspect_runner_eligibility(
    request: &RunnerPreflightRequest,
) -> crate::Result<RunnerPreflightPacket> {
    validate_request(request)?;
    let token = crate::github_token::resolve(request.token_file.as_deref())?;
    let crab = github_client(token)?;

    let hosted: HostedRunnerList = crab
        .get(
            format!("/orgs/{}/actions/hosted-runners", request.organization),
            None::<&()>,
        )
        .await
        .map_err(remote)?;
    let group: RunnerGroup = crab
        .get(
            format!(
                "/orgs/{}/actions/runner-groups/{}",
                request.organization, request.runner_group_id
            ),
            None::<&()>,
        )
        .await
        .map_err(remote)?;
    let repositories: RepositoryList = crab
        .get(
            format!(
                "/orgs/{}/actions/runner-groups/{}/repositories",
                request.organization, request.runner_group_id
            ),
            None::<&()>,
        )
        .await
        .map_err(remote)?;

    if group.id != request.runner_group_id {
        return Err(crate::V2Error::new(
            crate::ErrorCode::RemoteFailure,
            "GitHub runner group response did not match the requested group",
        ));
    }

    let runner = hosted.runners.iter().find(|runner| {
        runner.name == request.expected_label && runner.runner_group_id == request.runner_group_id
    });
    let capacity = classify_capacity(runner);
    let repository_selected = repositories
        .repositories
        .iter()
        .any(|repo| repo.full_name == request.repository);
    let policy = classify_policy(
        &group.visibility,
        repository_selected,
        group.restricted_to_workflows,
    );

    let mut workflow_refs = Vec::new();
    for selected in &group.selected_workflows {
        workflow_refs.push(observe_workflow_ref(&crab, request, selected).await);
    }

    let canary = if let Some(job_id) = request.canary_job_id {
        let job: ActionsJob = crab
            .get(
                format!("/repos/{}/actions/jobs/{job_id}", request.repository),
                None::<&()>,
            )
            .await
            .map_err(remote)?;
        Some(CanaryObservation {
            job_id: job.id,
            status: job.status,
            conclusion: job.conclusion,
            labels: job.labels,
            runner_name: job.runner_name,
            runner_group_name: job.runner_group_name,
            created_at: job.created_at,
            started_at: job.started_at,
        })
    } else {
        None
    };
    let dispatchability = classify_dispatch(
        canary.as_ref(),
        &request.expected_label,
        request.queue_timeout_seconds,
        OffsetDateTime::now_utc(),
    );
    let classification = classify_overall(capacity, policy, dispatchability);
    let diagnostics = diagnostics(
        capacity,
        policy,
        dispatchability,
        repository_selected,
        group.restricted_to_workflows,
    );

    Ok(RunnerPreflightPacket {
        schema: "csdlc.runner_preflight.v1".into(),
        repository: request.repository.clone(),
        organization: request.organization.clone(),
        runner_group_id: request.runner_group_id,
        runner_group_name: group.name,
        runner_group_visibility: group.visibility,
        expected_label: request.expected_label.clone(),
        runner_status: runner.map(|runner| runner.status.clone()),
        maximum_runners: runner.map(|runner| runner.maximum_runners),
        repository_selected,
        restricted_to_workflows: group.restricted_to_workflows,
        selected_workflows: group.selected_workflows,
        workflow_refs,
        capacity,
        policy,
        dispatchability,
        classification,
        canary,
        diagnostics,
    })
}

fn validate_request(request: &RunnerPreflightRequest) -> crate::Result<()> {
    let (owner, repo) = request.repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })?;
    if owner != request.organization
        || repo.is_empty()
        || request.runner_group_id == 0
        || request.expected_label.trim().is_empty()
        || !request.workflow_path.starts_with(".github/workflows/")
        || !request.workflow_path.ends_with(".yaml") && !request.workflow_path.ends_with(".yml")
        || request.queue_timeout_seconds == 0
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "runner preflight request is invalid",
        ));
    }
    Ok(())
}

fn classify_capacity(runner: Option<&HostedRunner>) -> CapacityState {
    if runner.is_some_and(|runner| runner.status == "Ready" && runner.maximum_runners > 0) {
        CapacityState::Ready
    } else {
        CapacityState::Unavailable
    }
}

fn classify_policy(
    visibility: &str,
    repository_selected: bool,
    restricted_to_workflows: bool,
) -> PolicyState {
    if visibility == "selected" && repository_selected && !restricted_to_workflows {
        PolicyState::Eligible
    } else {
        PolicyState::Ineligible
    }
}

fn classify_dispatch(
    canary: Option<&CanaryObservation>,
    expected_label: &str,
    timeout_seconds: u64,
    now: OffsetDateTime,
) -> DispatchState {
    let Some(canary) = canary else {
        return DispatchState::Unproven;
    };
    let expected_label_present = canary.labels.iter().any(|label| label == expected_label);
    if expected_label_present
        && canary
            .runner_name
            .as_deref()
            .is_some_and(|name| !name.is_empty())
    {
        return DispatchState::Proven;
    }
    if canary
        .runner_name
        .as_deref()
        .is_some_and(|name| !name.is_empty())
    {
        return DispatchState::Mismatched;
    }
    let timed_out = canary
        .created_at
        .as_deref()
        .and_then(|created| OffsetDateTime::parse(created, &Rfc3339).ok())
        .is_some_and(|created| (now - created).whole_seconds() >= timeout_seconds as i64);
    if timed_out || canary.status == "completed" {
        DispatchState::TimedOut
    } else {
        DispatchState::Unproven
    }
}

fn classify_overall(
    capacity: CapacityState,
    policy: PolicyState,
    dispatch: DispatchState,
) -> OverallState {
    if policy == PolicyState::Ineligible {
        return OverallState::PolicyIneligible;
    }
    if capacity == CapacityState::Unavailable {
        return OverallState::CapacityUnavailable;
    }
    match dispatch {
        DispatchState::Proven => OverallState::Eligible,
        DispatchState::Unproven => OverallState::ConfigurationEligibleDispatchUnproven,
        DispatchState::TimedOut | DispatchState::Mismatched => OverallState::DispatchUnavailable,
    }
}

fn diagnostics(
    capacity: CapacityState,
    policy: PolicyState,
    dispatch: DispatchState,
    repository_selected: bool,
    restricted: bool,
) -> Vec<String> {
    let mut values = BTreeSet::new();
    if capacity == CapacityState::Unavailable {
        values.insert(
            "expected hosted runner label is absent, not Ready, or has zero configured maximum"
                .into(),
        );
    }
    if !repository_selected {
        values.insert("runner group does not explicitly select the target repository".into());
    }
    if restricted {
        values
            .insert("workflow restriction is branch-dependent policy and must be disabled".into());
    }
    match dispatch {
        DispatchState::Unproven => {
            values
                .insert("configuration is not dispatch proof; provide a bounded canary job".into());
        }
        DispatchState::TimedOut => {
            values.insert("canary exceeded the queue bound without runner assignment".into());
        }
        DispatchState::Mismatched => {
            values.insert("canary ran on a different label or runner group".into());
        }
        DispatchState::Proven => {}
    }
    if policy == PolicyState::Ineligible && repository_selected && !restricted {
        values.insert("runner group visibility is not selected-repository scope".into());
    }
    values.into_iter().collect()
}

async fn observe_workflow_ref(
    crab: &Octocrab,
    request: &RunnerPreflightRequest,
    selected: &str,
) -> WorkflowRefObservation {
    let mut observation = parse_workflow_ref(request, selected);
    if observation.state != WorkflowRefState::Unverified {
        return observation;
    }
    let repository = observation
        .repository
        .as_deref()
        .expect("parsed repository");
    let workflow_path = observation
        .workflow_path
        .as_deref()
        .expect("parsed workflow path");
    let git_ref = observation.git_ref.as_deref().expect("parsed git ref");
    let endpoint = format!(
        "/repos/{repository}/contents/{workflow_path}?ref={}",
        url::form_urlencoded::byte_serialize(git_ref.as_bytes()).collect::<String>()
    );
    observation.state = match crab.get::<Value, _, ()>(endpoint, None::<&()>).await {
        Ok(_) => classify_workflow_ref_http(Some(true)),
        Err(octocrab::Error::GitHub { source, .. }) if source.status_code.as_u16() == 404 => {
            classify_workflow_ref_http(Some(false))
        }
        Err(_) => classify_workflow_ref_http(None),
    };
    observation
}

fn parse_workflow_ref(request: &RunnerPreflightRequest, selected: &str) -> WorkflowRefObservation {
    let Some((identity, git_ref)) = selected.rsplit_once('@') else {
        return workflow_observation(selected, None, None, None, WorkflowRefState::Malformed);
    };
    let components: Vec<_> = identity.splitn(3, '/').collect();
    if components.len() != 3 {
        return workflow_observation(
            selected,
            None,
            None,
            Some(git_ref),
            WorkflowRefState::Malformed,
        );
    }
    let repository = format!("{}/{}", components[0], components[1]);
    let workflow_path = components[2].to_owned();
    if repository != request.repository {
        return workflow_observation(
            selected,
            Some(&repository),
            Some(&workflow_path),
            Some(git_ref),
            WorkflowRefState::Foreign,
        );
    }
    if workflow_path != request.workflow_path || !git_ref.starts_with("refs/") {
        return workflow_observation(
            selected,
            Some(&repository),
            Some(&workflow_path),
            Some(git_ref),
            WorkflowRefState::Malformed,
        );
    }
    workflow_observation(
        selected,
        Some(&repository),
        Some(&workflow_path),
        Some(git_ref),
        WorkflowRefState::Unverified,
    )
}

fn classify_workflow_ref_http(found: Option<bool>) -> WorkflowRefState {
    match found {
        Some(true) => WorkflowRefState::Current,
        Some(false) => WorkflowRefState::Stale,
        None => WorkflowRefState::Unverified,
    }
}

fn workflow_observation(
    selected: &str,
    repository: Option<&str>,
    workflow_path: Option<&str>,
    git_ref: Option<&str>,
    state: WorkflowRefState,
) -> WorkflowRefObservation {
    WorkflowRefObservation {
        selected_workflow: selected.to_owned(),
        repository: repository.map(str::to_owned),
        workflow_path: workflow_path.map(str::to_owned),
        git_ref: git_ref.map(str::to_owned),
        state,
    }
}

fn github_client(token: String) -> crate::Result<Octocrab> {
    let mut builder = Octocrab::builder().personal_token(token);
    #[cfg(debug_assertions)]
    if let Some(base) = std::env::var_os("CSDLC_V2_TEST_GITHUB_API_BASE") {
        let base = base.to_string_lossy();
        let parsed = url::Url::parse(&base).map_err(|_| {
            crate::V2Error::new(crate::ErrorCode::InvalidInput, "test API base is invalid")
        })?;
        let loopback = match parsed.host() {
            Some(url::Host::Domain("localhost")) => true,
            Some(url::Host::Ipv4(address)) => address.is_loopback(),
            Some(url::Host::Ipv6(address)) => address.is_loopback(),
            _ => false,
        };
        if parsed.scheme() != "http" || !loopback || parsed.path() != "/" {
            return Err(crate::V2Error::new(
                crate::ErrorCode::InvalidInput,
                "test API base must be an HTTP loopback origin",
            ));
        }
        builder = builder.base_uri(base.as_ref()).map_err(remote)?;
    }
    builder.build().map_err(remote)
}

fn remote(error: octocrab::Error) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::RemoteFailure,
        format!("GitHub runner observation failed: {error}"),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn canary(labels: &[&str], runner: Option<&str>, created_at: &str) -> CanaryObservation {
        CanaryObservation {
            job_id: 7,
            status: "queued".into(),
            conclusion: None,
            labels: labels.iter().map(|value| (*value).into()).collect(),
            runner_name: runner.map(str::to_owned),
            runner_group_name: runner.map(|_| "adl-build-experiment".into()),
            created_at: Some(created_at.into()),
            started_at: None,
        }
    }

    fn request() -> RunnerPreflightRequest {
        RunnerPreflightRequest {
            repository: "agent-logic/agent-design-language".into(),
            organization: "agent-logic".into(),
            runner_group_id: 3,
            expected_label: "adl-ubuntu-24.04-16core".into(),
            workflow_path: ".github/workflows/ci.yaml".into(),
            canary_job_id: None,
            queue_timeout_seconds: 300,
            token_file: None,
        }
    }

    #[test]
    fn ready_and_policy_eligible_is_not_dispatch_proof() {
        assert_eq!(
            classify_overall(
                CapacityState::Ready,
                PolicyState::Eligible,
                DispatchState::Unproven
            ),
            OverallState::ConfigurationEligibleDispatchUnproven
        );
    }

    #[test]
    fn policy_ineligible_takes_precedence_over_ready_capacity() {
        assert_eq!(
            classify_overall(
                CapacityState::Ready,
                PolicyState::Ineligible,
                DispatchState::Unproven
            ),
            OverallState::PolicyIneligible
        );
    }

    #[test]
    fn assigned_expected_label_proves_dispatch() {
        let observation = canary(
            &["adl-ubuntu-24.04-16core"],
            Some("runner-1"),
            "2026-08-08T03:00:00Z",
        );
        assert_eq!(
            classify_dispatch(
                Some(&observation),
                "adl-ubuntu-24.04-16core",
                300,
                OffsetDateTime::parse("2026-08-08T03:01:00Z", &Rfc3339).unwrap()
            ),
            DispatchState::Proven
        );
    }

    #[test]
    fn unassigned_job_over_bound_is_dispatch_unavailable() {
        let observation = canary(&["adl-ubuntu-24.04-16core"], None, "2026-08-08T03:00:00Z");
        assert_eq!(
            classify_dispatch(
                Some(&observation),
                "adl-ubuntu-24.04-16core",
                300,
                OffsetDateTime::parse("2026-08-08T03:06:00Z", &Rfc3339).unwrap()
            ),
            DispatchState::TimedOut
        );
    }

    #[test]
    fn repository_scope_and_workflow_restriction_are_both_required() {
        assert_eq!(
            classify_policy("selected", true, false),
            PolicyState::Eligible
        );
        assert_eq!(
            classify_policy("selected", false, false),
            PolicyState::Ineligible
        );
        assert_eq!(
            classify_policy("selected", true, true),
            PolicyState::Ineligible
        );
        assert_eq!(classify_policy("all", true, false), PolicyState::Ineligible);
    }

    #[test]
    fn malformed_and_foreign_workflow_refs_are_explicit() {
        let request = request();
        let malformed = parse_workflow_ref(&request, "bad");
        assert_eq!(malformed.state, WorkflowRefState::Malformed);
        let foreign = parse_workflow_ref(
            &request,
            "other/repo/.github/workflows/ci.yaml@refs/heads/main",
        );
        assert_eq!(foreign.state, WorkflowRefState::Foreign);
        let current = parse_workflow_ref(
            &request,
            "agent-logic/agent-design-language/.github/workflows/ci.yaml@refs/heads/main",
        );
        assert_eq!(current.state, WorkflowRefState::Unverified);
        assert_eq!(
            classify_workflow_ref_http(Some(false)),
            WorkflowRefState::Stale
        );
    }
}
