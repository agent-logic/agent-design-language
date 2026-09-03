use std::fs;
use std::path::{Path, PathBuf};

use clap::{Parser, Subcommand};
use csdlc_v2::error::{ErrorCode, V2Error};
use csdlc_v2::{
    prepare_publication, prepare_ready_publication, prepare_ready_reconciliation, reconcile_action,
    record_publication, record_ready_publication, record_ready_reconciliation, PublicationAction,
    PublicationEvidence, PublicationIntent, PublicationRequest,
    ReadyPublicationReconciliationRequest, ReadyPublicationRequest, RemotePullRequest, Store,
};
use octocrab::models::IssueState;
use octocrab::params::State;
use tokio::time::{sleep, Duration};

#[derive(Parser)]
struct Cli {
    #[arg(long, default_value = ".")]
    root: PathBuf,
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Publish {
        #[arg(long)]
        request: PathBuf,
    },
    Status {
        #[arg(long)]
        request: PathBuf,
    },
    Ready {
        #[arg(long)]
        request: PathBuf,
    },
    ReconcileReady {
        #[arg(long)]
        request: PathBuf,
    },
    Schema,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    let result = run(&cli).await;
    match result {
        Ok(value) => println!("{}", serde_json::to_string_pretty(&value).expect("JSON")),
        Err(error) => {
            println!(
                "{}",
                serde_json::json!({"schema":"csdlc.error.v1","code":error.code.to_string(),"message":error.message})
            );
            std::process::exit(error.code.exit_code());
        }
    }
}

