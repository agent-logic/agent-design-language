use csdlc_v3::application::{
    FoundationState, IssueProjection, FOUNDATION_PREDECESSORS, ISSUE_START_MINUTES_MAX,
    REQUIREMENT_PROOFS,
};
use csdlc_v3::repository::RepositoryContext;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("manifest has repository parent")
        .to_path_buf()
}

fn state() -> FoundationState {
    let context = RepositoryContext::discover(repo_root()).expect("explicit repository context");
    FoundationState::load(&context).expect("foundation state loads")
}

#[test]
fn repository_context_is_explicit() {
    let root = repo_root();
    let context = RepositoryContext::discover(&root).expect("explicit repository context");
    assert_eq!(
        context.root(),
        root.canonicalize().expect("canonical repository root")
    );
    assert_eq!(
        context.relative_display(context.contract_path()),
        "docs/csdlc-v3/CONTRACT.md"
    );
    assert_eq!(
        context.relative_display(context.predecessor_coverage_path()),
        "docs/csdlc-v3/predecessor-coverage.json"
    );
    assert_eq!(
        context.relative_display(context.proportional_lifecycle_path()),
        "docs/csdlc-v3/proportional-lifecycle.json"
    );
}

#[test]
fn single_binary_foundation_command_is_read_only_and_explicit() {
    let output = Command::new(env!("CARGO_BIN_EXE_csdlc-v3-foundation"))
        .args(["--repo-root", &repo_root().to_string_lossy()])
        .output()
        .expect("run foundation binary");
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).expect("utf8 stdout");
    assert!(stdout.contains("\"schema\":\"csdlc.v3.foundation.v1\""));
    assert!(stdout.contains("\"read_only\":true"));
    assert!(stdout.contains("\"operational_authority\":\"csdlc-v2\""));
}

#[test]
fn application_context_reports_typed_missing_root_errors() {
    let missing = repo_root().join("definitely-missing-v3-foundation-root");
    let error = RepositoryContext::discover(&missing).expect_err("missing root is rejected");
    assert!(error.to_string().contains("repository root"));
}

#[test]
fn read_only_v2_import_loads_issue_record_and_cards() {
    let context = RepositoryContext::discover(repo_root()).expect("explicit repository context");
    let issue_record: serde_json::Value =
        serde_json::from_str(&context.issue_record_text(501).expect("issue record text"))
            .expect("issue record json");
    let projection = IssueProjection::load(&context, 501).expect("read-only issue projection");
    assert_eq!(projection.issue, 501);
    assert_eq!(projection.schema, "csdlc.issue.index.v1");
    assert_eq!(
        projection.phase,
        issue_record
            .get("phase")
            .and_then(serde_json::Value::as_str)
            .expect("issue record phase")
    );
    assert!(projection.generation > 0);
    assert_eq!(projection.digest.len(), 64);
    assert_eq!(projection.card_count, 6);
    assert_eq!(
        projection
            .cards
            .iter()
            .map(|projection| projection.key.as_str())
            .collect::<Vec<_>>(),
        ["sip", "sor", "spp", "srp", "stp", "vpp"]
    );
    for card in projection.cards {
        assert!(card.value.contains("status="));
        assert!(card.value.contains("markdown_bytes="));
        assert!(card.value.contains("values_digest="));
    }
}

#[test]
fn read_only_v2_import_rejects_malformed_issue_records() {
    let fixture = FixtureRepo::new("malformed-issue-record");
    fixture.write_v3_contracts();
    fixture.write_issue_with_record(
        777,
        r#"{"schema":"csdlc.issue.index.v1","issue":777,"phase":"ready","digest":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cards":{}}"#,
    );
    fixture.write_all_cards(777);

    let context = RepositoryContext::discover(fixture.root()).expect("fixture context");
    let error = IssueProjection::load(&context, 777).expect_err("missing generation is rejected");
    assert!(error.to_string().contains("generation"));
}

