use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

static FIXTURE_SEQUENCE: AtomicU64 = AtomicU64::new(0);

const RECORDS: [(u64, &str); 7] = [
    (511, "prepared"),
    (517, "bound_dirty"),
    (497, "implemented"),
    (3, "reviewed"),
    (505, "published"),
    (122, "terminal"),
    (980, "pending_recovery"),
];

#[test]
fn first_journal_entry_survives_abrupt_exit_and_same_request_resumes() {
    let fixture = Fixture::new();
    let operation = fixture.operation("journal-create");
    let request = fixture.write_request(
        &operation,
        Some(json!({
            "point": "operation_journal_creation",
            "boundary": "after",
            "mode": "once"
        })),
    );

    let interrupted = invoke(&request);
    assert!(!interrupted.status.success());
    assert_ne!(interrupted.status.code(), Some(2));
    assert!(
        interrupted.stdout.is_empty(),
        "fault was returned as an ordinary error"
    );

    let journal = fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .join(&operation)
        .join("journal.jsonl");
    let retained = fs::read(&journal).expect("journal entry must survive abrupt exit");
    assert!(String::from_utf8_lossy(&retained).contains("request_digest"));
    for relative in [
        "csdlc-v3",
        "csdlc-v3/local",
        "csdlc-v3/local/conversion-rehearsals",
        &format!("csdlc-v3/local/conversion-rehearsals/{operation}"),
    ] {
        assert!(
            fixture.git_common.join(relative).is_dir(),
            "missing {relative}"
        );
    }
    // This proves first-use layout plus fresh-process recovery. A physical power-loss
    // claim requires a filesystem crash harness; the production code requests that
    // guarantee by fsyncing each new child into its parent.

    let resumed = invoke(&request);
    assert_success(&resumed);
    assert!(fs::read(&journal)
        .expect("journal remains readable")
        .starts_with(&retained));
}

#[test]
fn same_operation_rejects_an_altered_source_census_before_effects() {
    let fixture = Fixture::new();
    let operation = fixture.operation("identity-mismatch");
    let request = fixture.write_request(
        &operation,
        Some(json!({
            "point": "operation_journal_creation",
            "boundary": "after",
            "mode": "once"
        })),
    );
    let interrupted = invoke(&request);
    assert!(!interrupted.status.success());
    assert_ne!(interrupted.status.code(), Some(2));

    let index = fixture.copied.join("511/index.json");
    let mut bytes = fs::read(&index).unwrap();
    bytes.extend_from_slice(b"\n");
    fs::write(&index, bytes).unwrap();

    let rejected = invoke(&request);
    assert_eq!(rejected.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&rejected.stdout).contains("operation identity mismatch"));
    assert!(!fixture.git_common.join("csdlc-v3/semantic/issues").exists());
}