async fn run(cli: &Cli) -> csdlc_v2::Result<serde_json::Value> {
    if matches!(cli.command, Command::Schema) {
        return Ok(csdlc_v2::public_schema_bundle());
    }
    match &cli.command {
        Command::Publish { .. } => {
            csdlc_v2::verify_installed_owner_operation(&cli.root, "publish")?
        }
        Command::Ready { .. } => csdlc_v2::verify_installed_owner_operation(&cli.root, "ready")?,
        Command::ReconcileReady { .. } => {
            csdlc_v2::verify_installed_owner_operation(&cli.root, "reconcile-ready")?
        }
        Command::Status { .. } | Command::Schema => {}
    }
    if let Command::Ready { request } = &cli.command {
        return mark_ready(&cli.root, request).await;
    }
    if let Command::ReconcileReady { request } = &cli.command {
        return reconcile_ready(&cli.root, request).await;
    }
    let request_path = match &cli.command {
        Command::Publish { request } | Command::Status { request } => request,
        Command::Ready { .. } | Command::ReconcileReady { .. } => unreachable!(),
        Command::Schema => unreachable!(),
    };
    let request: PublicationRequest = serde_json::from_slice(&fs::read(request_path)?)?;
    let token = resolve_token(&request)?;
    let crab = github_client(token)?;
    let store = Store::new(&cli.root);
    if let Some(intent) =
        csdlc_v2::publication::resume_recorded_publication_intent(&store, &request)?
    {
        verify_git_remote(&cli.root, &request.remote, &intent)?;
        let metadata_head = if let Some(metadata_head) =
            csdlc_v2::publication::commit_publication_metadata_tail(&cli.root, request.issue)?
        {
            push(&cli.root, &request.remote, &request.head)?;
            metadata_head
        } else {
            csdlc_v2::publication::current_head_sha(&cli.root)?
        };
        push(&cli.root, &request.remote, &request.head)?;
        let observed = reobserve_pr_at_head(&crab, &intent, &metadata_head)
            .await?
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "resumed metadata publication head could not be reconciled",
                )
            })?;
        let publication = normalize(&intent, &observed, &crab).await?;
        if metadata_head == intent.commit_sha {
            csdlc_v2::publication::validate_remote(&intent, &publication)?;
        } else {
            validate_metadata_followup_remote(&cli.root, &intent, &publication, &metadata_head)?;
        }
        let record = store.load_record(request.issue)?;
        return Ok(
            serde_json::json!({"schema":"csdlc.publication_result.v1","publication":publication,"generation":record.generation,"digest":record.digest}),
        );
    }
    let intent = prepare_publication(&store, &request)?;
    let observed = find_pr(&crab, &intent).await?;
    if matches!(cli.command, Command::Status { .. }) {
        let observed = observed.ok_or_else(|| {
            V2Error::new(ErrorCode::ReconciliationRequired, "matching PR not found")
        })?;
        let normalized = normalize(&intent, &observed, &crab).await?;
        csdlc_v2::publication::validate_remote(&intent, &normalized)?;
        return serde_json::to_value(normalized).map_err(Into::into);
    }
    verify_git_remote(&cli.root, &request.remote, &intent)?;
    let before = match observed.as_ref() {
        Some(pr) => Some(normalize(&intent, pr, &crab).await?),
        None => None,
    };
    if let Some(value) = &before {
        if !existing_pr_matches_governed_mode(&intent, value) {
            return Err(V2Error::new(
                ErrorCode::ReconciliationRequired,
                "existing PR does not match this issue's governed publication mode",
            ));
        }
    }
    let action = reconcile_action(&intent, before.as_ref())?;
    csdlc_v2::publication::persist_publication_intent(&cli.root, &intent)?;
    if before
        .as_ref()
        .is_none_or(|value| value.head_sha != intent.commit_sha)
    {
        push(&cli.root, &request.remote, &request.head)?;
    }
    let remote = match observed {
        Some(pr) => {
            if action == PublicationAction::Update {
                crab.pulls(owner(&intent)?, repo(&intent)?)
                    .update(pr_number(&pr)?)
                    .title(&intent.title)
                    .body(&intent.body)
                    .base(&intent.base)
                    .send()
                    .await
                    .map_err(|e| remote(e.to_string()))?;
            }
            find_pr(&crab, &intent).await?.ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "updated PR could not be reconciled",
                )
            })?
        }
        None => {
            let pulls = crab.pulls(owner(&intent)?, repo(&intent)?);
            let create = pulls
                .create(&intent.title, &intent.head, &intent.base)
                .body(&intent.body)
                .draft(intent.draft);
            let send_failed = create.send().await.is_err();
            let observed = find_pr(&crab, &intent).await?;
            reconcile_create_observation(send_failed, observed.is_some())?;
            observed.expect("presence checked")
        }
    };
    let normalized = normalize(&intent, &remote, &crab).await?;
    csdlc_v2::publication::validate_remote(&intent, &normalized)?;
    let record = record_publication(&store, &request, &intent, normalized.clone())?;
    let mut publication = normalized;
    if let Some(metadata_head) =
        csdlc_v2::publication::commit_publication_metadata_tail(&cli.root, request.issue)?
    {
        push(&cli.root, &request.remote, &request.head)?;
        let observed = reobserve_pr_at_head(&crab, &intent, &metadata_head)
            .await?
            .ok_or_else(|| {
                V2Error::new(
                    ErrorCode::ReconciliationRequired,
                    "metadata publication head could not be reconciled",
                )
            })?;
        publication = normalize(&intent, &observed, &crab).await?;
        validate_metadata_followup_remote(&cli.root, &intent, &publication, &metadata_head)?;
    }
    Ok(
        serde_json::json!({"schema":"csdlc.publication_result.v1","publication":publication,"generation":record.generation,"digest":record.digest}),
    )
}

async fn mark_ready(root: &Path, request_path: &Path) -> csdlc_v2::Result<serde_json::Value> {
    let request: ReadyPublicationRequest = serde_json::from_slice(&fs::read(request_path)?)?;
    let store = Store::new(root);
    let governed = prepare_ready_publication(&store, &request)?;
    let token = csdlc_v2::github_token::resolve(request.token_file.as_deref())?;
    let crab = github_client(token)?;
    let (owner, repo) = repository_parts(
        request
            .code_repository
            .as_deref()
            .unwrap_or(&request.repository),
    )?;
    let before = crab
        .pulls(owner, repo)
        .get(request.pull_request)
        .await
        .map_err(|error| remote(error.to_string()))?;
    let before = normalize_ready(&governed, &before)?;
    csdlc_v2::publication::validate_ready_remote(&governed, &request, &before, true)?;
    let node_id = ready_node_id(&request, &before, &crab).await?;
    let _: serde_json::Value = crab
        .graphql(&serde_json::json!({
            "query": "mutation MarkReady($pullRequestId: ID!) { markPullRequestReadyForReview(input: {pullRequestId: $pullRequestId}) { pullRequest { id isDraft } } }",
            "variables": {"pullRequestId": node_id}
        }))
        .await
        .map_err(|error| remote(error.to_string()))?;
    let after = crab
        .pulls(owner, repo)
        .get(request.pull_request)
        .await
        .map_err(|error| remote(error.to_string()))?;
    let after = normalize_ready(&governed, &after)?;
    csdlc_v2::publication::validate_ready_remote(&governed, &request, &after, false)?;
    let record = record_ready_publication(&store, &request, &governed, after.clone())?;
    let mut publication = after;
    if let Some(metadata_head) =
        csdlc_v2::publication::commit_publication_metadata_tail(root, request.issue)?
    {
        push(root, &request.remote, &governed.head)?;
        let observed =
            reobserve_ready_pr_at_head(&crab, owner, repo, request.pull_request, &metadata_head)
                .await?;
        publication = normalize_ready(&governed, &observed)?;
        validate_ready_metadata_followup_remote(
            root,
            &governed,
            &request,
            &publication,
            &metadata_head,
        )?;
    }
    Ok(serde_json::json!({
        "schema": "csdlc.ready_publication_result.v1",
        "publication": publication,
        "generation": record.generation,
        "digest": record.digest
    }))
}

