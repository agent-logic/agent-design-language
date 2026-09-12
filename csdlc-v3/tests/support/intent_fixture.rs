//! SIM-03 fixtures install the candidate locally and bootstrap only repository authority.
//! Issue transitions must run through installed intent commands, never fabricated state.
use serde_json::{json, Value};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicU64, Ordering},
    time::Instant,
};
static NEXT: AtomicU64 = AtomicU64::new(0);
pub struct Fixture {
    pub root: PathBuf,
    pub binary: PathBuf,
    pub attempts: Vec<Value>,
    report_path: PathBuf,
    provenance: Value,
    git_log: PathBuf,
}
pub fn source_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_owned()
}
pub fn git(root: &Path, args: &[&str]) -> String {
    let output = Command::new("git")
        .current_dir(root)
        .args(args)
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("GIT_CONFIG_GLOBAL", "/dev/null")
        .env("GIT_AUTHOR_NAME", "Intent fixture")
        .env("GIT_AUTHOR_EMAIL", "fixture@example.invalid")
        .env("GIT_COMMITTER_NAME", "Intent fixture")
        .env("GIT_COMMITTER_EMAIL", "fixture@example.invalid")
        .output()
        .unwrap();
    assert!(output.status.success(), "git {args:?}: {output:?}");
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}
pub fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target)
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}
impl Fixture {
    pub fn new(label: &str) -> Self {
        let root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/sim03-installed-intent")
            .join(format!(
                "{}-{}-{label}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
        fs::create_dir_all(&root).unwrap();
        git(&root, &["init", "--quiet", "--initial-branch=main"]);
        git(
            &root,
            &[
                "remote",
                "add",
                "origin",
                "https://github.com/agent-logic/agent-design-language.git",
            ],
        );
        fs::write(root.join("tracked"), "fixture\n").unwrap();
        git(&root, &["add", "tracked"]);
        git(&root, &["commit", "--quiet", "-m", "fixture"]);
        fs::create_dir_all(root.join(".adl")).unwrap();
        fs::create_dir_all(root.join("csdlc-v3/operator")).unwrap();
        fs::create_dir_all(root.join("worktrees")).unwrap();
        fs::write(root.join(".adl/worktree-policy.json"), serde_json::to_vec(&json!({"schema":"adl.worktree_policy.v1", "required_parent":root.join("worktrees")})).unwrap()).unwrap();
        let reviewed_head = git(&root, &["rev-parse", "HEAD"]);
        git(
            &root,
            &[
                "commit",
                "--quiet",
                "--allow-empty",
                "-m",
                "V3-F cutover (#591)",
            ],
        );
        let merge_commit = git(&root, &["rev-parse", "HEAD"]);
        let terminal = serde_json::to_vec_pretty(&json!({
        "schema":"csdlc.v3.terminal_receipt.v1", "repository":"agent-logic/agent-design-language",
        "issue":505, "pull_request":591, "head_sha":reviewed_head,
        "disposition":"closed_out", "state_digest":"fixture"
    }))
    .unwrap();
        fs::create_dir_all(root.join(".csdlc/evidence/505")).unwrap();
        fs::write(
            root.join(".csdlc/evidence/505/terminal-receipt.json"),
            &terminal,
        )
        .unwrap();
        let observation = serde_json::to_vec_pretty(&json!({
            "schema":"csdlc.github_pr_state.v1", "repository":"agent-logic/agent-design-language",
            "pull_request":591, "base_ref":"main", "head_sha":reviewed_head,
            "merge_commit_sha":merge_commit, "state":"closed", "merged":true,
            "linked_issue":505, "linkage_source":"github_closing_issues_references",
            "observation_authority":"typed-csdlc-github-pr-authenticated-readback"
        }))
        .unwrap();
        fs::write(
            root.join("csdlc-v3/operator/native-authority-pr-observation.json"),
            &observation,
        )
        .unwrap();
        let receipt_path = root.join("csdlc-v3/operator/native-authority-receipt.json");
        let receipt = serde_json::to_vec_pretty(&json!({
        "schema": "csdlc.v3.native_authority_receipt.v1", "authority_issue": 505,
        "authority_pull_request": 591, "reviewed_head": reviewed_head,
        "merge_commit": merge_commit, "pr_observation_path":"csdlc-v3/operator/native-authority-pr-observation.json",
        "pr_observation_digest":blake3::hash(&observation).to_hex().to_string(), "terminal_receipt_path": ".csdlc/evidence/505/terminal-receipt.json",
        "terminal_receipt_digest": blake3::hash(&terminal).to_hex().to_string(), "source_selector_schema": "csdlc.generation_selector.v2",
        "source_selector_digest": format!("sha256:{}", "3".repeat(64)),
        "operational_authority": "csdlc-v3", "review_authority": "typed-exact-head",
        "approval_authority": "merged-pr-591-closed-issue-505",
        "remote_reconciliation": "canonical-terminal-receipt-and-git-objects"
    }))
    .unwrap();
        fs::write(&receipt_path, &receipt).unwrap();
        let selector_path = root.join("csdlc-v3/operator/authority-selector.json");
        let selector = serde_json::to_vec_pretty(&json!({
            "schema": "csdlc.v3.authority_selector.v1",
            "generation": "v3",
            "operational_authority": "csdlc-v3",
            "authority_issue": 505,
            "authority_pull_request": 591,
            "review_authority": "typed-exact-head",
            "approval_authority": "merged-pr-591-closed-issue-505",
            "receipt_path": "csdlc-v3/operator/native-authority-receipt.json",
            "receipt_digest": blake3::hash(&receipt).to_hex().to_string()
        }))
        .unwrap();
        fs::write(&selector_path, &selector).unwrap();
        git(
            &root,
            &[
                "add",
                ".adl/worktree-policy.json",
                "csdlc-v3/operator/authority-selector.json",
                "csdlc-v3/operator/native-authority-receipt.json",
                "csdlc-v3/operator/native-authority-pr-observation.json",
                ".csdlc/evidence/505/terminal-receipt.json",
            ],
        );
        git(&root, &["commit", "--quiet", "-m", "activate v3"]);
        let exact_head = git(&root, &["rev-parse", "HEAD"]);
        git(
            &root,
            &["update-ref", "refs/remotes/origin/main", &exact_head],
        );

        copy_tree(
            &source_root().join("docs/templates/prompts/1.0.5"),
            &root.join("docs/templates/prompts/1.0.5"),
        );
        fs::copy(
            source_root().join("docs/templates/prompts/current.json"),
            root.join("docs/templates/prompts/current.json"),
        )
        .unwrap();
        fs::create_dir_all(root.join("docs/csdlc-v3")).unwrap();
        for name in [
            "CONTRACT.md",
            "predecessor-coverage.json",
            "proportional-lifecycle.json",
        ] {
            fs::copy(
                source_root().join("docs/csdlc-v3").join(name),
                root.join("docs/csdlc-v3").join(name),
            )
            .unwrap();
        }
        fs::create_dir_all(root.join("fixture-proof/src")).unwrap();
        fs::write(
            root.join("fixture-proof/Cargo.toml"),
            "[package]\nname = \"intent-fixture-proof\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )
        .unwrap();
        fs::write(root.join("fixture-proof/src/lib.rs"), "#[test]\nfn tracked_fixture_bytes_are_preserved() { assert_eq!(include_str!(\"../../tracked\"), \"fixture\\n\"); }\n").unwrap();
        fs::write(
            root.join(".gitignore"),
            "/target/\n/fixture-proof/target/\n/worktrees/\n",
        )
        .unwrap();
        let lock = Command::new("cargo")
            .current_dir(&root)
            .args([
                "generate-lockfile",
                "--offline",
                "--manifest-path",
                "fixture-proof/Cargo.toml",
            ])
            .output()
            .unwrap();
        assert!(
            lock.status.success(),
            "fixture Cargo lock generation failed: {lock:?}"
        );
        git(&root, &["add", "docs", "fixture-proof", ".gitignore"]);
        git(&root, &["commit", "--quiet", "-m", "fixture templates"]);
        git(&root, &["update-ref", "refs/remotes/origin/main", "HEAD"]);
        let binary = root.join(".git/installed-candidate/csdlc");
        fs::create_dir_all(binary.parent().unwrap()).unwrap();
        fs::copy(env!("CARGO_BIN_EXE_csdlc"), &binary).unwrap();
        assert_eq!(
            fs::read(&binary).unwrap(),
            fs::read(env!("CARGO_BIN_EXE_csdlc")).unwrap()
        );
        let fake_bin = root.join(".git/installed-candidate/fake-bin");
        fs::create_dir_all(&fake_bin).unwrap();
        let script = fake_bin.join("curl");
        fs::write(&script, r#"#!/bin/sh
case "$*" in *'--config -'*) cat >/dev/null;; esac
case "$*" in
 *api.github.com/repos/agent-logic/agent-design-language/issues/505*) printf '%s' '{"number":505,"title":"Installed intent fixture","body":"Fixture issue","state":"open","labels":[],"assignees":[],"milestone":null}' ;;
 *) exit 9 ;;
esac
"#).unwrap();
        #[cfg(unix)]
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        fs::write(
            root.join(".git/installed-candidate/token"),
            "SIM03_SYNTHETIC_TOKEN",
        )
        .unwrap();
        let report_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("target/sim03-intent-corpus")
            .join(format!(
                "{}.json",
                root.file_name().unwrap().to_string_lossy()
            ));
        fs::create_dir_all(report_path.parent().unwrap()).unwrap();
        let provenance = json!({"source_head":git(&source_root(), &["rev-parse","HEAD"]),
          "source_has_uncommitted_changes":!git(&source_root(), &["status","--porcelain"]).is_empty(),
          "installed_binary_blake3":blake3::hash(&fs::read(&binary).unwrap()).to_hex().to_string(),
          "source_binding":"Candidate copied byte-for-byte from this test build; final accepted source installation requires renewed exact-head proof"});
        let git_log = report_path.with_extension("git.jsonl");
        let real_git = std::env::split_paths(&std::env::var_os("PATH").unwrap())
            .map(|path| path.join("git"))
            .find(|path| path.is_file())
            .expect("real Git executable")
            .canonicalize()
            .unwrap();
        let wrapper = root.join(".git/installed-candidate/fake-bin/git");
        let quote = |value: &str| format!("'{}'", value.replace('\'', "'\\''"));
        let script = r#"#!/bin/sh
verb=
subcommand=
skip=0
for arg do
 if test "$skip" = 1; then skip=0; continue; fi
 if test -z "$verb"; then
  case "$arg" in -C|-c) skip=1;; *) verb=$arg;; esac
 else subcommand=$arg; break; fi
done
printf '{"verb":"%s","subcommand":"%s"}\n' "$verb" "$subcommand" >> @LOG@
if test -f @GATE@ && test "$verb" = status; then
 for manifest in @ARCHIVES@/*/manifest.json; do
  if test -f "$manifest"; then
   if test "$(sed -n '1p' @GATE@)" = after-index-removal && test -f "$(sed -n '2p' @GATE@)"; then continue; fi
   printf 'requested archive interruption boundary observed\n' > @PAUSED@
   while test -f @GATE@; do sleep 0.01; done
   break
  fi
 done
fi
exec @GIT@ "$@"
"#
        .replace("@LOG@", &quote(git_log.to_str().unwrap()))
        .replace("@GIT@", &quote(real_git.to_str().unwrap()))
        .replace(
            "@GATE@",
            &quote(git_log.with_extension("archive-gate").to_str().unwrap()),
        )
        .replace(
            "@PAUSED@",
            &quote(git_log.with_extension("archive-paused").to_str().unwrap()),
        )
        .replace(
            "@ARCHIVES@",
            &quote(root.join(".git/csdlc-v3/local/archives").to_str().unwrap()),
        );
        fs::write(&wrapper, script).unwrap();
        #[cfg(unix)]
        fs::set_permissions(wrapper, fs::Permissions::from_mode(0o700)).unwrap();
        Self {
            root,
            binary,
            attempts: Vec::new(),
            report_path,
            provenance,
            git_log,
        }
    }
    pub fn run(&mut self, cwd: &Path, args: &[&str]) -> Output {
        self.run_with_env(cwd, args, &[])
    }
    pub fn run_with_env(&mut self, cwd: &Path, args: &[&str], env: &[(&str, &str)]) -> Output {
        let git_before = fs::read_to_string(&self.git_log)
            .unwrap_or_default()
            .lines()
            .count();
        let start = Instant::now();
        let out = self
            .command(cwd, args)
            .envs(env.iter().copied())
            .output()
            .unwrap();
        self.record_output(cwd, args, &out, start.elapsed().as_millis(), git_before);
        out
    }
    fn command(&self, cwd: &Path, args: &[&str]) -> Command {
        let mut command = Command::new(&self.binary);
        command
            .current_dir(cwd)
            .args(args)
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_CONFIG_GLOBAL", "/dev/null")
            .env(
                "PATH",
                format!(
                    "{}:{}",
                    self.root
                        .join(".git/installed-candidate/fake-bin")
                        .display(),
                    std::env::var("PATH").unwrap()
                ),
            )
            .env(
                "ADL_GITHUB_TOKEN_FILE",
                self.root.join(".git/installed-candidate/token"),
            )
            .env_remove("GH_TOKEN")
            .env("GITHUB_TOKEN", "SIM03_SYNTHETIC_TOKEN")
            .env_remove("CARGO_TARGET_DIR")
            .env_remove("CARGO_BUILD_TARGET_DIR");
        command
    }
    fn record_output(
        &mut self,
        cwd: &Path,
        args: &[&str],
        out: &Output,
        elapsed: u128,
        git_before: usize,
    ) {
        let envelope: Value = serde_json::from_slice(&out.stdout).unwrap_or(Value::Null);
        let content = args
            .windows(2)
            .find(|pair| {
                matches!(
                    pair[0],
                    "--plan" | "--changes" | "--evidence" | "--operation" | "--intent-request"
                )
            })
            .and_then(|pair| {
                let path = Path::new(pair[1]);
                if !path.starts_with(&self.root) {
                    return None;
                }
                let bytes = fs::read(path).ok()?;
                if bytes.len() > 1_048_576 {
                    return None;
                }
                serde_json::from_slice::<Value>(&bytes).ok()
            });
        self.attempts.push(
            json!({"argv":args, "topology":if cwd==self.root {"primary"}else{"linked"},
 "attempt_id":format!("{}-{}",self.root.file_name().unwrap().to_string_lossy(),self.attempts.len()+1),
 "exit_code":out.status.code(), "elapsed_millis":elapsed, "result":envelope,"input":content,"stderr":String::from_utf8_lossy(&out.stderr),"git_observations":self.git_counts_since(git_before)}),
        );
        let corpus = json!({"schema":"csdlc.v3.installed_intent_attempts.v1","issue":869,
          "provenance":self.provenance,"attempted":self.attempts.len(),"attempts":self.attempts});
        let text = serde_json::to_string_pretty(&corpus)
            .unwrap()
            .replace(self.root.to_str().unwrap(), "$FIXTURE_ROOT")
            .replace(source_root().to_str().unwrap(), "$SOURCE_ROOT");
        assert!(
            !text.contains("SIM03_SYNTHETIC_TOKEN"),
            "credential escaped into installed result"
        );
        fs::write(&self.report_path, text).unwrap();
    }
    pub fn write_json(&self, name: &str, value: &Value) -> PathBuf {
        let path = self.root.join(".git/installed-candidate").join(name);
        fs::write(&path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
        path
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

pub fn inventory(root: &Path) -> std::collections::BTreeMap<PathBuf, String> {
    fn visit(root: &Path, path: &Path, entries: &mut std::collections::BTreeMap<PathBuf, String>) {
        let metadata = fs::symlink_metadata(path).unwrap();
        let value = if metadata.file_type().is_symlink() {
            format!("link:{:?}", fs::read_link(path).unwrap())
        } else if metadata.is_dir() {
            "directory".to_owned()
        } else {
            format!("file:{}", blake3::hash(&fs::read(path).unwrap()).to_hex())
        };
        entries.insert(path.strip_prefix(root).unwrap().to_path_buf(), value);
        if metadata.is_dir() {
            for entry in fs::read_dir(path).unwrap() {
                visit(root, &entry.unwrap().path(), entries);
            }
        }
    }
    let mut entries = std::collections::BTreeMap::new();
    visit(root, root, &mut entries);
    entries
}

impl Fixture {
    /// Deterministic stateful fake transport. Only fixed issue endpoints exist;
    /// mutation payload bytes are the real native owner's private input file.
    pub fn enable_issue_transport(&self) {
        let base = self.root.join(".git/installed-candidate");
        fs::write(base.join("remote-issue.json"),serde_json::to_vec(&json!({"number":505,"title":"Installed intent fixture","body":"Fixture issue","state":"open","labels":["retained"],"assignees":["fixture-assignee"],"milestone":{"number":4}})).unwrap()).unwrap();
        let script = base.join("fake-bin/curl");
        fs::write(&script,r#"#!/bin/sh
set -eu
base=$(dirname "$0")/..
method=GET
payload=
url=
while test "$#" -gt 0; do
 case "$1" in
 --request) shift; method=$1 ;;
 --data-binary) shift; payload=${1#@} ;;
 --config) shift; if test "$1" = -; then cat >/dev/null; fi ;;
 https://api.github.com/*) url=$1 ;;
 esac
 shift
done
case "$method:$url" in
 POST:https://api.github.com/repos/agent-logic/agent-design-language/issues/505/comments)
  data=$(cat "$payload"); printf '{"id":700,%s' "${data#\{}" > "$base/remote-comment.json"
  printf 'comment\n' >> "$base/remote-effects"; cat "$base/remote-comment.json" ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/issues/505/comments*)
  if test -f "$base/remote-comment.json"; then printf '['; cat "$base/remote-comment.json"; printf ']'; else printf '[]'; fi ;;
 POST:https://api.github.com/repos/agent-logic/agent-design-language/issues)
  data=$(cat "$payload"); printf '{"number":506,"state":"open",%s' "${data#\{}" > "$base/remote-created.json"
  printf 'create\n' >> "$base/remote-effects"; cat "$base/remote-created.json" ;;
 GET:https://api.github.com/search/issues*)
  printf '{"items":['; if test -f "$base/remote-created.json"; then cat "$base/remote-created.json"; fi; printf ']}' ;;
 PATCH:https://api.github.com/repos/agent-logic/agent-design-language/issues/505)
  data=$(cat "$payload" | sed 's/"milestone":7/"milestone":{"number":7}/'); previous=$(cat "$base/remote-issue.json")
  printf '%s,%s' "${previous%\}}" "${data#\{}" > "$base/remote-issue.json"
  printf 'edit-or-close\n' >> "$base/remote-effects"; cat "$base/remote-issue.json" ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/issues/505) cat "$base/remote-issue.json" ;;
 *) exit 9 ;;
esac
"#).unwrap();
        #[cfg(unix)]
        fs::set_permissions(script, fs::Permissions::from_mode(0o700)).unwrap();
    }
    pub fn remote_issue(&self) -> Value {
        serde_json::from_slice(
            &fs::read(self.root.join(".git/installed-candidate/remote-issue.json")).unwrap(),
        )
        .unwrap()
    }
    pub fn remote_effects(&self) -> usize {
        fs::read_to_string(self.root.join(".git/installed-candidate/remote-effects"))
            .unwrap_or_default()
            .lines()
            .count()
    }
}

impl Fixture {
    pub fn enable_pr_transport(&self, linked: &Path) {
        self.enable_issue_transport();
        let script = self.root.join(".git/installed-candidate/fake-bin/curl");
        let old = fs::read_to_string(&script).unwrap();
        let head = git(linked, &["rev-parse", "HEAD"]);
        let branch = git(linked, &["symbolic-ref", "--short", "HEAD"]);
        let cases=r##"
 POST:https://api.github.com/repos/agent-logic/agent-design-language/pulls)
  data=$(cat "$payload")
  data=$(printf '%s' "$data" | sed 's#"head":"[^"]*"#"head":{"sha":"@HEAD@","ref":"@BRANCH@"}#;s#"base":"main"#"base":{"ref":"main"}#')
  printf '{"number":639,"id":639,"node_id":"PR_ready639","state":"open","merged":false,%s' "${data#\{}" > "$base/remote-pr.json"
  printf 'pr-create\n' >> "$base/remote-effects"; cat "$base/remote-pr.json" ;;
 PATCH:https://api.github.com/repos/agent-logic/agent-design-language/pulls/639)
  data=$(cat "$payload"); previous=$(cat "$base/remote-pr.json")
  printf '%s,%s' "${previous%\}}" "${data#\{}" > "$base/remote-pr.json"
  printf 'pr-update\n' >> "$base/remote-effects"; cat "$base/remote-pr.json" ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/pulls\?*)
  printf '['; if test -f "$base/remote-pr.json"; then cat "$base/remote-pr.json"; fi; printf ']' ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/pulls/639)
  if test -f "$base/drop-readback"; then exit 9; fi
  cat "$base/remote-pr.json" ;;
 POST:https://api.github.com/graphql)
  if test -f "$base/reject-ready-before-effect"; then exit 9; fi
  if ! test -f "$base/remote-pr.json"; then exit 9; fi
  sed 's/"draft":true/"draft":false/' "$base/remote-pr.json" > "$base/remote-pr.next"
  mv "$base/remote-pr.next" "$base/remote-pr.json"
  printf 'pr-ready\n' >> "$base/remote-effects"
  if test -f "$base/uncertain-response"; then touch "$base/drop-readback"; fi
  printf '%s' '{"data":{"markPullRequestReadyForReview":{"pullRequest":{"number":639,"headRefOid":"@HEAD@","isDraft":false}}}}' ;;
"##.replace("@HEAD@",&head).replace("@BRANCH@",&branch);
        let replaced = old.replacen(
            "case \"$method:$url\" in\n",
            &format!("case \"$method:$url\" in\n{cases}"),
            1,
        );
        fs::write(script, replaced).unwrap();
    }
    pub fn remote_pr(&self) -> Value {
        serde_json::from_slice(
            &fs::read(self.root.join(".git/installed-candidate/remote-pr.json")).unwrap(),
        )
        .unwrap()
    }
    pub fn set_remote_pr(&self, value: &Value) {
        fs::write(
            self.root.join(".git/installed-candidate/remote-pr.json"),
            serde_json::to_vec(value).unwrap(),
        )
        .unwrap();
    }
    pub fn remote_flag(&self, name: &str, present: bool) {
        let path = self.root.join(".git/installed-candidate").join(name);
        if present {
            fs::write(path, b"synthetic fixture transport condition").unwrap();
        } else if path.exists() {
            fs::remove_file(path).unwrap();
        }
    }
}

impl Fixture {
    fn git_counts_since(&self, start: usize) -> Value {
        let text = fs::read_to_string(&self.git_log).unwrap_or_default();
        let mut reads = 0;
        let mut mutations = 0;
        let mut unknown = 0;
        let mut authority_reads = 0;
        for line in text.lines().skip(start) {
            let value: Value = serde_json::from_str(line).expect("instrumented Git event");
            let verb = value["verb"].as_str().unwrap();
            let sub = value["subcommand"].as_str().unwrap();
            let read = matches!(
                verb,
                "rev-parse"
                    | "show"
                    | "cat-file"
                    | "status"
                    | "ls-files"
                    | "merge-base"
                    | "rev-list"
                    | "log"
                    | "check-ref-format"
                    | "for-each-ref"
            ) || verb == "symbolic-ref" && sub == "--short"
                || verb == "remote" && sub == "get-url"
                || verb == "worktree" && sub == "list"
                || verb == "branch" && matches!(sub, "--show-current" | "--list");
            if read {
                reads += 1;
            } else if verb == "worktree" && matches!(sub, "add" | "remove" | "prune")
                || matches!(verb, "branch" | "update-ref" | "checkout" | "switch")
            {
                mutations += 1;
            } else {
                unknown += 1;
            }
            if verb == "show"
                && (sub.contains("authority-selector")
                    || sub.contains("authority-receipt")
                    || sub.contains("authority-pr-observation"))
            {
                authority_reads += 1;
            }
        }
        json!({"read_invocations":reads,"mutation_invocations":mutations,"unclassified_invocations":unknown,"authority_blob_reads":authority_reads,"total":reads+mutations+unknown,"instrumentation":"actual child Git argv categories; observer log outside fixture inventory; in-process file reads are separate"})
    }
}

fn fixture_merge_state(head: &str, merged: bool) -> Value {
    let checks = json!({"nodes":[{"__typename":"CheckRun","name":"ci","status":"COMPLETED","conclusion":"SUCCESS","isRequired":true,"checkSuite":{"app":{"databaseId":42}}}],"pageInfo":{"hasNextPage":false}});
    json!({"data":{"repository":{"nameWithOwner":"agent-logic/agent-design-language","mergeCommitAllowed":true,"pullRequest":{
        "body":"Closes #505", "closingIssuesReferences":{"nodes":[{"number":505,"url":"https://github.com/agent-logic/agent-design-language/issues/505","repository":{"nameWithOwner":"agent-logic/agent-design-language"}}],"pageInfo":{"hasNextPage":false}},
        "number":639,"url":"https://github.com/agent-logic/agent-design-language/pull/639", "headRefOid":head,
        "baseRefName":"main","baseRefOid":"1111111111111111111111111111111111111111","state":if merged {"MERGED"} else {"OPEN"},
        "merged":merged,"isDraft":false,"mergeable":"MERGEABLE","mergeStateStatus":"CLEAN","reviewDecision":null,
        "baseRef":{"branchProtectionRule":null},"reviewThreads":{"nodes":[],"pageInfo":{"hasNextPage":false}},
        "latestReviews":{"nodes":[],"pageInfo":{"hasNextPage":false}},
        "commits":{"nodes":[{"commit":{"oid":head,"statusCheckRollup":{"state":"SUCCESS","contexts":checks}}}]},
        "mergeCommit":if merged {json!({"oid":"2222222222222222222222222222222222222222","parents":{"nodes":[{"oid":"1111111111111111111111111111111111111111"},{"oid":head}],"pageInfo":{"hasNextPage":false}}})} else {Value::Null}
    }},"linkedRepository":{"nameWithOwner":"agent-logic/agent-design-language","issue":{"number":505,"url":"https://github.com/agent-logic/agent-design-language/issues/505","state":if merged {"CLOSED"} else {"OPEN"}}}}})
}

impl Fixture {
    pub fn enable_merge_transport(&self, linked: &Path) {
        let base = self.root.join(".git/installed-candidate");
        let head = git(linked, &["rev-parse", "HEAD"]);
        for (name, merged) in [("merge-before.json", false), ("merge-after.json", true)] {
            fs::write(
                base.join(name),
                serde_json::to_vec(&fixture_merge_state(&head, merged)).unwrap(),
            )
            .unwrap();
        }
        fs::write(base.join("merge-rules.json"),serde_json::to_vec(&json!([{"type":"required_status_checks","parameters":{"required_status_checks":[{"context":"ci","integration_id":42}]}}])).unwrap()).unwrap();
        let script = base.join("fake-bin/curl");
        let old = fs::read_to_string(&script).unwrap();
        let cases = r#"
 GET:https://api.github.com/graphql)
  if test -f "$base/merged"; then cat "$base/merge-after.json"; else cat "$base/merge-before.json"; fi ;;
 GET:https://api.github.com/repos/agent-logic/agent-design-language/rules/branches/main*) cat "$base/merge-rules.json" ;;
 PUT:https://api.github.com/repos/agent-logic/agent-design-language/pulls/639/merge)
  touch "$base/merged"
  sed 's/"merged":false/"merged":true/g;s/"state":"open"/"state":"closed"/g' "$base/remote-pr.json" > "$base/remote-pr.next"
  mv "$base/remote-pr.next" "$base/remote-pr.json"
  sed 's/"state":"open"/"state":"closed"/g' "$base/remote-issue.json" > "$base/remote-issue.next"
  mv "$base/remote-issue.next" "$base/remote-issue.json"
  printf 'pr-merge\n' >> "$base/remote-effects"
  printf '%s' '{"merged":true,"sha":"2222222222222222222222222222222222222222"}' ;;
"#;
        fs::write(
            script,
            old.replacen(
                "case \"$method:$url\" in\n",
                &format!("case \"$method:$url\" in\n{cases}"),
                1,
            ),
        )
        .unwrap();
    }
}

impl Fixture {
    /// Pause a fixture-only Git observer after the archive manifest is durable,
    /// then kill the candidate before its next inventory permits original removal.
    pub fn interrupt_clean_after_archive(&mut self, cwd: &Path, args: &[&str]) -> Output {
        self.interrupt_clean(cwd, args, false)
    }
    pub fn interrupt_clean_after_index_removal(&mut self, cwd: &Path, args: &[&str]) -> Output {
        self.interrupt_clean(cwd, args, true)
    }
    fn interrupt_clean(&mut self, cwd: &Path, args: &[&str], after_index_removal: bool) -> Output {
        let gate = self.git_log.with_extension("archive-gate");
        let paused = self.git_log.with_extension("archive-paused");
        if paused.exists() {
            fs::remove_file(&paused).unwrap();
        }
        let linked = git(&self.root, &["worktree", "list", "--porcelain"])
            .lines()
            .filter_map(|line| line.strip_prefix("worktree "))
            .map(PathBuf::from)
            .find(|path| path != &self.root)
            .unwrap();
        fs::write(
            &gate,
            format!(
                "{}\n{}\n",
                if after_index_removal {
                    "after-index-removal"
                } else {
                    "after-archive-manifest"
                },
                linked.join(".csdlc/issues/505/index.json").display()
            ),
        )
        .unwrap();
        let git_before = fs::read_to_string(&self.git_log)
            .unwrap_or_default()
            .lines()
            .count();
        let start = Instant::now();
        let mut child = self
            .command(cwd, args)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .unwrap();
        while !paused.exists() && start.elapsed() < std::time::Duration::from_secs(30) {
            if child.try_wait().unwrap().is_some() {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(5));
        }
        let reached = paused.exists();
        let _ = child.kill();
        fs::remove_file(&gate).unwrap();
        let output = child.wait_with_output().unwrap();
        self.record_output(cwd, args, &output, start.elapsed().as_millis(), git_before);
        assert!(
            reached,
            "candidate never reached durable-archive barrier: {output:?}"
        );
        output
    }
}