#[test]
fn linked_worktree_admission_rejects_primary_arbitrary_and_identity_mismatch() {
    let fixture = Fixture::new();

    let mut primary_request = fixture.request_value(&fixture.operation("primary"), None);
    primary_request["linked_worktree"] = json!(fixture.primary);
    primary_request["linked_branch"] = json!("main");
    primary_request["linked_head"] = json!(git_stdout(&["rev-parse", "HEAD"], &fixture.primary));
    let rejected_primary = invoke(&fixture.write_value("primary.json", &primary_request));
    assert_normal_rejection(&rejected_primary, "registered non-primary worktree");

    let arbitrary = fixture.root.join("arbitrary");
    fs::create_dir_all(&arbitrary).unwrap();
    let mut arbitrary_request = fixture.request_value(&fixture.operation("arbitrary"), None);
    arbitrary_request["linked_worktree"] = json!(arbitrary);
    let rejected_arbitrary = invoke(&fixture.write_value("arbitrary.json", &arbitrary_request));
    assert_normal_rejection(&rejected_arbitrary, "git");

    let mut escape_request = fixture.request_value(&fixture.operation("escape"), None);
    escape_request["linked_worktree"] = json!(fixture.linked.join("..").join("linked"));
    let rejected_escape = invoke(&fixture.write_value("escape.json", &escape_request));
    assert_normal_rejection(&rejected_escape, "without traversal components");

    let other = fixture.root.join("other-repository");
    fs::create_dir_all(&other).unwrap();
    git(&["init", "-q", "-b", "main"], &other);
    git(
        &["config", "user.email", "issue872@example.invalid"],
        &other,
    );
    git(&["config", "user.name", "Issue 872 test"], &other);
    fs::write(other.join("seed"), b"other repository\n").unwrap();
    git(&["add", "seed"], &other);
    git(&["commit", "-qm", "seed"], &other);
    let mut common_request = fixture.request_value(&fixture.operation("common"), None);
    common_request["linked_worktree"] = json!(other);
    common_request["linked_branch"] = json!("main");
    common_request["linked_head"] = json!(git_stdout(&["rev-parse", "HEAD"], &other));
    let rejected_common = invoke(&fixture.write_value("common.json", &common_request));
    assert_normal_rejection(&rejected_common, "different Git common directory");

    let mut branch_request = fixture.request_value(&fixture.operation("branch"), None);
    branch_request["linked_branch"] = json!("codex/wrong-branch");
    let rejected_branch = invoke(&fixture.write_value("branch.json", &branch_request));
    assert_normal_rejection(&rejected_branch, "branch mismatch");

    let mut mismatch_request = fixture.request_value(&fixture.operation("mismatch"), None);
    mismatch_request["linked_head"] = json!("0000000000000000000000000000000000000000");
    let rejected_mismatch = invoke(&fixture.write_value("mismatch.json", &mismatch_request));
    assert_normal_rejection(&rejected_mismatch, "HEAD mismatch");

    assert!(!fixture.git_common.join("csdlc-v3/semantic/issues").exists());
}

#[test]
fn incomplete_convertible_record_stops_the_whole_census_before_effects() {
    let fixture = Fixture::new();
    fs::remove_file(fixture.copied.join("497/index.json")).unwrap();
    let request = fixture.write_request(&fixture.operation("incomplete"), None);

    let rejected = invoke(&request);
    assert_eq!(rejected.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&rejected.stdout);
    assert!(stdout.contains("issue 497"), "{stdout}");
    assert!(
        stdout.contains("unsupported_ambiguous_incomplete_source"),
        "{stdout}"
    );
    assert!(!fixture.git_common.join("csdlc-v3/semantic").exists());
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/projections")
        .exists());
    assert!(!all_files(&fixture.git_common).iter().any(|path| path
        .file_name()
        .is_some_and(|name| name.to_string_lossy().contains("ledger"))));
}

#[test]
fn invalid_generation_in_a_late_record_stops_before_any_semantic_effect() {
    let fixture = Fixture::new();
    let index_path = fixture.copied.join("122/index.json");
    let mut index: Value = serde_json::from_slice(&fs::read(&index_path).unwrap()).unwrap();
    index["generation"] = json!(0);
    fs::write(&index_path, serde_json::to_vec_pretty(&index).unwrap()).unwrap();
    let request = fixture.write_request(&fixture.operation("late-generation"), None);

    let rejected = invoke(&request);
    assert_eq!(rejected.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&rejected.stdout);
    assert!(stdout.contains("issue 122"), "{stdout}");
    assert!(stdout.contains("invalid numeric generation"), "{stdout}");
    assert!(!fixture.git_common.join("csdlc-v3/semantic").exists());
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/projections")
        .exists());
    assert!(!fixture
        .git_common
        .join("csdlc-v3/local/conversion-rehearsals")
        .exists());
}

fn invoke(request: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .args(["convert", "--request"])
        .arg(request)
        .output()
        .expect("conversion process must start")
}

fn assert_success(output: &Output) {
    assert!(
        output.status.success(),
        "stdout={} stderr={}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        value.get("status").and_then(Value::as_str),
        Some("completed")
    );
}