async fn reconcile_ready(root: &Path, request_path: &Path) -> csdlc_v2::Result<serde_json::Value> {
    let request: ReadyPublicationReconciliationRequest =
        serde_json::from_slice(&fs::read(request_path)?)?;
    let store = Store::new(root);
    let governed = prepare_ready_reconciliation(&store, &request)?;
    let token = csdlc_v2::github_token::resolve(request.ready.token_file.as_deref())?;
    let crab = github_client(token)?;
    let (owner, repo) = repository_parts(
        request
            .ready
            .code_repository
            .as_deref()
            .unwrap_or(&request.ready.repository),
    )?;
    let observed = crab
        .pulls(owner, repo)
        .get(request.ready.pull_request)
        .await
        .map_err(|error| remote(error.to_string()))?;
    let observed = normalize_ready(&governed, &observed)?;
    csdlc_v2::publication::validate_ready_remote(&governed, &request.ready, &observed, false)?;
    let record = record_ready_reconciliation(&store, &request, &governed, observed.clone())?;
    let mut publication = observed;
    if let Some(metadata_head) =
        csdlc_v2::publication::commit_publication_metadata_tail(root, request.ready.issue)?
    {
        push(root, &request.ready.remote, &governed.head)?;
        let observed = reobserve_ready_pr_at_head(
            &crab,
            owner,
            repo,
            request.ready.pull_request,
            &metadata_head,
        )
        .await?;
        publication = normalize_ready(&governed, &observed)?;
        validate_ready_metadata_followup_remote(
            root,
            &governed,
            &request.ready,
            &publication,
            &metadata_head,
        )?;
    }
    Ok(serde_json::json!({
        "schema": "csdlc.ready_publication_reconciliation_result.v1",
        "publication": publication,
        "generation": record.generation,
        "digest": record.digest
    }))
}

fn existing_pr_matches_governed_mode(
    intent: &PublicationIntent,
    value: &RemotePullRequest,
) -> bool {
    csdlc_v2::publication::body_has_exact_publication_linkage(
        &value.body,
        intent.issue,
        &intent.issue_repository,
        intent.repository != intent.issue_repository,
        intent.linkage_mode,
    ) && value.draft == intent.draft
        && match intent.linkage_mode {
            csdlc_v2::PublicationLinkageMode::Closing => {
                value.linked_issue == Some(intent.issue)
                    && value.linkage_source.as_deref() == Some("github_closing_issues_references")
            }
            csdlc_v2::PublicationLinkageMode::PartOf => {
                value.linked_issue.is_none() && value.linkage_source.is_none()
            }
        }
}

fn normalize_ready(
    governed: &PublicationEvidence,
    pr: &octocrab::models::pulls::PullRequest,
) -> csdlc_v2::Result<RemotePullRequest> {
    let base = pr.base.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no base identity",
        )
    })?;
    let head = pr.head.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no head identity",
        )
    })?;
    validate_ready_repository_identity(
        governed,
        base.repo
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref()),
        head.repo
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref()),
    )?;
    Ok(RemotePullRequest {
        number: pr_number(pr)?,
        url: pr
            .html_url
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_else(|| governed.url.clone()),
        repository: governed.repository.clone(),
        base: base.ref_field.clone(),
        head: head.ref_field.clone(),
        title: pr.title.clone().unwrap_or_default(),
        body: pr.body.clone().unwrap_or_default(),
        linkage_mode: governed.linkage_mode.unwrap_or_default(),
        draft: pr.draft.unwrap_or(false),
        state: normalized_remote_state(pr).into(),
        head_sha: head.sha.clone(),
        linked_issue: None,
        linkage_source: None,
    })
}

