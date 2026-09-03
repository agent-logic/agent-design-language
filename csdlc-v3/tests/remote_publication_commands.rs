use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use csdlc_v3::adapters::{FakeProcessAdapter, ProcessOutput, ProcessStatus};
use csdlc_v3::commands::remote::{
    github_adapter_receipt_payload_digest, github_readback_receipt_payload_digest,
    load_remote_route_receipts, observe_github_pr_readback, prepare_remote_publication_route,
    prepare_remote_publication_route_with_receipts, typed_review_receipt_payload_digest,
    GithubAdapterReceipt, GithubReadbackReceipt, RemotePublicationMode, RemoteReadbackSource,
    RemoteRouteReceipts, RemoteRouteRequest, RemoteRouteStatus, TypedReviewReceipt,
    REMOTE_PUBLICATION_ROUTE_NAMES,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn git_common_dir() -> PathBuf {
    let output = Command::new("git")
        .arg("-C")
        .arg(repo_root())
        .arg("rev-parse")
        .arg("--git-common-dir")
        .output()
        .expect("git common dir should be discoverable");
    assert!(output.status.success(), "git common dir failed: {output:?}");
    let path = PathBuf::from(
        String::from_utf8(output.stdout)
            .expect("git common dir should be utf8")
            .trim(),
    );
    if path.is_absolute() {
        path
    } else {
        repo_root().join(path)
    }
}

fn fixture_dir(name: &str) -> PathBuf {
    let dir = git_common_dir()
        .join("csdlc-v3/test-fixtures/629/remote-publication")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("fixture directory");
    dir
}

fn request() -> (RemoteRouteRequest, RemoteRouteReceipts) {
    let mut request = RemoteRouteRequest {
        repository: "agent-logic/agent-design-language".into(),
        issue: 629,
        pull_request: Some(639),
        actor: Some("worker-6".into()),
        implementer: Some("worker-6".into()),
        reviewer: Some("reviewer-629".into()),
        review_revision: Some("abc123".into()),
        expected_head_sha: Some("abc123".into()),
        head_sha: Some("abc123".into()),
        mode: Some(RemotePublicationMode::Closing),
        body: Some("Closes #629\n\nPart of #625".into()),
        review_present: true,
        typed_review_receipt_path: None,
        typed_review_receipt_digest: None,
        readback_source: Some(RemoteReadbackSource::Github),
        readback_receipt_path: None,
        readback_receipt_digest: None,
        adapter_receipt_path: None,
        adapter_receipt_digest: None,
        closes_issue: Some(629),
        part_of_issue: None,
        credential_names: vec!["GITHUB_TOKEN".into()],
    };
    let receipts = receipts_for(&request);
    request.typed_review_receipt_digest = receipts
        .typed_review
        .as_ref()
        .map(typed_review_receipt_payload_digest);
    request.readback_receipt_digest = receipts
        .github_readback
        .as_ref()
        .map(github_readback_receipt_payload_digest);
    request.adapter_receipt_digest = receipts
        .adapter
        .as_ref()
        .map(github_adapter_receipt_payload_digest);
    (request, receipts)
}

fn receipts_for(request: &RemoteRouteRequest) -> RemoteRouteReceipts {
    let readback = GithubReadbackReceipt {
        schema: "csdlc.v3.github_readback_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request.expect("pull request"),
        head_sha: request.head_sha.clone().expect("head sha"),
        closes_issue: request.closes_issue,
        part_of_issue: request.part_of_issue,
        source: RemoteReadbackSource::Github,
        observed_by: "github-api-read-only".into(),
    };
    let adapter = GithubAdapterReceipt {
        schema: "csdlc.v3.github_adapter_receipt.v1".into(),
        repository: request.repository.clone(),
        issue: request.issue,
        pull_request: request.pull_request.expect("pull request"),
        head_sha: request.head_sha.clone().expect("head sha"),
        readback_receipt_digest: github_readback_receipt_payload_digest(&readback),
        credential_names: request.credential_names.clone(),
        adapter: "github-api-read-only".into(),
        authenticated: true,
    };
    RemoteRouteReceipts {
        typed_review: Some(TypedReviewReceipt {
            schema: "csdlc.v3.typed_review_receipt.v1".into(),
            repository: request.repository.clone(),
            issue: request.issue,
            implementer: request.implementer.clone().expect("implementer"),
            reviewer: request.reviewer.clone().expect("reviewer"),
            reviewed_revision: request.review_revision.clone().expect("review revision"),
            expected_head_sha: request.expected_head_sha.clone().expect("expected head"),
            evidence_digest: "typed-review-evidence-digest".into(),
        }),
        github_readback: Some(readback),
        adapter: Some(adapter),
    }
}

fn write_receipts(dir: &Path, request: &mut RemoteRouteRequest, receipts: &RemoteRouteReceipts) {
    let typed_path = dir.join("typed-review-receipt.json");
    let readback_path = dir.join("github-readback-receipt.json");
    let adapter_path = dir.join("github-adapter-receipt.json");
    fs::write(
        &typed_path,
        serde_json::to_vec_pretty(receipts.typed_review.as_ref().expect("typed receipt"))
            .expect("typed receipt json"),
    )
    .expect("write typed receipt");
    fs::write(
        &readback_path,
        serde_json::to_vec_pretty(receipts.github_readback.as_ref().expect("readback receipt"))
            .expect("readback receipt json"),
    )
    .expect("write readback receipt");
    fs::write(
        &adapter_path,
        serde_json::to_vec_pretty(receipts.adapter.as_ref().expect("adapter receipt"))
            .expect("adapter receipt json"),
    )
    .expect("write adapter receipt");
    request.typed_review_receipt_path = Some(typed_path.to_string_lossy().into());
    request.readback_receipt_path = Some(readback_path.to_string_lossy().into());
    request.adapter_receipt_path = Some(adapter_path.to_string_lossy().into());
}

#[test]
fn remote_publication_routes_are_typed_and_non_authoritative() {
    let dir = fixture_dir("cli");
    let request_path = dir.join("request.json");
    let (mut request, receipts) = request();
    write_receipts(&dir, &mut request, &receipts);
    fs::write(
        &request_path,
        serde_json::to_vec(&request).expect("request json"),
    )
    .expect("write request fixture");

    for route in REMOTE_PUBLICATION_ROUTE_NAMES {
        let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
            .arg(route)
            .arg("--request")
            .arg(&request_path)
            .output()
            .unwrap_or_else(|error| panic!("run {route}: {error}"));
        assert!(output.status.success(), "{route} failed: {output:?}");
        assert!(output.stderr.is_empty(), "{route} stderr should be empty");
        let value: serde_json::Value =
            serde_json::from_slice(&output.stdout).expect("machine-readable JSON");
        assert_eq!(value["schema"], "csdlc.v3.remote_publication.v1");
        assert_eq!(value["command"], route);
        assert_eq!(value["read_only"], true);
        assert_eq!(value["operational_authority"], false);
        assert_eq!(value["cutover_issue"], 505);
        assert_eq!(value["result"]["issue"], 629);
        assert_eq!(
            value["result"]["redacted_credentials"][0],
            "GITHUB_TOKEN=<redacted>"
        );
        let findings = value["result"]["findings"]
            .as_array()
            .expect("findings array");
        assert_eq!(value["result"]["status"], "ready");
        assert!(findings.is_empty(), "{route} should be ready: {findings:?}");
        assert!(!String::from_utf8_lossy(&output.stdout).contains("secret"));
    }
}

#[test]
fn authenticated_github_observation_builds_readback_receipts_without_trusting_caller_json() {
    let (mut request, _) = request();
    request.head_sha = Some("caller-forged-head".into());
    request.closes_issue = None;
    request.readback_receipt_digest = None;
    request.adapter_receipt_digest = None;

    let mut process = FakeProcessAdapter::new(ProcessOutput {
        status: ProcessStatus::Exit(0),
        stdout: serde_json::json!({
            "number": 639,
            "head": {"sha": "observed-github-head"},
            "merged": false,
            "body": "Closes #629\n\nPart of #625"
        })
        .to_string(),
        stderr: String::new(),
        truncated: false,
    });
    let observed =
        observe_github_pr_readback(&request, &mut process).expect("GitHub observation succeeds");
    assert_eq!(process.invocations().len(), 1);
    assert_eq!(observed.invocation.program, "github-api-read-only");
    assert_eq!(
        observed.invocation.child_credential_name(),
        Some("GITHUB_TOKEN")
    );
    assert_eq!(
        observed.invocation.argv(),
        ["pull-request", "agent-logic/agent-design-language", "639"]
    );
    assert_eq!(
        observed.request.head_sha.as_deref(),
        Some("observed-github-head")
    );
    assert_eq!(observed.request.closes_issue, Some(629));
    assert_eq!(observed.request.part_of_issue, None);

    let mut receipts = receipts_for(&observed.request);
    receipts.github_readback = observed.receipts.github_readback;
    receipts.adapter = observed.receipts.adapter;
    receipts.typed_review = receipts_for(&observed.request).typed_review;
    let mut ready = observed.request;
    ready.typed_review_receipt_digest = receipts
        .typed_review
        .as_ref()
        .map(typed_review_receipt_payload_digest);
    let plan = prepare_remote_publication_route_with_receipts("pr-state", &ready, &receipts)
        .expect("observed readback plans");
    assert_eq!(plan.status, RemoteRouteStatus::Ready);
}

#[test]
fn cli_receipt_loader_rejects_disposable_target_receipts() {
    let dir = repo_root().join("csdlc-v3/target/remote-publication-fixtures/disposable-receipts");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("disposable fixture directory");
    let (mut request, receipts) = request();
    write_receipts(&dir, &mut request, &receipts);

    let finding = load_remote_route_receipts(&repo_root(), &request)
        .expect_err("target receipts are not durable publication evidence");
    assert_eq!(finding.code, "receipt_path_not_durable");
}

#[test]
fn cli_observe_github_fails_before_network_without_an_explicit_credential_name() {
    let dir = fixture_dir("observe-missing-credential");
    let request_path = dir.join("request.json");
    let (mut request, _) = request();
    request.credential_names.clear();
    fs::write(
        &request_path,
        serde_json::to_vec(&request).expect("request json"),
    )
    .expect("write request fixture");

    let output = Command::new(env!("CARGO_BIN_EXE_csdlc"))
        .arg("pr-state")
        .arg("--request")
        .arg(&request_path)
        .arg("--observe-github")
        .output()
        .expect("run pr-state observation preflight");
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("github_credential_missing"), "{stderr}");
}

