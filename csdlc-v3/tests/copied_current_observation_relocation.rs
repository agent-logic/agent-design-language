use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[test]
fn relocates_complete_primary_and_linked_observations_with_provenance() {
    for (issue, role) in [(970, "primary")] {
        let target = Target::new();
        let source = source(issue);
        let before = tree_bytes(&source);
        let request_value = target.request_value(issue, role, &source);
        reconcile_local_cards_with_semantic_projection(&request_value);
        let request = target.write("consistent-primary.json", &request_value);
        let output = invoke(&request);
        assert_success(&output);
        let result: Value = serde_json::from_slice(&output.stdout).unwrap();
        assert_eq!(result["status"], "completed");
        assert_eq!(result["issue"], issue);
        assert!(Path::new(result["journal_path"].as_str().unwrap()).is_file());
        assert!(Path::new(result["provenance_path"].as_str().unwrap()).is_file());
        for kind in ["local_lifecycle", "semantic_current", "projection_state"] {
            assert!(result["source_hashes"][kind]
                .as_str()
                .is_some_and(|value| value.starts_with("semantic-projection-v1:")));
            assert!(result["relocated_hashes"][kind]
                .as_str()
                .is_some_and(|value| value.starts_with("semantic-projection-v1:")));
        }
        assert_eq!(
            tree_bytes(&source),
            before,
            "source bytes changed for {issue}"
        );

        let observed = Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
            .args(["status", "--git-common"])
            .arg(&target.git_common)
            .args([
                "--repository",
                "agent-logic/agent-design-language",
                "--issue",
                &issue.to_string(),
            ])
            .output()
            .unwrap();
        assert_success_process(&observed);
    }
}

#[test]
fn refuses_inconsistent_local_and_semantic_sources_before_effects() {
    let target = Target::new();
    let linked_source = source(981);
    let linked_value = target.request_value(981, "linked", &linked_source);
    reconcile_local_cards_with_semantic_projection(&linked_value);
    assert_rejected(
        &invoke(&target.write("local-phase-mismatch.json", &linked_value)),
        "local lifecycle phase disagrees",
    );
    assert_no_effect(&target, 981);

    let target = Target::new();
    let primary_source = source(970);
    let value = target.request_value(970, "primary", &primary_source);
    reconcile_local_cards_with_semantic_projection(&value);
    let local = PathBuf::from(value["source_local_issue"].as_str().unwrap());
    let card = local.join("cards/sip.values.json");
    let mut altered: Value = serde_json::from_slice(&fs::read(&card).unwrap()).unwrap();
    altered["title"] = json!("substituted local title");
    fs::write(&card, serde_json::to_vec_pretty(&altered).unwrap()).unwrap();
    recompute_local_digest(&local);
    assert_rejected(
        &invoke(&target.write("local-card-mismatch.json", &value)),
        "local card values disagree",
    );
    assert_no_effect(&target, 970);
}

#[test]
fn refuses_incomplete_linked_source_before_any_target_effect() {
    let target = Target::new();
    let source = source(978);
    let before = tree_bytes(&source);
    let output = invoke(&target.request(978, "linked", &source));
    assert_rejected(&output, "incomplete/recovery-required");
    assert_eq!(tree_bytes(&source), before);
    assert_no_effect(&target, 978);
}

#[test]
fn refuses_escape_and_exact_topology_mismatch_before_effects() {
    let target = Target::new();
    let source = source(970);
    let consistent = target.request_value(970, "primary", &source);
    reconcile_local_cards_with_semantic_projection(&consistent);

    let mut escape = consistent.clone();
    escape["operation_id"] = json!("escape");
    escape["target_worktree"] = json!(target.primary.join("..").join("primary"));
    assert_rejected(
        &invoke(&target.write("escape.json", &escape)),
        "without traversal",
    );

    let mut mismatch = consistent.clone();
    mismatch["operation_id"] = json!("head-mismatch");
    mismatch["target_head"] = json!("0000000000000000000000000000000000000000");
    assert_rejected(
        &invoke(&target.write("mismatch.json", &mismatch)),
        "mismatch",
    );
    for (name, operation_id) in [("dot", "."), ("dotdot", "..")] {
        let mut value = consistent.clone();
        value["operation_id"] = json!(operation_id);
        assert_rejected(
            &invoke(&target.write(&format!("{name}.json"), &value)),
            "non-dot path segment",
        );
    }
    assert_no_effect(&target, 970);
}