fn validate_ready_repository_identity(
    governed: &PublicationEvidence,
    base_repository: Option<&str>,
    head_repository: Option<&str>,
) -> csdlc_v2::Result<()> {
    if base_repository != Some(governed.repository.as_str())
        || head_repository != Some(governed.repository.as_str())
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR base or head repository differs from governed publication",
        ));
    }
    Ok(())
}

async fn ready_node_id(
    request: &ReadyPublicationRequest,
    observed: &RemotePullRequest,
    crab: &octocrab::Octocrab,
) -> csdlc_v2::Result<String> {
    let (owner, repo) = repository_parts(
        request
            .code_repository
            .as_deref()
            .unwrap_or(&request.repository),
    )?;
    let pull = crab
        .pulls(owner, repo)
        .get(observed.number)
        .await
        .map_err(|error| remote(error.to_string()))?;
    pull.node_id.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "draft PR node id is missing",
        )
    })
}

fn resolve_token(request: &PublicationRequest) -> csdlc_v2::Result<String> {
    csdlc_v2::github_token::resolve(request.token_file.as_deref())
}

fn repository_parts(repository: &str) -> csdlc_v2::Result<(&str, &str)> {
    repository
        .split_once('/')
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))
}

fn validate_metadata_followup_remote(
    root: &Path,
    intent: &PublicationIntent,
    remote: &RemotePullRequest,
    metadata_head: &str,
) -> csdlc_v2::Result<()> {
    if remote.repository != intent.repository
        || remote.base != intent.base
        || remote.head != intent.head
        || remote.title != intent.title
        || remote.body != intent.body
        || remote.linkage_mode != intent.linkage_mode
        || remote.draft != intent.draft
        || remote.head_sha != metadata_head
        || !existing_pr_matches_governed_mode(intent, remote)
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "metadata publication PR did not converge to the exact governed follow-up head",
        ));
    }
    csdlc_v2::publication::governed_publication_metadata_followup_paths(
        root,
        intent.issue,
        &intent.commit_sha,
        metadata_head,
    )?;
    Ok(())
}

fn validate_ready_metadata_followup_remote(
    root: &Path,
    governed: &PublicationEvidence,
    request: &ReadyPublicationRequest,
    remote: &RemotePullRequest,
    metadata_head: &str,
) -> csdlc_v2::Result<()> {
    let mut expected = request.clone();
    expected.expected_head_sha = metadata_head.to_owned();
    csdlc_v2::publication::validate_ready_remote(governed, &expected, remote, false)?;
    csdlc_v2::publication::governed_publication_metadata_followup_paths(
        root,
        request.issue,
        &request.expected_head_sha,
        metadata_head,
    )?;
    Ok(())
}

fn push(root: &Path, remote_name: &str, head: &str) -> csdlc_v2::Result<()> {
    csdlc_v2::git::run(
        root,
        &["push", remote_name, &format!("HEAD:refs/heads/{head}")],
    )
    .map(|_| ())
}

fn verify_git_remote(
    root: &Path,
    remote_name: &str,
    intent: &PublicationIntent,
) -> csdlc_v2::Result<()> {
    let fetch_urls = csdlc_v2::git::run(root, &["remote", "get-url", "--all", remote_name])?.stdout;
    let push_urls =
        csdlc_v2::git::run(root, &["remote", "get-url", "--push", "--all", remote_name])?.stdout;
    if !remote_urls_match(&fetch_urls, &intent.repository)
        || !remote_urls_match(&push_urls, &intent.repository)
    {
        return Err(V2Error::new(
            ErrorCode::UnsafeCheckout,
            "configured Git fetch or effective push remote does not match publication repository",
        ));
    }
    csdlc_v2::git::run(
        root,
        &[
            "rev-parse",
            "--verify",
            &format!("refs/remotes/{remote_name}/{}", intent.base),
        ],
    )
    .map_err(|_| {
        V2Error::new(
            ErrorCode::UnsafeCheckout,
            "publication base is not a locally observed remote branch",
        )
    })?;
    Ok(())
}

