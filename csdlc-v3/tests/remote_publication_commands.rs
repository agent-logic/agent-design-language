use std::{fs, path::PathBuf, process::Command};

use csdlc_v3::commands::remote::{
    prepare_remote_publication_route, RemotePublicationMode, RemoteReadbackSource,
    RemoteRouteRequest, RemoteRouteStatus, REMOTE_PUBLICATION_ROUTE_NAMES,
};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn fixture_dir(name: &str) -> PathBuf {
    let dir = repo_root()
        .join("csdlc-v3/target/remote-publication-fixtures")
        .join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).expect("fixture directory");
    dir
}

fn request() -> RemoteRouteRequest {
    RemoteRouteRequest {
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
        readback_source: Some(RemoteReadbackSource::Github),
        closes_issue: Some(629),
        part_of_issue: None,
        credential_names: vec!["GITHUB_TOKEN".into()],
    }
}

#[test]
fn remote_publication_routes_are_typed_and_non_authoritative() {
    let dir = fixture_dir("cli");
    let request_path = dir.join("request.json");
    fs::write(
        &request_path,
        serde_json::to_vec(&request()).expect("request json"),
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
        assert!(!String::from_utf8_lossy(&output.stdout).contains("secret"));
    }
}

#[test]
fn publish_route_requires_current_review_and_closing_relation() {
    let mut missing_review = request();
    missing_review.review_present = false;
    let plan = prepare_remote_publication_route("publish", &missing_review).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_review_truth"));

    let mut stale = request();
    stale.head_sha = Some("different".into());
    let plan = prepare_remote_publication_route("publish", &stale).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "stale_review_truth"));

    let mut related_only = request();
    related_only.body = Some("Related #629".into());
    let plan = prepare_remote_publication_route("publish", &related_only).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_closing_relation"));
}

#[test]
fn pr_state_route_rejects_caller_forged_readback() {
    let mut forged = request();
    forged.readback_source = Some(RemoteReadbackSource::Caller);
    let plan = prepare_remote_publication_route("pr-state", &forged).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "caller_forged_readback"));

    let mut missing_linkage = request();
    missing_linkage.closes_issue = None;
    let plan = prepare_remote_publication_route("github-pr", &missing_linkage).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "missing_closing_readback"));
}

#[test]
fn review_route_rejects_self_review_even_with_whitespace() {
    let mut self_review = request();
    self_review.reviewer = Some(" worker-6 ".into());
    let plan = prepare_remote_publication_route("review", &self_review).expect("plan");
    assert_eq!(plan.status, RemoteRouteStatus::Blocked);
    assert!(plan
        .findings
        .iter()
        .any(|finding| finding.code == "self_review_denied"));
}