fn source(issue: u64) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures/issue872-current-observations")
        .join(issue.to_string())
}

fn reconcile_local_cards_with_semantic_projection(request: &Value) {
    let local = PathBuf::from(request["source_local_issue"].as_str().unwrap());
    let projection = PathBuf::from(request["source_projection_state"].as_str().unwrap());
    let projection: Value = serde_json::from_slice(&fs::read(projection).unwrap()).unwrap();
    let cards = projection["inputs"]["intent_plan"]["cards"]
        .as_object()
        .unwrap();
    for (kind, card) in cards {
        fs::write(
            local.join("cards").join(format!("{kind}.values.json")),
            serde_json::to_vec_pretty(card).unwrap(),
        )
        .unwrap();
    }
    recompute_local_digest(&local);
}

fn recompute_local_digest(local: &Path) {
    let index_path = local.join("index.json");
    let mut index: Value = serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
    index.as_object_mut().unwrap().remove("digest");
    let mut hasher = blake3::Hasher::new();
    hasher.update(&serde_json::to_vec(&index).unwrap());
    for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
        for suffix in ["values.json", "md"] {
            hasher.update(&fs::read(local.join(format!("cards/{kind}.{suffix}"))).unwrap());
        }
    }
    let binding = local.join("binding.json");
    if binding.is_file() {
        hasher.update(&fs::read(binding).unwrap());
    }
    let intent_plan = local.join("intent-plan.json");
    if intent_plan.is_file() {
        hasher.update(b"csdlc.v3.intent_plan.v1\0");
        hasher.update(&fs::read(intent_plan).unwrap());
    }
    index["digest"] = json!(hasher.finalize().to_hex().to_string());
    fs::write(index_path, serde_json::to_vec_pretty(&index).unwrap()).unwrap();
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

struct Target {
    root: PathBuf,
    primary: PathBuf,
    linked: PathBuf,
    git_common: PathBuf,
    head: String,
}