fn remote_urls_match(value: &str, repository: &str) -> bool {
    let mut urls = value.lines().map(str::trim).filter(|url| !url.is_empty());
    let Some(first) = urls.next() else {
        return false;
    };
    remote_url_matches(first.trim_end_matches(".git"), repository)
        && urls.all(|url| remote_url_matches(url.trim_end_matches(".git"), repository))
}

fn remote_url_matches(value: &str, repository: &str) -> bool {
    if let Some(path) = value.strip_prefix("git@github.com:") {
        return path == repository;
    }
    let expected_path = format!("/{repository}");
    url::Url::parse(value).is_ok_and(|parsed| {
        matches!(parsed.scheme(), "https" | "ssh")
            && parsed.host_str() == Some("github.com")
            && parsed.path() == expected_path
    })
}

async fn find_pr(
    crab: &octocrab::Octocrab,
    intent: &PublicationIntent,
) -> csdlc_v2::Result<Option<octocrab::models::pulls::PullRequest>> {
    let head = format!("{}:{}", owner(intent)?, intent.head);
    let page = crab
        .pulls(owner(intent)?, repo(intent)?)
        .list()
        .state(State::Open)
        .head(head)
        .base(&intent.base)
        .per_page(100)
        .send()
        .await
        .map_err(|e| remote(e.to_string()))?;
    let items = crab
        .all_pages(page)
        .await
        .map_err(|e| remote(e.to_string()))?;
    select_unique(items)
}

async fn reobserve_pr_at_head(
    crab: &octocrab::Octocrab,
    intent: &PublicationIntent,
    expected_head: &str,
) -> csdlc_v2::Result<Option<octocrab::models::pulls::PullRequest>> {
    let mut observed = None;
    for attempt in 0..5 {
        observed = find_pr(crab, intent).await?;
        if observed
            .as_ref()
            .and_then(|pr| pr.head.as_ref())
            .is_some_and(|head| head.sha == expected_head)
        {
            return Ok(observed);
        }
        if attempt < 4 {
            sleep(Duration::from_secs(2)).await;
        }
    }
    Ok(observed)
}

async fn reobserve_ready_pr_at_head(
    crab: &octocrab::Octocrab,
    owner: &str,
    repo: &str,
    pull_request: u64,
    expected_head: &str,
) -> csdlc_v2::Result<octocrab::models::pulls::PullRequest> {
    let mut observed = None;
    for attempt in 0..5 {
        let pull = crab
            .pulls(owner, repo)
            .get(pull_request)
            .await
            .map_err(|error| remote(error.to_string()))?;
        if pull
            .head
            .as_ref()
            .is_some_and(|head| head.sha == expected_head)
        {
            return Ok(pull);
        }
        observed = Some(pull);
        if attempt < 4 {
            sleep(Duration::from_secs(2)).await;
        }
    }
    observed.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "ready metadata publication head could not be reconciled",
        )
    })
}

fn select_unique<T>(mut items: Vec<T>) -> csdlc_v2::Result<Option<T>> {
    if items.len() > 1 {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "multiple matching PRs observed",
        ));
    }
    Ok(items.pop())
}

async fn normalize(
    intent: &PublicationIntent,
    pr: &octocrab::models::pulls::PullRequest,
    crab: &octocrab::Octocrab,
) -> csdlc_v2::Result<RemotePullRequest> {
    let base = pr.base.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no base identity",
        )
    })?;
    let head = pr.head.as_ref().ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no head identity",
        )
    })?;
    validate_observed_repository_identity(
        intent,
        base.repo
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref()),
        head.repo
            .as_ref()
            .and_then(|repo| repo.full_name.as_deref()),
    )?;
    let linked_issue = match intent.linkage_mode {
        csdlc_v2::PublicationLinkageMode::Closing => {
            Some(observe_closing_issue(crab, intent, pr_number(pr)?).await?)
        }
        csdlc_v2::PublicationLinkageMode::PartOf => None,
    };
    Ok(RemotePullRequest {
        number: pr_number(pr)?,
        url: pr
            .html_url
            .as_ref()
            .map(ToString::to_string)
            .unwrap_or_default(),
        repository: intent.repository.clone(),
        base: base.ref_field.clone(),
        head: head.ref_field.clone(),
        title: pr.title.clone().unwrap_or_default(),
        body: pr.body.clone().unwrap_or_default(),
        linkage_mode: intent.linkage_mode,
        draft: pr.draft.unwrap_or(false),
        state: normalized_remote_state(pr).into(),
        head_sha: head.sha.clone(),
        linked_issue,
        linkage_source: linked_issue.map(|_| "github_closing_issues_references".into()),
    })
}

