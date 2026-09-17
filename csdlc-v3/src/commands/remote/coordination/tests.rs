//! PVF: deterministic local owner regression; synthetic authenticated adapter;
//! no live GitHub writes, providers, or timing assumptions. Required #1006 guard proof.
use super::*;
use crate::adapters::{CommandInvocation, ProcessAdapter, ProcessOutput, ProcessStatus};
use serde_json::{json, Value};
use std::{
    collections::VecDeque,
    fs,
    path::PathBuf,
    sync::atomic::{AtomicUsize, Ordering},
};

const REPO: &str = "agent-logic/agent-design-language";
const HEAD: &str = "1111111111111111111111111111111111111111";
static NEXT: AtomicUsize = AtomicUsize::new(0);

struct Fixture {
    root: PathBuf,
    request: GithubMutationRequest,
    parent: Value,
    child: Value,
    linkage: Value,
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}
impl Fixture {
    fn new() -> Self {
        let root = std::env::temp_dir().join(format!(
            "csdlc-coordination-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(root.join(".csdlc/evidence/1006")).unwrap();
        fs::create_dir(root.join(".git")).unwrap();
        let path = ".csdlc/evidence/1006/completion.json";
        let evidence = b"{\"proof\":\"bounded synthetic completion evidence\"}\n";
        fs::write(root.join(path), evidence).unwrap();
        let declaration = json!({"repository":REPO,"issue":1006,"kind":"coordination_only","children":[{"issue":887,"pull_request":989,"head_sha":HEAD}]});
        let body =
            format!("Preserved coordinator prose.\n\n<!-- csdlc-coordination:v1 {declaration} -->");
        let request = serde_json::from_value(json!({"repository":REPO,"issue":1006,"expected_head_sha":HEAD,"operator_approval":"Operator approved exact coordination completion","credential_names":["GITHUB_TOKEN"],"mutation":{"action":"issue_complete_coordination","completion":{"current_body":body,"expected_updated_at":"2026-09-16T00:00:00Z","rationale":"All declared child deliveries verified","evidence":[{"path":path,"digest":blake3::hash(evidence).to_hex().to_string()}]}}})).unwrap();
        let parent = json!({"number":1006,"html_url":format!("https://github.com/{REPO}/issues/1006"),"state":"open","body":body,"updated_at":"2026-09-16T00:00:00Z"});
        let child = json!({"number":887,"html_url":format!("https://github.com/{REPO}/issues/887"),"state":"closed","state_reason":"completed"});
        let linkage = json!({"data":{"repository":{"nameWithOwner":REPO,"pullRequest":{"number":989,"url":format!("https://github.com/{REPO}/pull/989"),"state":"MERGED","headRefOid":HEAD,"merged":true,"mergeCommit":{"oid":"2222222222222222222222222222222222222222"},"body":"Closes #887","closingIssuesReferences":{"nodes":[{"number":887,"url":format!("https://github.com/{REPO}/issues/887"),"repository":{"nameWithOwner":REPO}}],"pageInfo":{"hasNextPage":false}}}},"linkedRepository":{"nameWithOwner":REPO,"issue":{"number":887,"url":format!("https://github.com/{REPO}/issues/887"),"state":"CLOSED"}}}});
        Self {
            root,
            request,
            parent,
            child,
            linkage,
        }
    }
    fn completion(&mut self) -> &mut CoordinationCompletion {
        match &mut self.request.mutation {
            GithubMutation::IssueCompleteCoordination { completion } => completion,
            _ => unreachable!(),
        }
    }
    fn adapter(&self) -> Reads {
        Reads {
            values: VecDeque::from([
                self.parent.clone(),
                self.child.clone(),
                self.linkage.clone(),
                self.parent.clone(),
            ]),
            calls: vec![],
        }
    }
}
struct Reads {
    values: VecDeque<Value>,
    calls: Vec<Vec<String>>,
}
impl ProcessAdapter for Reads {
    fn run(&mut self, invocation: CommandInvocation) -> ProcessOutput {
        assert_eq!(
            invocation.program, "github-api-read-only",
            "guard must not mutate GitHub"
        );
        assert_eq!(invocation.child_credential_name(), Some("GITHUB_TOKEN"));
        self.calls.push(invocation.argv().to_vec());
        ProcessOutput {
            status: ProcessStatus::Exit(0),
            stdout: self
                .values
                .pop_front()
                .expect("unexpected extra read")
                .to_string(),
            stderr: String::new(),
            truncated: false,
        }
    }
}

#[test]
fn coordination_accepts_exact_authenticated_completed_children() {
    let fixture = Fixture::new();
    validate(&fixture.request).unwrap();
    let mut adapter = fixture.adapter();
    verify(&fixture.root, &fixture.request, &mut adapter).unwrap();
    assert!(adapter.values.is_empty());
    let receipts = fixture.root.join(".git/csdlc-v3/coordination-readiness");
    let files = fs::read_dir(&receipts)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .collect::<Vec<_>>();
    assert_eq!(files.len(), 1);
    let retained = fs::read(&files[0]).unwrap();
    let receipt: Value = serde_json::from_slice(&retained).unwrap();
    assert_eq!(receipt["issue"], 1006);
    assert_eq!(receipt["children"][0]["issue"], 887);
    assert_eq!(receipt["children"][0]["head_sha"], HEAD);
    assert!(!String::from_utf8_lossy(&retained).contains("Preserved coordinator prose"));
    let mut repeated = fixture.adapter();
    verify(&fixture.root, &fixture.request, &mut repeated).unwrap();
    assert_eq!(fs::read_dir(&receipts).unwrap().count(), 1);
    assert_eq!(fs::read(&files[0]).unwrap(), retained);
    assert_eq!(adapter.calls[0], vec!["issue", REPO, "1006"]);
    assert_eq!(adapter.calls[1], vec!["issue", REPO, "887"]);
    assert_eq!(
        adapter.calls[2],
        vec![
            "pull-request-merge-linkage",
            REPO,
            "989:agent-logic/agent-design-language#887"
        ]
    );
}

#[test]
fn coordination_contract_installation_is_canonical_and_preserves_pre_state() {
    let mut fixture = Fixture::new();
    let declaration: CoordinationContract = serde_json::from_value(json!({
        "repository": REPO,
        "issue": 1006,
        "kind": "coordination_only",
        "children": [{"issue":887,"pull_request":989,"head_sha":HEAD}]
    }))
    .unwrap();
    fixture.completion().current_body = "Preserved coordinator prose.  \n".into();
    fixture.completion().install_contract = Some(declaration);
    fixture.parent["body"] = json!("Preserved coordinator prose.  \n");

    validate(&fixture.request).unwrap();
    let target = target_body(fixture.completion()).unwrap();
    assert_eq!(
        target,
        format!(
            "Preserved coordinator prose.  \n\n\n<!-- csdlc-coordination:v1 {{\"repository\":\"{REPO}\",\"issue\":1006,\"kind\":\"coordination_only\",\"children\":[{{\"issue\":887,\"pull_request\":989,\"head_sha\":\"{HEAD}\"}}]}} -->"
        )
    );
    let mut adapter = fixture.adapter();
    verify(&fixture.root, &fixture.request, &mut adapter).unwrap();
}

#[test]
fn coordination_contract_installation_bounds_the_complete_outbound_body() {
    let mut fixture = Fixture::new();
    let declaration: CoordinationContract = serde_json::from_value(json!({
        "repository": REPO,
        "issue": 1006,
        "kind": "coordination_only",
        "children": [{"issue":887,"pull_request":989,"head_sha":HEAD}]
    }))
    .unwrap();
    fixture.completion().current_body = "x".repeat(65_500);
    fixture.completion().install_contract = Some(declaration);
    assert_eq!(
        validate(&fixture.request).unwrap_err().code,
        "github_coordination_completion_denied"
    );
}

#[test]
fn coordination_contract_installation_rejects_existing_or_wrong_contracts() {
    for case in ["existing", "repository", "issue", "kind", "head"] {
        let mut fixture = Fixture::new();
        let declaration: CoordinationContract = serde_json::from_value(json!({
            "repository": REPO,
            "issue": 1006,
            "kind": "coordination_only",
            "children": [{"issue":887,"pull_request":989,"head_sha":HEAD}]
        }))
        .unwrap();
        fixture.completion().install_contract = Some(declaration);
        match case {
            "existing" => {}
            "repository" => {
                fixture.completion().current_body = "Legacy umbrella".into();
                fixture
                    .completion()
                    .install_contract
                    .as_mut()
                    .unwrap()
                    .repository = "other/repository".into();
            }
            "issue" => {
                fixture.completion().current_body = "Legacy umbrella".into();
                fixture
                    .completion()
                    .install_contract
                    .as_mut()
                    .unwrap()
                    .issue = 1007;
            }
            "kind" => {
                fixture.completion().current_body = "Legacy umbrella".into();
                fixture.completion().install_contract.as_mut().unwrap().kind =
                    "implementation".into();
            }
            "head" => {
                fixture.completion().current_body = "Legacy umbrella".into();
                fixture
                    .completion()
                    .install_contract
                    .as_mut()
                    .unwrap()
                    .children[0]
                    .head_sha = "short".into();
            }
            _ => unreachable!(),
        }
        assert_eq!(
            validate(&fixture.request).unwrap_err().code,
            "github_coordination_completion_denied",
            "installation bypass: {case}"
        );
    }
}

#[test]
fn coordination_structural_contract_rejects_missing_authority_and_unqualified_body() {
    for case in [
        "approval",
        "marker",
        "wrong_repository",
        "wrong_issue",
        "duplicate_marker",
        "empty_evidence",
    ] {
        let mut fixture = Fixture::new();
        match case {
            "approval" => fixture.request.operator_approval = None,
            "marker" => fixture.completion().current_body = "ordinary implementation issue".into(),
            "wrong_repository" => {
                fixture.completion().current_body = fixture
                    .completion()
                    .current_body
                    .replace(REPO, "unrelated/repository")
            }
            "wrong_issue" => {
                fixture.completion().current_body = fixture
                    .completion()
                    .current_body
                    .replace("\"issue\":1006", "\"issue\":1007")
            }
            "duplicate_marker" => {
                let body = fixture.completion().current_body.clone();
                fixture.completion().current_body = format!("{body}\n{body}");
            }
            "empty_evidence" => fixture.completion().evidence.clear(),
            _ => unreachable!(),
        }
        assert_eq!(
            validate(&fixture.request).unwrap_err().code,
            "github_coordination_completion_denied",
            "structural bypass: {case}"
        );
    }
}

#[test]
fn coordination_authenticated_readiness_rejects_substitution_and_unfinished_delivery() {
    for case in [
        "parent_body",
        "parent_updated",
        "parent_closed",
        "parent_identity",
        "child_open",
        "child_reason",
        "child_identity",
        "child_repo",
        "head",
        "not_merged",
        "no_merge_commit",
        "wrong_closing_issue",
        "truncated_links",
    ] {
        let mut fixture = Fixture::new();
        match case {
            "parent_body" => fixture.parent["body"] = json!("substituted authenticated body"),
            "parent_updated" => fixture.parent["updated_at"] = json!("2026-09-16T00:01:00Z"),
            "parent_closed" => fixture.parent["state"] = json!("closed"),
            "parent_identity" => fixture.parent["number"] = json!(1007),
            "child_open" => fixture.child["state"] = json!("open"),
            "child_reason" => {
                fixture
                    .child
                    .as_object_mut()
                    .unwrap()
                    .remove("state_reason");
            }
            "child_identity" => fixture.child["number"] = json!(888),
            "child_repo" => {
                fixture.child["html_url"] = json!("https://github.com/unrelated/repo/issues/887")
            }
            "head" => {
                fixture.linkage["data"]["repository"]["pullRequest"]["headRefOid"] =
                    json!("3333333333333333333333333333333333333333")
            }
            "not_merged" => {
                fixture.linkage["data"]["repository"]["pullRequest"]["merged"] = json!(false)
            }
            "no_merge_commit" => {
                fixture.linkage["data"]["repository"]["pullRequest"]["mergeCommit"] = Value::Null
            }
            "wrong_closing_issue" => {
                fixture.linkage["data"]["repository"]["pullRequest"]["body"] = json!("Closes #888")
            }
            "truncated_links" => {
                fixture.linkage["data"]["repository"]["pullRequest"]["closingIssuesReferences"]
                    ["pageInfo"]["hasNextPage"] = json!(true)
            }
            _ => unreachable!(),
        }
        let mut adapter = fixture.adapter();
        let expected = if matches!(case, "wrong_closing_issue" | "truncated_links") {
            "github_merge_linkage_ineligible"
        } else {
            "github_coordination_completion_denied"
        };
        assert_eq!(
            verify(&fixture.root, &fixture.request, &mut adapter)
                .unwrap_err()
                .code,
            expected,
            "authenticated bypass: {case}"
        );
    }
}

#[test]
fn coordination_durable_evidence_rejects_missing_or_changed_bytes() {
    for remove in [true, false] {
        let fixture = Fixture::new();
        let path = fixture.root.join(".csdlc/evidence/1006/completion.json");
        if remove {
            fs::remove_file(path).unwrap();
        } else {
            fs::write(path, b"changed evidence").unwrap();
        }
        let mut adapter = fixture.adapter();
        assert_eq!(
            verify(&fixture.root, &fixture.request, &mut adapter)
                .unwrap_err()
                .code,
            "github_coordination_completion_denied"
        );
    }
}

#[test]
fn coordination_rechecks_parent_before_dispatch_evidence() {
    let fixture = Fixture::new();
    let mut adapter = fixture.adapter();
    adapter.values.back_mut().unwrap()["updated_at"] = json!("changed-during-child-read");
    assert_eq!(
        verify(&fixture.root, &fixture.request, &mut adapter)
            .unwrap_err()
            .code,
        "github_coordination_completion_denied"
    );
    assert!(!fixture
        .root
        .join(".git/csdlc-v3/coordination-readiness")
        .exists());
}

#[test]
fn completed_child_allows_parent_context_without_weakening_merge_admission() {
    for parent in [
        "Part of #505",
        "Part-of agent-logic/agent-design-language#505",
        "Part of https://github.com/agent-logic/agent-design-language/issues/505",
    ] {
        let mut fixture = Fixture::new();
        fixture.linkage["data"]["repository"]["pullRequest"]["body"] =
            json!(format!("Closes #887\n\n{parent}"));
        let mut adapter = fixture.adapter();
        verify(&fixture.root, &fixture.request, &mut adapter).unwrap();
        let linkage = PublicationLinkage {
            repository: REPO.into(),
            issue: 887,
            mode: RemotePublicationMode::Closing,
        };
        let mut request = fixture.request.clone();
        request.issue = 887;
        request.pull_request = Some(989);
        assert_eq!(
            linkage
                .validate(&fixture.linkage, &request, true)
                .unwrap_err()
                .code,
            "github_merge_linkage_ineligible"
        );
    }
}

#[test]
fn parent_context_cannot_replace_or_expand_child_closing_evidence() {
    for case in [
        "part_of_only",
        "wrong_child",
        "extra_closing",
        "extra_authenticated_link",
        "wrong_authenticated_link",
        "open_linked_child",
        "truncated_links",
    ] {
        let mut fixture = Fixture::new();
        fixture.linkage["data"]["repository"]["pullRequest"]["body"] =
            json!("Closes #887\nPart of #505");
        let pr = &mut fixture.linkage["data"]["repository"]["pullRequest"];
        match case {
            "part_of_only" => pr["body"] = json!("Part of #887\nPart of #505"),
            "wrong_child" => pr["body"] = json!("Closes #888\nPart of #505"),
            "extra_closing" => pr["body"] = json!("Closes #887\nCloses #888\nPart of #505"),
            "extra_authenticated_link" => {
                let extra = json!({"number":888,"url":format!("https://github.com/{REPO}/issues/888"),"repository":{"nameWithOwner":REPO}});
                pr["closingIssuesReferences"]["nodes"]
                    .as_array_mut()
                    .unwrap()
                    .push(extra);
            }
            "wrong_authenticated_link" => {
                pr["closingIssuesReferences"]["nodes"][0]["number"] = json!(888)
            }
            "truncated_links" => {
                pr["closingIssuesReferences"]["pageInfo"]["hasNextPage"] = json!(true)
            }
            "open_linked_child" => {
                fixture.linkage["data"]["linkedRepository"]["issue"]["state"] = json!("OPEN")
            }
            _ => unreachable!(),
        }
        assert_eq!(
            verify(&fixture.root, &fixture.request, &mut fixture.adapter())
                .unwrap_err()
                .code,
            "github_merge_linkage_ineligible",
            "accepted {case}"
        );
    }
}
