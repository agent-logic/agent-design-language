use csdlc_v2::{
    publication::{
        body_has_github_closing_keyword, body_has_github_part_of_reference,
        body_has_qualified_github_closing_keyword, body_has_qualified_github_part_of_reference,
        validate_remote,
    },
    reconcile_action, PublicationAction, PublicationEvidence, PublicationIntent,
    PublicationLinkageMode, PublicationRequest, RemotePullRequest,
};

fn intent() -> PublicationIntent {
    PublicationIntent {
        schema: "csdlc.publication_intent.v1".into(),
        issue: 5236,
        repository: "agent-logic/agent-design-language".into(),
        issue_repository: "agent-logic/agent-design-language".into(),
        base: "main".into(),
        head: "codex/5236".into(),
        title: "Gate 6".into(),
        body: "Closes #5236".into(),
        linkage_mode: PublicationLinkageMode::Closing,
        draft: true,
        revision: "revision".into(),
        commit_sha: "abc123".into(),
    }
}

#[test]
fn canonical_code_pr_can_close_a_legacy_issue_with_qualified_linkage() {
    let mut intent = intent();
    intent.issue_repository = "danielbaustin/agent-design-language".into();
    intent.body = "Closes danielbaustin/agent-design-language#5236".into();
    let mut remote = remote();
    remote.body = intent.body.clone();
    assert!(validate_remote(&intent, &remote).is_ok());

    remote.body = "Closes #5236".into();
    assert!(validate_remote(&intent, &remote).is_err());
    assert!(!body_has_qualified_github_closing_keyword(
        &remote.body,
        5236,
        "danielbaustin/agent-design-language"
    ));
}

fn remote() -> RemotePullRequest {
    let intent = intent();
    RemotePullRequest {
        number: 7,
        url: "https://example.invalid/pr/7".into(),
        repository: intent.repository,
        base: intent.base,
        head: intent.head,
        title: intent.title,
        body: intent.body,
        linkage_mode: intent.linkage_mode,
        draft: intent.draft,
        state: "open".into(),
        head_sha: intent.commit_sha,
        linked_issue: Some(intent.issue),
        linkage_source: Some("github_closing_issues_references".into()),
    }
}

#[test]
fn ambiguous_create_outage_is_reconciled_by_observation_before_retry() {
    let intent = intent();
    assert_eq!(
        reconcile_action(&intent, None).unwrap(),
        PublicationAction::Create
    );
    assert_eq!(
        reconcile_action(&intent, Some(&remote())).unwrap(),
        PublicationAction::Noop
    );
}

#[test]
fn drifted_mutable_fields_update_same_pr() {
    let intent = intent();
    let mut remote = remote();
    remote.body = "Closes #5236\nOld text".into();
    assert_eq!(
        reconcile_action(&intent, Some(&remote)).unwrap(),
        PublicationAction::Update
    );
}

#[test]
fn base_head_or_repository_mismatch_fails_closed() {
    let intent = intent();
    for field in 0..3 {
        let mut remote = remote();
        match field {
            0 => remote.base = "release".into(),
            1 => remote.head = "wrong".into(),
            _ => remote.repository = "other/repo".into(),
        };
        assert!(validate_remote(&intent, &remote).is_err());
    }
}

#[test]
fn publication_body_requires_github_closing_keyword_for_issue() {
    for body in [
        "Closes #5236",
        "fixes #5236",
        "Resolved #5236",
        "Closes: agent-logic/agent-design-language#5236",
    ] {
        assert!(body_has_github_closing_keyword(
            body,
            5236,
            "agent-logic/agent-design-language"
        ));
    }
    for body in [
        "Related #5236",
        "See #5236",
        "Closes #52360",
        "Closes issue 5236",
        "Close\n#5236",
        "Closes wrong/repo#5236",
    ] {
        assert!(!body_has_github_closing_keyword(
            body,
            5236,
            "agent-logic/agent-design-language"
        ));
    }
}

#[test]
fn part_of_requires_one_exact_non_closing_reference() {
    assert!(body_has_github_part_of_reference(
        "Part of #5236",
        5236,
        "agent-logic/agent-design-language"
    ));
    for body in [
        "Part of issue #5236",
        "This is part of #5236",
        "Part of #52360",
        "Related #5236",
        "Part\nof #5236",
    ] {
        assert!(!body_has_github_part_of_reference(
            body,
            5236,
            "agent-logic/agent-design-language"
        ));
    }
}

#[test]
fn split_authority_part_of_requires_qualified_reference() {
    assert!(body_has_qualified_github_part_of_reference(
        "Part of danielbaustin/agent-design-language#5236",
        5236,
        "danielbaustin/agent-design-language"
    ));
    assert!(!body_has_qualified_github_part_of_reference(
        "Part of #5236",
        5236,
        "danielbaustin/agent-design-language"
    ));
}

#[test]
fn remote_part_of_mode_is_retained_and_mixed_linkage_fails_closed() {
    let mut intent = intent();
    intent.linkage_mode = PublicationLinkageMode::PartOf;
    intent.body = "Part of #5236".into();
    let mut remote = remote();
    remote.linkage_mode = PublicationLinkageMode::PartOf;
    remote.body = intent.body.clone();
    remote.linked_issue = None;
    remote.linkage_source = None;
    assert!(validate_remote(&intent, &remote).is_ok());

    remote.body = "Part of #5236\n\nCloses #5236".into();
    assert!(validate_remote(&intent, &remote).is_err());
    remote.body = "Part of #5236".into();
    remote.linkage_mode = PublicationLinkageMode::Closing;
    assert!(validate_remote(&intent, &remote).is_err());
}

#[test]
fn omitted_request_linkage_mode_defaults_to_closing() {
    let request: PublicationRequest = serde_json::from_value(serde_json::json!({
        "schema": "csdlc.publication_request.v1",
        "issue": 5236,
        "expected_generation": 1,
        "expected_digest": "digest",
        "actor": "operator",
        "repository": "agent-logic/agent-design-language",
        "base": "main",
        "head": "codex/5236",
        "title": "title",
        "body": "Closes #5236",
        "draft": false,
        "remote": "origin",
        "token_file": null
    }))
    .expect("legacy request");
    assert_eq!(request.linkage_mode, PublicationLinkageMode::Closing);
}

#[test]
fn legacy_publication_evidence_omits_mode_without_digest_churn() {
    let evidence: PublicationEvidence = serde_json::from_value(serde_json::json!({
        "repository": "agent-logic/agent-design-language",
        "issue": 5236,
        "pull_request": 7,
        "url": "https://example.invalid/pr/7",
        "base": "main",
        "head": "codex/5236",
        "revision": "git:legacy",
        "draft": false,
        "observed_state": "open"
    }))
    .expect("legacy publication evidence");
    assert_eq!(evidence.linkage_mode, None);
    let encoded = serde_json::to_value(evidence).expect("encoded legacy evidence");
    assert!(encoded.get("linkage_mode").is_none());
}

#[test]
fn public_schema_keeps_publication_and_drops_merged_reconciliation() {
    let bundle = csdlc_v2::public_schema_bundle();
    assert!(bundle.get("publication_request").is_some());
    assert!(bundle.get("publication_intent").is_some());
    assert!(bundle.get("remote_pull_request").is_some());
    for key in [
        "publication_request",
        "publication_intent",
        "remote_pull_request",
        "issue_record",
    ] {
        let schema = bundle[key].to_string();
        assert!(schema.contains("linkage_mode"));
        assert!(schema.contains("part_of"));
    }
    assert!(bundle
        .get("merged_publication_reconciliation_request")
        .is_none());
}