async fn observe_closing_issue(
    crab: &octocrab::Octocrab,
    intent: &PublicationIntent,
    pull_request: u64,
) -> csdlc_v2::Result<u64> {
    let response: serde_json::Value = crab
        .graphql(&serde_json::json!({
            "query": "query ClosingIssues($owner: String!, $repo: String!, $number: Int!) { repository(owner: $owner, name: $repo) { pullRequest(number: $number) { closingIssuesReferences(first: 100) { nodes { number repository { nameWithOwner } } } } } }",
            "variables": {"owner": owner(intent)?, "repo": repo(intent)?, "number": pull_request}
        }))
        .await
        .map_err(|error| remote(error.to_string()))?;
    let nodes = response
        .pointer("/repository/pullRequest/closingIssuesReferences/nodes")
        .or_else(|| response.pointer("/data/repository/pullRequest/closingIssuesReferences/nodes"))
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| {
            V2Error::new(
                ErrorCode::ReconciliationRequired,
                "GitHub closing-issue relation is absent; wait for the dependency branch to merge, publish the closing PR against the default branch, or choose the explicit non-closing checkpoint route",
            )
        })?;
    if nodes.iter().any(|node| {
        node.get("number").and_then(serde_json::Value::as_u64) == Some(intent.issue)
            && node
                .pointer("/repository/nameWithOwner")
                .and_then(serde_json::Value::as_str)
                == Some(intent.issue_repository.as_str())
    }) {
        return Ok(intent.issue);
    }
    Err(V2Error::new(
        ErrorCode::ReconciliationRequired,
        "caller-linked issue is not a remote GitHub closing relation; wait for the dependency branch to merge, publish the closing PR against the default branch, or choose the explicit non-closing checkpoint route",
    ))
}

fn normalized_remote_state(pr: &octocrab::models::pulls::PullRequest) -> &'static str {
    if pr.merged == Some(true) {
        "merged"
    } else {
        match pr.state {
            Some(IssueState::Open) => "open",
            Some(IssueState::Closed) => "closed",
            _ => "unknown",
        }
    }
}

fn github_client(token: String) -> csdlc_v2::Result<octocrab::Octocrab> {
    let mut builder = octocrab::Octocrab::builder().personal_token(token);
    #[cfg(debug_assertions)]
    if let Some(base) = std::env::var_os("CSDLC_V2_TEST_GITHUB_API_BASE") {
        let base = base.to_string_lossy();
        let parsed = url::Url::parse(&base)
            .map_err(|_| V2Error::new(ErrorCode::InvalidInput, "test API base is invalid"))?;
        let loopback = match parsed.host() {
            Some(url::Host::Domain("localhost")) => true,
            Some(url::Host::Ipv4(address)) => address.is_loopback(),
            Some(url::Host::Ipv6(address)) => address.is_loopback(),
            _ => false,
        };
        if parsed.scheme() != "http" || !loopback || parsed.path() != "/" {
            return Err(V2Error::new(
                ErrorCode::InvalidInput,
                "test API base must be an HTTP loopback origin",
            ));
        }
        builder = builder
            .base_uri(base.as_ref())
            .map_err(|error| remote(error.to_string()))?;
    }
    builder.build().map_err(|error| remote(error.to_string()))
}

fn validate_observed_repository_identity(
    intent: &PublicationIntent,
    base_repository: Option<&str>,
    head_repository: Option<&str>,
) -> csdlc_v2::Result<()> {
    if base_repository != Some(intent.repository.as_str())
        || head_repository != Some(intent.repository.as_str())
    {
        return Err(V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR base or head repository differs from publication intent",
        ));
    }
    Ok(())
}
fn pr_number(pr: &octocrab::models::pulls::PullRequest) -> csdlc_v2::Result<u64> {
    pr.number.ok_or_else(|| {
        V2Error::new(
            ErrorCode::ReconciliationRequired,
            "observed PR has no number",
        )
    })
}
fn owner(intent: &PublicationIntent) -> csdlc_v2::Result<&str> {
    intent
        .repository
        .split_once('/')
        .map(|v| v.0)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))
}
fn repo(intent: &PublicationIntent) -> csdlc_v2::Result<&str> {
    intent
        .repository
        .split_once('/')
        .map(|v| v.1)
        .ok_or_else(|| V2Error::new(ErrorCode::InvalidInput, "repository must be owner/name"))
}
fn remote(message: String) -> V2Error {
    V2Error::new(
        ErrorCode::RemoteFailure,
        format!("GitHub operation failed: {message}"),
    )
}

