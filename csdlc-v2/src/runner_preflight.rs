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
    #[serde(default)]
    pub expected_run_id: Option<u64>,
    #[serde(default)]
    pub expected_head_sha: Option<String>,
    #[serde(default)]
    pub expected_pull_request: Option<u64>,
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
    TerminalUnassigned,
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
    pub run_id: u64,
    pub status: String,
    pub conclusion: Option<String>,
    pub labels: Vec<String>,
    pub runner_name: Option<String>,
    pub runner_group_name: Option<String>,
    pub created_at: Option<String>,
    pub started_at: Option<String>,
    pub workflow_path: String,
    pub head_sha: String,
    pub pull_requests: Vec<u64>,
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
    total_count: u64,
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
    total_count: u64,
    repositories: Vec<RepositoryIdentity>,
}

#[derive(Debug, Deserialize)]
struct RepositoryIdentity {
    full_name: String,
}

#[derive(Debug, Deserialize)]
struct ActionsJob {
    id: u64,
    run_id: u64,
    status: String,
    conclusion: Option<String>,
    #[serde(default)]
    labels: Vec<String>,
    runner_name: Option<String>,
    runner_group_name: Option<String>,
    created_at: Option<String>,
    started_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ActionsWorkflowRun {
    path: String,
    head_sha: String,
    #[serde(default)]
    pull_requests: Vec<RunPullRequest>,
}

#[derive(Debug, Deserialize)]
struct RunPullRequest {
    number: u64,
}

pub async fn inspect_runner_eligibility(
    request: &RunnerPreflightRequest,
) -> crate::Result<RunnerPreflightPacket> {
    validate_request(request)?;
    let token = crate::github_token::resolve(request.token_file.as_deref())?;
    let crab = github_client(token)?;

    let hosted = list_hosted_runners(&crab, &request.organization).await?;
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
    let repositories =
        list_selected_repositories(&crab, &request.organization, request.runner_group_id).await?;

    if group.id != request.runner_group_id {
        return Err(crate::V2Error::new(
            crate::ErrorCode::RemoteFailure,
            "GitHub runner group response did not match the requested group",
        ));
    }

    let runner = hosted.iter().find(|runner| {
        runner.name == request.expected_label && runner.runner_group_id == request.runner_group_id
    });
    let capacity = classify_capacity(runner);
    let repository_selected = repositories
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
        let run: ActionsWorkflowRun = crab
            .get(
                format!("/repos/{}/actions/runs/{}", request.repository, job.run_id),
                None::<&()>,
            )
            .await
            .map_err(remote)?;
        Some(CanaryObservation {
            job_id: job.id,
            run_id: job.run_id,
            status: job.status,
            conclusion: job.conclusion,
            labels: job.labels,
            runner_name: job.runner_name,
            runner_group_name: job.runner_group_name,
            created_at: job.created_at,
            started_at: job.started_at,
            workflow_path: run.path,
            head_sha: run.head_sha,
            pull_requests: run.pull_requests.into_iter().map(|pr| pr.number).collect(),
        })
    } else {
        None
    };
    let dispatchability = classify_dispatch(
        canary.as_ref(),
        request,
        &group.name,
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

async fn list_hosted_runners(
    crab: &Octocrab,
    organization: &str,
) -> crate::Result<Vec<HostedRunner>> {
    let mut runners = Vec::new();
    for page in 1_u64.. {
        let response: HostedRunnerList = crab
            .get(
                format!("/orgs/{organization}/actions/hosted-runners?per_page=100&page={page}"),
                None::<&()>,
            )
            .await
            .map_err(remote)?;
        let total = response.total_count as usize;
        if response.runners.is_empty() && runners.len() < total {
            return Err(incomplete_pagination("hosted runner"));
        }
        runners.extend(response.runners);
        if runners.len() >= total {
            return Ok(runners);
        }
    }
    unreachable!()
}

async fn list_selected_repositories(
    crab: &Octocrab,
    organization: &str,
    runner_group_id: u64,
) -> crate::Result<Vec<RepositoryIdentity>> {
    let mut repositories = Vec::new();
    for page in 1_u64.. {
        let response: RepositoryList = crab
            .get(
                format!(
                    "/orgs/{organization}/actions/runner-groups/{runner_group_id}/repositories?per_page=100&page={page}"
                ),
                None::<&()>,
            )
            .await
            .map_err(remote)?;
        let total = response.total_count as usize;
        if response.repositories.is_empty() && repositories.len() < total {
            return Err(incomplete_pagination("selected repository"));
        }
        repositories.extend(response.repositories);
        if repositories.len() >= total {
            return Ok(repositories);
        }
    }
    unreachable!()
}

fn incomplete_pagination(kind: &str) -> crate::V2Error {
    crate::V2Error::new(
        crate::ErrorCode::RemoteFailure,
        format!("GitHub {kind} pagination ended before total_count"),
    )
}

fn validate_request(request: &RunnerPreflightRequest) -> crate::Result<()> {
    let (owner, repo) = request.repository.split_once('/').ok_or_else(|| {
        crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "repository must be owner/name",
        )
    })?;
    let canary_context_complete = match request.canary_job_id {
        Some(_) => {
            request.expected_run_id.is_some_and(|number| number > 0)
                && request.expected_head_sha.as_deref().is_some_and(is_git_sha)
                && request
                    .expected_pull_request
                    .is_none_or(|number| number > 0)
        }
        None => {
            request.expected_run_id.is_none()
                && request.expected_head_sha.is_none()
                && request.expected_pull_request.is_none()
        }
    };
    if owner != request.organization
        || repo.is_empty()
        || request.runner_group_id == 0
        || request.expected_label.trim().is_empty()
        || !request.workflow_path.starts_with(".github/workflows/")
        || !request.workflow_path.ends_with(".yaml") && !request.workflow_path.ends_with(".yml")
        || request.queue_timeout_seconds == 0
        || !canary_context_complete
    {
        return Err(crate::V2Error::new(
            crate::ErrorCode::InvalidInput,
            "runner preflight request is invalid",
        ));
    }
    Ok(())
}

fn is_git_sha(value: &str) -> bool {
    matches!(value.len(), 40 | 64) && value.bytes().all(|byte| byte.is_ascii_hexdigit())
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
    request: &RunnerPreflightRequest,
    expected_runner_group: &str,
    now: OffsetDateTime,
) -> DispatchState {
    let Some(canary) = canary else {
        return DispatchState::Unproven;
    };
    let context_matches = request.expected_run_id == Some(canary.run_id)
        && canary.workflow_path == request.workflow_path
        && request
            .expected_head_sha
            .as_deref()
            .is_some_and(|sha| canary.head_sha == sha)
        && request
            .expected_pull_request
            .is_none_or(|number| canary.pull_requests.contains(&number));
    if !context_matches {
        return DispatchState::Mismatched;
    }
    let expected_label_present = canary
        .labels
        .iter()
        .any(|label| label == &request.expected_label);
    if canary
        .runner_name
        .as_deref()
        .is_some_and(|name| !name.is_empty())
    {
        return if expected_label_present
            && canary.runner_group_name.as_deref() == Some(expected_runner_group)
        {
            DispatchState::Proven
        } else {
            DispatchState::Mismatched
        };
    }
    let timed_out = canary
        .created_at
        .as_deref()
        .and_then(|created| OffsetDateTime::parse(created, &Rfc3339).ok())
        .is_some_and(|created| {
            (now - created).whole_seconds() >= request.queue_timeout_seconds as i64
        });
    if timed_out {
        DispatchState::TimedOut
    } else if canary.status == "completed" {
        DispatchState::TerminalUnassigned
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
        DispatchState::TimedOut | DispatchState::TerminalUnassigned | DispatchState::Mismatched => {
            OverallState::DispatchUnavailable
        }
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
        DispatchState::TerminalUnassigned => {
            values.insert(
                "canary reached a terminal state without assignment before the queue bound".into(),
            );
        }
        DispatchState::Mismatched => {
            values.insert(
                "canary identity mismatched the expected run, workflow, head, PR, label, or runner group"
                    .into(),
            );
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
            classify_workflow_ref_http(verify_ref(crab, repository, git_ref).await)
        }
        Err(_) => classify_workflow_ref_http(None),
    };
    observation
}

async fn verify_ref(crab: &Octocrab, repository: &str, git_ref: &str) -> Option<bool> {
    let (owner, name) = repository.split_once('/')?;
    let response: Value = crab
        .graphql(&serde_json::json!({
            "query": "query RunnerPreflightRef($owner: String!, $name: String!, $qualifiedName: String!) { repository(owner: $owner, name: $name) { ref(qualifiedName: $qualifiedName) { id } } }",
            "variables": {"owner": owner, "name": name, "qualifiedName": git_ref}
        }))
        .await
        .ok()?;
    classify_ref_query(&response)
}

fn classify_ref_query(response: &Value) -> Option<bool> {
    let repository = response.pointer("/data/repository")?;
    if repository.is_null() {
        return None;
    }
    Some(!repository.get("ref").is_none_or(Value::is_null))
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
            run_id: 11,
            status: "queued".into(),
            conclusion: None,
            labels: labels.iter().map(|value| (*value).into()).collect(),
            runner_name: runner.map(str::to_owned),
            runner_group_name: runner.map(|_| "adl-build-experiment".into()),
            created_at: Some(created_at.into()),
            started_at: None,
            workflow_path: ".github/workflows/ci.yaml".into(),
            head_sha: "3558f41b2395e9cb80f2804ba09f68914e9690ec".into(),
            pull_requests: vec![30],
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
            expected_run_id: None,
            expected_head_sha: None,
            expected_pull_request: None,
            queue_timeout_seconds: 300,
            token_file: None,
        }
    }

    fn classify(canary: &CanaryObservation, now: &str) -> DispatchState {
        let mut request = request();
        request.canary_job_id = Some(canary.job_id);
        request.expected_run_id = Some(11);
        request.expected_head_sha = Some("3558f41b2395e9cb80f2804ba09f68914e9690ec".into());
        request.expected_pull_request = Some(30);
        classify_dispatch(
            Some(canary),
            &request,
            "adl-build-experiment",
            OffsetDateTime::parse(now, &Rfc3339).unwrap(),
        )
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
    fn unavailable_capacity_is_distinct_from_eligible_policy() {
        assert_eq!(
            classify_overall(
                CapacityState::Unavailable,
                PolicyState::Eligible,
                DispatchState::Unproven
            ),
            OverallState::CapacityUnavailable
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
            classify(&observation, "2026-08-08T03:01:00Z"),
            DispatchState::Proven
        );
    }

    #[test]
    fn unassigned_job_over_bound_is_dispatch_unavailable() {
        let observation = canary(&["adl-ubuntu-24.04-16core"], None, "2026-08-08T03:00:00Z");
        assert_eq!(
            classify(&observation, "2026-08-08T03:06:00Z"),
            DispatchState::TimedOut
        );
    }

    #[test]
    fn terminal_unassigned_inside_bound_is_not_a_timeout() {
        let mut observation = canary(&["adl-ubuntu-24.04-16core"], None, "2026-08-08T03:00:00Z");
        observation.status = "completed".into();
        observation.conclusion = Some("skipped".into());
        assert_eq!(
            classify(&observation, "2026-08-08T03:01:00Z"),
            DispatchState::TerminalUnassigned
        );
    }

    #[test]
    fn stale_job_context_cannot_prove_dispatch() {
        let mut observation = canary(
            &["adl-ubuntu-24.04-16core"],
            Some("runner-1"),
            "2026-08-08T03:00:00Z",
        );
        observation.head_sha = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".into();
        assert_eq!(
            classify(&observation, "2026-08-08T03:01:00Z"),
            DispatchState::Mismatched
        );
        assert!(diagnostics(
            CapacityState::Ready,
            PolicyState::Eligible,
            DispatchState::Mismatched,
            true,
            false
        )
        .iter()
        .any(|message| message.contains("run, workflow, head, PR, label, or runner group")));
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
        let wrong_path = parse_workflow_ref(
            &request,
            "agent-logic/agent-design-language/.github/workflows/other.yaml@refs/heads/main",
        );
        assert_eq!(wrong_path.state, WorkflowRefState::Malformed);
        assert_eq!(
            classify_workflow_ref_http(Some(false)),
            WorkflowRefState::Stale
        );
    }

    #[test]
    fn ref_query_separates_stale_from_authorization_uncertainty() {
        assert_eq!(
            classify_ref_query(&serde_json::json!({"data": {"repository": {"ref": null}}})),
            Some(false)
        );
        assert_eq!(
            classify_ref_query(
                &serde_json::json!({"data": {"repository": {"ref": {"id": "R_1"}}}})
            ),
            Some(true)
        );
        assert_eq!(
            classify_ref_query(&serde_json::json!({"data": {"repository": null}})),
            None
        );
        assert_eq!(classify_ref_query(&serde_json::json!({"errors": []})), None);
    }
}