#[test]
fn publish_route_requires_current_review_and_closing_relation() {
    let (mut missing_review, receipts) = request();
    missing_review.review_present = false;
    missing_review.typed_review_receipt_digest = None;
    let plan =
        prepare_remote_publication_route_with_receipts("publish", &missing_review, &receipts)
            .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_review_truth"));
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "authenticated_review_receipt_missing"));

    let (mut stale, receipts) = request();
    stale.head_sha = Some("different".into());
    let plan =
        prepare_remote_publication_route_with_receipts("publish", &stale, &receipts).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "stale_review_truth"));

    let (mut missing_revision, receipts) = request();
    missing_revision.expected_head_sha = None;
    missing_revision.head_sha = None;
    let plan =
        prepare_remote_publication_route_with_receipts("publish", &missing_revision, &receipts)
            .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_review_revision"));

    let (mut related_only, receipts) = request();
    related_only.body = Some("Related #629".into());
    let plan = prepare_remote_publication_route_with_receipts("publish", &related_only, &receipts)
        .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_closing_relation"));

    let (mut missing_readback, mut receipts) = request();
    missing_readback.readback_receipt_digest = None;
    missing_readback.adapter_receipt_digest = None;
    receipts.github_readback = None;
    receipts.adapter = None;
    let plan =
        prepare_remote_publication_route_with_receipts("publish", &missing_readback, &receipts)
            .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "github_readback_receipt_missing"));
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "authenticated_github_adapter_missing"));

    let (mut wrong_issue, receipts) = request();
    wrong_issue.body = Some("Closes #6290".into());
    let plan = prepare_remote_publication_route_with_receipts("publish", &wrong_issue, &receipts)
        .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_closing_relation"));
}