fn reconcile_create_observation(send_failed: bool, observed: bool) -> csdlc_v2::Result<()> {
    if observed {
        return Ok(());
    }
    let message = if send_failed {
        "create outcome is ambiguous; no matching PR observed"
    } else {
        "created PR could not be reconciled"
    };
    Err(V2Error::new(ErrorCode::ReconciliationRequired, message))
}

#[cfg(test)]
mod tests {
    use super::{
        existing_pr_matches_governed_mode, reconcile_create_observation, remote_url_matches,
        remote_urls_match, select_unique, validate_observed_repository_identity, verify_git_remote,
    };
    use csdlc_v2::{ErrorCode, PublicationIntent, PublicationLinkageMode, RemotePullRequest};
    use std::process::Command;

    fn git(root: &std::path::Path, args: &[&str]) {
        assert!(Command::new("git")
            .args(args)
            .current_dir(root)
            .status()
            .expect("run git")
            .success());
    }

    #[test]
    fn remote_url_requires_exact_github_host_and_repository() {
        let repo = "agent-logic/agent-design-language";
        assert!(remote_url_matches(
            "https://github.com/agent-logic/agent-design-language",
            repo
        ));
        assert!(remote_url_matches(
            "git@github.com:agent-logic/agent-design-language",
            repo
        ));
        assert!(!remote_url_matches(
            "https://evilgithub.com/agent-logic/agent-design-language",
            repo
        ));
        assert!(!remote_url_matches(
            "https://github.com/other/agent-design-language",
            repo
        ));
    }

    #[test]
    fn every_effective_fetch_and_push_url_must_match_the_code_repository() {
        let repo = "agent-logic/agent-design-language";
        assert!(remote_urls_match(
            "https://github.com/agent-logic/agent-design-language.git\n",
            repo
        ));
        assert!(remote_urls_match(
            "git@github.com:agent-logic/agent-design-language.git\nhttps://github.com/agent-logic/agent-design-language.git\n",
            repo
        ));
        assert!(!remote_urls_match("", repo));
        assert!(!remote_urls_match(
            "https://github.com/agent-logic/agent-design-language.git\nhttps://github.com/danielbaustin/agent-design-language.git\n",
            repo
        ));
    }

    #[test]
    fn substituted_pushurl_is_rejected_before_publication_push() {
        let temp = tempfile::tempdir().expect("temp repo");
        git(temp.path(), &["init", "-b", "main"]);
        git(
            temp.path(),
            &["config", "user.email", "test@example.invalid"],
        );
        git(temp.path(), &["config", "user.name", "C-SDLC Test"]);
        git(temp.path(), &["commit", "--allow-empty", "-m", "base"]);
        git(
            temp.path(),
            &[
                "remote",
                "add",
                "origin",
                "https://github.com/agent-logic/agent-design-language.git",
            ],
        );
        git(
            temp.path(),
            &["update-ref", "refs/remotes/origin/main", "HEAD"],
        );

        let intent = PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue: 3,
            repository: "agent-logic/agent-design-language".into(),
            issue_repository: "danielbaustin/agent-design-language".into(),
            base: "main".into(),
            head: "codex/3".into(),
            title: "title".into(),
            body: "Closes danielbaustin/agent-design-language#3".into(),
            linkage_mode: PublicationLinkageMode::Closing,
            draft: false,
            revision: "revision".into(),
            commit_sha: "sha".into(),
        };
        assert!(verify_git_remote(temp.path(), "origin", &intent).is_ok());

