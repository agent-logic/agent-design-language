use csdlc_v2::{github_token, ErrorCode};
use serde_json::Value;
use std::{env, fs};

const ROUTE_OWNER_CONTRACT: &str = "Covered C-SDLC GitHub route owners: issue actions = `csdlc-github-issue`; PR state = `csdlc-github-pr`; publication = `csdlc-publish`; terminal delivery = `csdlc-finish`.";
const ROUTE_PROHIBITION_CONTRACT: &str = "Route rule: the ChatGPT GitHub connector and raw `gh` are prohibited for covered lifecycle writes except for the audited break-glass transport below. A missing binary, unfamiliar error, timeout, or operator preference is not by itself break-glass authority.";
const BREAK_GLASS_DEFAULT_CONTRACT: &str =
    "Typed C-SDLC v2 remains the default and final lifecycle authority.";
const BREAK_GLASS_RECONCILIATION_CONTRACT: &str = "After a transported write, readiness, review, publication, merge-ready, terminal, and finish claims remain denied until the typed owner reconciles exact remote state and the immutable reconciliation event records success.";
const CONNECTOR_403_CONTRACT: &str = "A connector `403 Resource not accessible by integration` is an integration authorization failure. It is not evidence that the shared token resolver or operator-approved token failed, and it does not authorize connector retry or the audited raw-`gh` exception.";
const DEDICATED_PROOF_HOOK: &str =
    "cargo test --manifest-path csdlc-v2/Cargo.toml --test gate_github_route_policy";
const ROUTE_OWNER_BINARIES: [&str; 4] = [
    "csdlc-github-issue",
    "csdlc-github-pr",
    "csdlc-publish",
    "csdlc-finish",
];

fn normalized_policy(document: &str) -> String {
    document.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[test]
fn github_route_policy_is_consistent_and_fail_closed() {
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let agents = fs::read_to_string(repository.join("AGENTS.md")).expect("read root AGENTS");
    let boundary =
        fs::read_to_string(repository.join("docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md"))
            .expect("read GitHub client boundary");
    let coordination = fs::read_to_string(
        repository.join("docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md"),
    )
    .expect("read session coordination policy");

    for (name, document) in [
        ("AGENTS.md", agents.as_str()),
        ("client boundary", boundary.as_str()),
    ] {
        let document = normalized_policy(document);
        assert!(
            document.contains(ROUTE_OWNER_CONTRACT),
            "{name} must retain the exact GitHub route-owner contract"
        );
        assert!(
            document.contains(ROUTE_PROHIBITION_CONTRACT),
            "{name} must retain the exact fail-closed route and exception boundary"
        );
        assert!(
            document.contains(BREAK_GLASS_DEFAULT_CONTRACT),
            "{name} must preserve typed v2 as default and final authority"
        );
    }
    assert!(
        normalized_policy(&boundary).contains(BREAK_GLASS_RECONCILIATION_CONTRACT),
        "client boundary must deny later lifecycle claims until exact typed reconciliation"
    );
    let coordination = normalized_policy(&coordination);
    for required in [
        "intent.json",
        "result.json",
        "reconciliation.json",
        "must never be overwritten",
        "reconciliation_status: succeeded",
    ] {
        assert!(
            coordination.contains(required),
            "coordination policy must retain break-glass receipt contract: {required}"
        );
    }

    let coexistence: Value = serde_json::from_str(include_str!("../operator/coexistence.json"))
        .expect("parse C-SDLC coexistence manifest");
    let required = coexistence["required_v2_binaries"]
        .as_array()
        .expect("required_v2_binaries array");
    for owner in ROUTE_OWNER_BINARIES {
        assert!(
            required.iter().any(|binary| binary == owner),
            "the verified installation must include route owner {owner}"
        );
    }
    assert!(
        boundary.contains(DEDICATED_PROOF_HOOK),
        "the authoritative boundary must retain its dedicated proof hook"
    );
}

#[test]
fn connector_403_is_not_token_failure_or_fallback_authority() {
    let fixture: Value = serde_json::from_str(include_str!("fixtures/github_connector_403.json"))
        .expect("parse connector 403 fixture");
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let boundary =
        fs::read_to_string(repository.join("docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md"))
            .expect("read GitHub client boundary");

    assert_eq!(fixture["source"], "chatgpt_github_connector");
    assert_eq!(fixture["operation"], "covered_lifecycle_write");
    assert_eq!(fixture["status"], 403);
    assert_eq!(fixture["message"], "Resource not accessible by integration");
    assert_eq!(
        fixture["classification"],
        "integration_authorization_failure"
    );
    assert_eq!(fixture["token_failure"], false);
    assert_eq!(fixture["fallback_authorized"], false);
    assert_eq!(fixture["required_route"], "repo_native_rust_owner");
    assert!(
        normalized_policy(&boundary).contains(CONNECTOR_403_CONTRACT),
        "the authoritative boundary must retain the connector 403 classification"
    );

    let encoded = fixture.to_string();
    for forbidden in ["Bearer ", "github_pat_", "ghp_"] {
        assert!(
            !encoded.contains(forbidden),
            "fixture must not retain credential material matching {forbidden:?}"
        );
    }
}

#[test]
fn shared_token_precedence_and_error_redaction_are_preserved() {
    const KEYS: [&str; 5] = [
        "ADL_GITHUB_TOKEN",
        "GITHUB_TOKEN",
        "GH_TOKEN",
        "ADL_GITHUB_TOKEN_FILE",
        "HOME",
    ];
    let prior = KEYS.map(|key| (key, env::var_os(key)));
    let temp = tempfile::tempdir().expect("tempdir");
    let home = temp.path().join("home");
    let keys = home.join("keys");
    fs::create_dir_all(&keys).expect("create default token directory");
    fs::write(keys.join("github.token"), "home-default\n").expect("write default token");
    let configured = temp.path().join("configured.token");
    fs::write(&configured, "configured-file\n").expect("write configured token");
    let explicit = temp.path().join("explicit.token");
    fs::write(&explicit, "explicit-file\n").expect("write explicit token");

    for key in ["ADL_GITHUB_TOKEN", "GITHUB_TOKEN", "GH_TOKEN"] {
        env::remove_var(key);
    }
    env::set_var("HOME", &home);
    env::remove_var("ADL_GITHUB_TOKEN_FILE");
    assert_eq!(github_token::resolve(None).unwrap(), "home-default");

    env::set_var("ADL_GITHUB_TOKEN_FILE", &configured);
    assert_eq!(github_token::resolve(None).unwrap(), "configured-file");

    env::set_var("GH_TOKEN", "gh-token");
    assert_eq!(github_token::resolve(None).unwrap(), "gh-token");
    env::set_var("GITHUB_TOKEN", "github-token");
    assert_eq!(github_token::resolve(None).unwrap(), "github-token");
    env::set_var("ADL_GITHUB_TOKEN", "adl-token");
    assert_eq!(github_token::resolve(None).unwrap(), "adl-token");
    assert_eq!(
        github_token::resolve(explicit.to_str()).unwrap(),
        "explicit-file"
    );

    let missing = temp.path().join("secret-name-that-must-be-redacted.token");
    let error = github_token::resolve(missing.to_str()).expect_err("missing token source");
    assert_eq!(error.code, ErrorCode::InvalidInput);
    assert_eq!(error.message, "GitHub token source is unavailable");
    assert!(!error
        .message
        .contains(&missing.to_string_lossy().into_owned()));

    for (key, value) in prior {
        match value {
            Some(value) => env::set_var(key, value),
            None => env::remove_var(key),
        }
    }
}