#[test]
fn pr_state_route_rejects_caller_forged_readback() {
    let (self_consistent_forgery, receipts) = request();
    for route in ["github", "github-issue", "pr-state", "github-pr"] {
        let plan = prepare_remote_publication_route(route, &self_consistent_forgery)
            .expect("plan rejects caller-only receipt claims");
        assert_eq!(plan.status, RemoteRouteStatus::Blocked);
        assert!(plan
            .findings
            .iter()
            .any(|finding| finding.code == "github_readback_receipt_missing"));
    }

    for route in ["github", "github-issue", "pr-state", "github-pr"] {
        let plan = prepare_remote_publication_route_with_receipts(
            route,
            &self_consistent_forgery,
            &receipts,
        )
        .expect("plan accepts production-identity authenticated GitHub receipts");
        assert_eq!(plan.status, RemoteRouteStatus::Ready);
        assert!(plan.findings.is_empty());
    }

    let (synthetic, mut synthetic_receipts) = request();
    synthetic_receipts
        .github_readback
        .as_mut()
        .expect("readback receipt")
        .observed_by = "synthetic-test-fixture".into();
    synthetic_receipts
        .adapter
        .as_mut()
        .expect("adapter receipt")
        .adapter = "github".into();
    let plan =
        prepare_remote_publication_route_with_receipts("pr-state", &synthetic, &synthetic_receipts)
            .expect("plan rejects non-production adapter identity");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "github_readback_receipt_missing"));

    let (mut forged, receipts) = request();
    forged.readback_source = Some(RemoteReadbackSource::Caller);
    let plan = prepare_remote_publication_route_with_receipts("pr-state", &forged, &receipts)
        .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "caller_forged_readback"));

    let (mut missing_adapter, receipts) = request();
    missing_adapter.adapter_receipt_digest = None;
    let plan =
        prepare_remote_publication_route_with_receipts("pr-state", &missing_adapter, &receipts)
            .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "authenticated_github_adapter_missing"));

    let (mut stale_readback, receipts) = request();
    stale_readback.head_sha = Some("different-readback-head".into());
    let plan =
        prepare_remote_publication_route_with_receipts("pr-state", &stale_readback, &receipts)
            .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "github_readback_receipt_missing"));

    let (mut missing_linkage, receipts) = request();
    missing_linkage.closes_issue = None;
    let plan =
        prepare_remote_publication_route_with_receipts("github-pr", &missing_linkage, &receipts)
            .expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_closing_readback"));
}

#[test]
fn review_route_rejects_self_review_even_with_whitespace() {
    let (mut self_review, _receipts) = request();
    self_review.reviewer = Some(" worker-6 ".into());
    let plan = prepare_remote_publication_route("review", &self_review).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "self_review_denied"));
}