        git(
            temp.path(),
            &[
                "remote",
                "set-url",
                "--push",
                "origin",
                "https://github.com/danielbaustin/agent-design-language.git",
            ],
        );
        assert!(verify_git_remote(temp.path(), "origin", &intent).is_err());
    }

    #[test]
    fn exhaustive_pr_results_must_be_unique() {
        assert_eq!(select_unique::<u8>(vec![]).unwrap(), None);
        assert_eq!(select_unique(vec![7_u8]).unwrap(), Some(7));
        assert!(select_unique(vec![7_u8, 8_u8]).is_err());
    }

    #[test]
    fn ambiguous_create_failure_observes_before_deciding_retry() {
        assert!(reconcile_create_observation(true, true).is_ok());
        assert!(reconcile_create_observation(true, false).is_err());
        assert!(reconcile_create_observation(false, false).is_err());
    }

    #[test]
    fn normalization_rejects_fork_or_missing_repository_identity() {
        let intent = PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue: 5466,
            repository: "owner/repo".into(),
            issue_repository: "owner/repo".into(),
            base: "main".into(),
            head: "codex/5466".into(),
            title: "title".into(),
            body: "Resolves #5466".into(),
            linkage_mode: PublicationLinkageMode::Closing,
            draft: false,
            revision: "revision".into(),
            commit_sha: "sha".into(),
        };
        assert!(validate_observed_repository_identity(
            &intent,
            Some("owner/repo"),
            Some("owner/repo")
        )
        .is_ok());
        assert!(validate_observed_repository_identity(
            &intent,
            Some("owner/repo"),
            Some("fork/repo")
        )
        .is_err());
        assert!(validate_observed_repository_identity(&intent, None, Some("owner/repo")).is_err());
        assert!(validate_observed_repository_identity(&intent, Some("owner/repo"), None).is_err());
    }

    #[test]
    fn existing_split_authority_pr_requires_qualified_issue_linkage() {
        let intent = PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue: 5901,
            repository: "agent-logic/agent-design-language".into(),
            issue_repository: "danielbaustin/agent-design-language".into(),
            base: "main".into(),
            head: "codex/5901".into(),
            title: "title".into(),
            body: "Closes danielbaustin/agent-design-language#5901".into(),
            linkage_mode: PublicationLinkageMode::Closing,
            draft: false,
            revision: "revision".into(),
            commit_sha: "sha".into(),
        };
        let mut remote = RemotePullRequest {
            number: 1,
            url: "https://example.invalid/1".into(),
            repository: intent.repository.clone(),
            base: intent.base.clone(),
            head: intent.head.clone(),
            title: intent.title.clone(),
            body: intent.body.clone(),
            linkage_mode: intent.linkage_mode,
            draft: false,
            state: "open".into(),
            head_sha: intent.commit_sha.clone(),
            linked_issue: Some(5901),
            linkage_source: Some("github_closing_issues_references".into()),
        };
        assert!(existing_pr_matches_governed_mode(&intent, &remote));
        remote.body = "Closes #5901".into();
        assert!(!existing_pr_matches_governed_mode(&intent, &remote));
    }

    #[test]
    fn stacked_closing_publication_without_remote_relation_fails_closed() {
        let intent = PublicationIntent {
            schema: "csdlc.publication_intent.v1".into(),
            issue: 631,
            repository: "agent-logic/agent-design-language".into(),
            issue_repository: "agent-logic/agent-design-language".into(),
            base: "codex/627-v3-h1-command-denominator-r2".into(),
            head: "codex/631-v3-h5-proof-parity-install-impl".into(),
            title: "title".into(),
            body: "Closes #631".into(),
            linkage_mode: PublicationLinkageMode::Closing,
            draft: false,
            revision: "revision".into(),
            commit_sha: "85c19e5e230819c89f3f699258c1ad9062cad96d".into(),
        };
        let remote = RemotePullRequest {
            number: 644,
            url: "https://github.com/agent-logic/agent-design-language/pull/644".into(),
            repository: intent.repository.clone(),
            base: intent.base.clone(),
            head: intent.head.clone(),
            title: intent.title.clone(),
            body: intent.body.clone(),
            linkage_mode: intent.linkage_mode,
            draft: false,
            state: "open".into(),
            head_sha: intent.commit_sha.clone(),
            linked_issue: None,
            linkage_source: None,
        };

        assert!(!existing_pr_matches_governed_mode(&intent, &remote));
        let error = csdlc_v2::publication::validate_remote(&intent, &remote)
            .expect_err("body-only stacked closing PR must fail closed");
        assert_eq!(error.code, ErrorCode::ReconciliationRequired);
    }
}