impl Target {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = PathBuf::from("/Volumes/FastWork/adl-worktrees").join(format!(
            ".issue872-relocation-test-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        let primary = root.join("primary");
        let linked = root.join("linked");
        fs::create_dir_all(&primary).unwrap();
        git(&primary, &["init", "-q", "-b", "main"]);
        git(&primary, &["config", "user.name", "Issue 872 fixture"]);
        git(
            &primary,
            &["config", "user.email", "issue872@example.invalid"],
        );
        fs::write(primary.join("README.md"), b"isolated relocation target\n").unwrap();
        git(&primary, &["add", "README.md"]);
        git(&primary, &["commit", "-qm", "fixture"]);
        git(
            &primary,
            &[
                "remote",
                "add",
                "origin",
                "git@github.com:agent-logic/agent-design-language.git",
            ],
        );
        let git_common = primary.join(".git");
        fs::create_dir_all(git_common.join("objects/info")).unwrap();
        fs::write(
            git_common.join("objects/info/alternates"),
            format!(
                "{}/objects\n",
                git_output(
                    &repository_root(),
                    &["rev-parse", "--path-format=absolute", "--git-common-dir"]
                )
            ),
        )
        .unwrap();
        let origin_main = git_output(&repository_root(), &["rev-parse", "origin/main"]);
        git(
            &primary,
            &["update-ref", "refs/remotes/origin/main", &origin_main],
        );
        copy_tree(
            &repository_root().join("csdlc-v3/operator"),
            &primary.join("csdlc-v3/operator"),
        );
        copy_file(
            &repository_root().join(".csdlc/evidence/505/terminal-receipt.json"),
            &primary.join(".csdlc/evidence/505/terminal-receipt.json"),
        );
        copy_file(
            &repository_root().join(".adl/worktree-policy.json"),
            &primary.join(".adl/worktree-policy.json"),
        );
        copy_tree(
            &repository_root().join("docs/templates/prompts"),
            &primary.join("docs/templates/prompts"),
        );
        git(&primary, &["add", "."]);
        git(
            &primary,
            &["commit", "-qm", "authenticated relocation topology"],
        );
        git(&primary, &["branch", "codex/fixture-linked"]);
        git(
            &primary,
            &[
                "worktree",
                "add",
                "-q",
                linked.to_str().unwrap(),
                "codex/fixture-linked",
            ],
        );
        let head = git_output(&primary, &["rev-parse", "HEAD"]);
        Self {
            root,
            primary,
            linked,
            git_common,
            head,
        }
    }

    fn request_value(&self, issue: u64, role: &str, source: &Path) -> Value {
        let source_local = self
            .root
            .join("source-local/issues")
            .join(issue.to_string());
        if !source_local.exists() {
            copy_tree(&source.join("local"), &source_local);
        }
        let source_repository = self
            .root
            .join("source-repositories")
            .join(issue.to_string());
        if !source_repository.exists() {
            fs::create_dir_all(&source_repository).unwrap();
            git(&source_repository, &["init", "-q", "-b", "main"]);
            copy_tree(
                &source.join("git-common/csdlc-v3"),
                &source_repository.join(".git/csdlc-v3"),
            );
            copy_file(
                &source.join("projection/state.json"),
                &source_repository
                    .join(".git/csdlc-v3/local/projections")
                    .join(issue.to_string())
                    .join("state.json"),
            );
        }
        let worktree = if role == "primary" {
            &self.primary
        } else {
            &self.linked
        };
        let branch = if role == "primary" {
            "main"
        } else {
            "codex/fixture-linked"
        };
        json!({
            "schema":"csdlc.v3.current_observation_relocation.v1",
            "operation_id":format!("relocate-{issue}-{role}"),
            "repository":"agent-logic/agent-design-language",
            "issue":issue,
            "source_local_issue":source_local,
            "source_semantic_current":source_repository.join(".git/csdlc-v3/semantic/issues").join(issue.to_string()).join("current.json"),
            "source_projection_state":source.join("projection/state.json"),
            "target_repository_root":worktree,
            "target_git_common":self.git_common,
            "target_worktree":worktree,
            "target_branch":branch,
            "target_head":self.head,
            "target_worktree_role":role,
            "registry_path":worktree.join("docs/templates/prompts/current.json"),
        })
    }

    fn request(&self, issue: u64, role: &str, source: &Path) -> PathBuf {
        self.write(
            &format!("request-{issue}-{role}.json"),
            &self.request_value(issue, role, source),
        )
    }

    fn write(&self, name: &str, value: &Value) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
        path
    }
}

impl Drop for Target {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn invoke(request: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["relocate-current", "--request"])
        .arg(request)
        .output()
        .unwrap()
}

fn assert_success(output: &Output) {
    assert_success_process(output);
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["status"], "completed");
}

fn assert_success_process(output: &Output) {
    assert!(
        output.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn assert_rejected(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(expected), "{stdout}");
}

fn assert_no_effect(target: &Target, issue: u64) {
    assert!(!target
        .git_common
        .join("csdlc-v3/semantic/issues")
        .join(issue.to_string())
        .exists());
    assert!(!target
        .git_common
        .join("csdlc-v3/local/issues")
        .join(issue.to_string())
        .exists());
    assert!(!target
        .linked
        .join(".csdlc/issues")
        .join(issue.to_string())
        .exists());
    assert!(!target
        .git_common
        .join("csdlc-v3/local/current-observation-relocations")
        .exists());
}

fn git(cwd: &Path, args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_output(cwd: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "git {args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn copy_file(source: &Path, target: &Path) {
    fs::create_dir_all(target.parent().unwrap()).unwrap();
    fs::copy(source, target).unwrap();
}

fn copy_tree(source: &Path, target: &Path) {
    fs::create_dir_all(target).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let destination = target.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).unwrap();
        }
    }
}

fn tree_bytes(root: &Path) -> Vec<(PathBuf, Vec<u8>)> {
    fn visit(root: &Path, current: &Path, values: &mut Vec<(PathBuf, Vec<u8>)>) {
        let mut entries = fs::read_dir(current)
            .unwrap()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        entries.sort_by_key(|entry| entry.file_name());
        for entry in entries {
            if entry.file_type().unwrap().is_dir() {
                visit(root, &entry.path(), values);
            } else {
                values.push((
                    entry.path().strip_prefix(root).unwrap().to_path_buf(),
                    fs::read(entry.path()).unwrap(),
                ));
            }
        }
    }
    let mut values = Vec::new();
    visit(root, root, &mut values);
    values
}