#[test]
fn read_only_v2_import_rejects_unsupported_record_fields_and_card_identity_drift() {
    let fixture = FixtureRepo::new("identity-drift");
    fixture.write_v3_contracts();
    fixture.write_issue_with_record(
        778,
        &fixture.issue_record_json(778, Some(r#","surprise":true"#)),
    );
    fixture.write_all_cards(778);
    let context = RepositoryContext::discover(fixture.root()).expect("fixture context");
    let error = IssueProjection::load(&context, 778).expect_err("unsupported field is rejected");
    assert!(error.to_string().contains("unsupported field"));

    let fixture = FixtureRepo::new("card-identity-drift");
    fixture.write_v3_contracts();
    fixture.write_issue_with_record(779, &fixture.issue_record_json(779, None));
    fixture.write_all_cards(779);
    fixture.write_card_values(779, "sip", 780, "sip");
    let context = RepositoryContext::discover(fixture.root()).expect("fixture context");
    let error = IssueProjection::load(&context, 779).expect_err("card identity drift is rejected");
    assert!(error.to_string().contains("does not match requested"));
}

#[test]
fn repository_context_rejects_symlink_escape_for_issue_records() {
    let fixture = FixtureRepo::new("symlink-escape");
    fixture.write_v3_contracts();
    let outside = fixture.outside_path("outside-index.json");
    fs::write(&outside, fixture.issue_record_json(780, None)).expect("outside record");
    let issue_dir = fixture.root().join(".csdlc/issues/780");
    fs::create_dir_all(&issue_dir).expect("issue dir");
    std::os::unix::fs::symlink(&outside, issue_dir.join("index.json"))
        .expect("symlink escaped record");

    let context = RepositoryContext::discover(fixture.root()).expect("fixture context");
    let error = IssueProjection::load(&context, 780).expect_err("symlink escape is rejected");
    assert!(error.to_string().contains("outside repository root"));
}

#[test]
fn projection_replay_is_deterministic() {
    let first = state();
    let second = state();
    assert_eq!(first.projections(), second.projections());
    assert_eq!(first.to_machine_json(), second.to_machine_json());
    let keys = first
        .projections()
        .into_iter()
        .map(|projection| projection.key)
        .collect::<Vec<_>>();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
}

#[test]
fn retained_requirements_164_through_167_are_bound() {
    let state = state();
    assert_eq!(FOUNDATION_PREDECESSORS, [164, 165, 166, 167]);
    assert_eq!(state.foundation_predecessors(), [164, 165, 166, 167]);
    assert_eq!(state.operational_authority(), "csdlc-v2");
    assert_eq!(state.requirement_proofs(), REQUIREMENT_PROOFS);
    for issue in FOUNDATION_PREDECESSORS {
        assert!(
            state
                .requirement_proofs()
                .iter()
                .any(|proof| proof.issue == issue
                    && proof.title.contains(&format!("V3-{:02}", issue - 161))
                    && !proof.source_scope.is_empty()
                    && !proof.foundation_behavior.is_empty()),
            "missing behavioral retained-requirement proof for #{issue}"
        );
    }
}

#[test]
fn retained_requirement_behaviors_are_source_grounded() {
    let state = state();
    let proofs = state.requirement_proofs();
    assert!(proofs.iter().any(|proof| proof.issue == 164
        && proof.source_scope.contains("root parser")
        && proof.foundation_behavior.contains("csdlc-v3-foundation")));
    assert!(proofs.iter().any(|proof| proof.issue == 165
        && proof
            .source_scope
            .contains("invocation-scoped dependency container")
        && proof
            .foundation_behavior
            .contains("explicit RepositoryContext")));
    assert!(proofs.iter().any(|proof| proof.issue == 166
        && proof
            .source_scope
            .contains("read-only v2 record/card parsing")
        && proof
            .foundation_behavior
            .contains("canonicalizes an explicit root")));
    assert!(proofs.iter().any(|proof| proof.issue == 167
        && proof.source_scope.contains("canonical serialization")
        && proof
            .foundation_behavior
            .contains("byte-stable machine JSON")));
}

#[test]
fn issue_start_projection_preserves_three_minute_budget_without_authority_cutover() {
    let state = state();
    assert_eq!(ISSUE_START_MINUTES_MAX, 3);
    assert_eq!(state.issue_start_minutes_max(), 3);
    let json = state.to_machine_json();
    assert!(json.contains("\"read_only\":true"));
    assert!(json.contains("\"operational_authority\":\"csdlc-v2\""));
    assert!(json.contains("\"key\":\"issue_start_minutes_max\",\"value\":\"3\""));
    assert!(json.contains("\"key\":\"requirement_proofs\""));
}

struct FixtureRepo {
    root: PathBuf,
}

impl FixtureRepo {
    fn new(name: &str) -> Self {
        let root = repo_root().join(format!(
            "csdlc-v3/target/foundation-fixtures/{}-{name}",
            std::process::id()
        ));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(&root).expect("fixture root");
        Self { root }
    }

    fn root(&self) -> PathBuf {
        self.root.clone()
    }

    fn outside_path(&self, file_name: &str) -> PathBuf {
        let outside = self.outside_dir();
        fs::create_dir_all(&outside).expect("outside fixture dir");
        outside.join(file_name)
    }

    fn outside_dir(&self) -> PathBuf {
        self.root.with_file_name(format!(
            "{}-outside",
            self.root
                .file_name()
                .expect("fixture root has file name")
                .to_string_lossy()
        ))
    }

    fn write_v3_contracts(&self) {
        let docs = self.root.join("docs/csdlc-v3");
        fs::create_dir_all(&docs).expect("v3 docs dir");
        fs::write(
            docs.join("CONTRACT.md"),
            "v2 remains the sole operational authority",
        )
        .expect("contract");
        fs::write(
            docs.join("predecessor-coverage.json"),
            r#"{"denominator": [161, 162, 163]}"#,
        )
        .expect("coverage");
        fs::write(
            docs.join("proportional-lifecycle.json"),
            r#"{"three_issue_ready_minutes_max": 3}"#,
        )
        .expect("lifecycle");
    }

    fn write_issue_with_record(&self, issue: u64, record: &str) {
        let issue_dir = self.root.join(format!(".csdlc/issues/{issue}"));
        fs::create_dir_all(issue_dir.join("cards")).expect("issue cards dir");
        fs::write(issue_dir.join("index.json"), record).expect("issue record");
    }

    fn write_all_cards(&self, issue: u64) {
        for card in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            let card_dir = self.root.join(format!(".csdlc/issues/{issue}/cards"));
            fs::create_dir_all(&card_dir).expect("card dir");
            fs::write(card_dir.join(format!("{card}.md")), format!("# {card}\n"))
                .expect("card markdown");
            self.write_card_values(issue, card, issue, card);
        }
    }

    fn write_card_values(&self, issue: u64, card: &str, identity_issue: u64, kind: &str) {
        let card_dir = self.root.join(format!(".csdlc/issues/{issue}/cards"));
        fs::write(
            card_dir.join(format!("{card}.values.json")),
            format!(
                r#"{{"identity":{{"issue":{identity_issue}}},"status":"ready","content":{{"card_kind":"{kind}"}}}}"#
            ),
        )
        .expect("card values");
    }

    fn issue_record_json(&self, issue: u64, extra: Option<&str>) -> String {
        format!(
            r#"{{"schema":"csdlc.issue.index.v1","issue":{issue},"phase":"ready","generation":1,"digest":"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa","cards":{{"sip":{projection},"stp":{projection},"spp":{projection},"vpp":{projection},"srp":{projection},"sor":{projection}}}{extra}}}"#,
            projection = r#"{"values_digest":"v","rendered_digest":"r","ast_digest":"a"}"#,
            extra = extra.unwrap_or("")
        )
    }
}

impl Drop for FixtureRepo {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
        let _ = fs::remove_dir_all(self.outside_dir());
    }
}