fn assert_normal_rejection(output: &Output, expected: &str) {
    assert_eq!(output.status.code(), Some(2));
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(expected), "{stdout}");
}

struct Fixture {
    root: PathBuf,
    primary: PathBuf,
    linked: PathBuf,
    git_common: PathBuf,
    copied: PathBuf,
    linked_head: String,
}

impl Fixture {
    fn new() -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let sequence = FIXTURE_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let root = std::env::temp_dir().join(format!(
            "csdlc-v3-conversion-admission-{}-{nonce}-{sequence}",
            std::process::id()
        ));
        let primary = root.join("primary");
        let linked = root.join("linked");
        fs::create_dir_all(&primary).unwrap();
        git(&["init", "-q", "-b", "main"], &primary);
        git(
            &["config", "user.email", "issue872@example.invalid"],
            &primary,
        );
        git(&["config", "user.name", "Issue 872 test"], &primary);
        fs::write(primary.join("seed"), b"isolated fixture\n").unwrap();
        git(&["add", "seed"], &primary);
        git(&["commit", "-qm", "seed"], &primary);
        git(&["branch", "codex/fixture-linked"], &primary);
        git(
            &[
                "worktree",
                "add",
                "-q",
                linked.to_str().unwrap(),
                "codex/fixture-linked",
            ],
            &primary,
        );
        let linked_head = git_stdout(&["rev-parse", "HEAD"], &linked);
        let git_common = primary.join(".git");

        let source =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/issue872-copied-records");
        let copied = primary.join("copied-records");
        for (issue, _) in RECORDS {
            let source_record = if issue == 980 {
                source.join("980/complete-real-baseline")
            } else {
                source.join(issue.to_string())
            };
            copy_tree(&source_record, &copied.join(issue.to_string()));
        }
        let registry = primary.join("docs/templates/prompts");
        fs::create_dir_all(&registry).unwrap();
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/templates/prompts/current.json"),
            registry.join("current.json"),
        )
        .unwrap();
        fs::write(root.join("authority.bytes"), b"isolated-authority\n").unwrap();
        Self {
            root,
            primary,
            linked,
            git_common,
            copied,
            linked_head,
        }
    }

    fn operation(&self, suffix: &str) -> String {
        format!("issue872-{suffix}-{}", std::process::id())
    }

    fn request_value(&self, operation_id: &str, fault: Option<Value>) -> Value {
        let records = RECORDS
            .iter()
            .map(|(issue, role)| {
                json!({"issue": issue, "role": role, "source": self.copied.join(issue.to_string())})
            })
            .collect::<Vec<_>>();
        let mut request = json!({
            "schema": "csdlc.v3.copied_record_conversion.v1",
            "repository": "isolated/admission",
            "operation_id": operation_id,
            "authority_bytes_path": self.root.join("authority.bytes"),
            "git_common": self.git_common,
            "linked_branch": "codex/fixture-linked",
            "linked_head": self.linked_head,
            "linked_worktree": self.linked,
            "records": records,
            "registry_path": self.primary.join("docs/templates/prompts/current.json")
        });
        if let Some(fault) = fault {
            request["fault_injection"] = fault;
        }
        request
    }

    fn write_request(&self, operation_id: &str, fault: Option<Value>) -> PathBuf {
        self.write_value(
            &format!("{operation_id}.json"),
            &self.request_value(operation_id, fault),
        )
    }

    fn write_value(&self, name: &str, value: &Value) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, serde_json::to_vec_pretty(value).unwrap()).unwrap();
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn git(args: &[&str], cwd: &Path) {
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

fn git_stdout(args: &[&str], cwd: &Path) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(output.status.success());
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(source_path, destination_path).unwrap();
        }
    }
}

fn all_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                files.extend(all_files(&path));
            } else if path.is_file() {
                files.push(path);
            }
        }
    }
    files
}
