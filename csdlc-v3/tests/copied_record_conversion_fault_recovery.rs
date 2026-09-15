use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const CONVERTED_ISSUES: [u64; 6] = [511, 517, 497, 3, 505, 122];
const RECORDS: [(u64, &str); 7] = [
    (511, "prepared"),
    (517, "bound_dirty"),
    (497, "implemented"),
    (3, "reviewed"),
    (505, "published"),
    (122, "terminal"),
    (980, "pending_recovery"),
];
const FAULT_POINTS: [&str; 15] = [
    "conversion_intent_durability",
    "per_issue_staging_write",
    "whole_census_staging_complete",
    "semantic_state_activation",
    "per_issue_conversion_receipt_persistence",
    "projection_data_completion",
    "projection_publication",
    "candidate_executable_activation",
    "fake_remote_request_dispatch",
    "fake_remote_success_readback",
    "local_reconciled_success_persistence",
    "restore_intent_durability",
    "source_record_restoration",
    "prior_executable_restoration",
    "restore_receipt_persistence_and_fence_release",
];

#[test]
fn every_durability_fault_crashes_then_resumes_once_in_a_fresh_process() {
    for point in FAULT_POINTS {
        for boundary in ["before", "after"] {
            exercise_fault(point, boundary);
        }
    }
}

fn exercise_fault(point: &str, boundary: &str) {
    let fixture = Fixture::new(point, boundary);
    let operation_id = format!(
        "issue872-{point}-{boundary}-{}-{}",
        std::process::id(),
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be after Unix epoch")
            .as_nanos()
    );
    let request = fixture.write_request(&operation_id, point, boundary);

    let interrupted = invoke_convert(&request);
    assert!(
        !interrupted.status.success(),
        "{point}/{boundary}: injected conversion unexpectedly returned success; stdout={} stderr={}",
        String::from_utf8_lossy(&interrupted.stdout),
        String::from_utf8_lossy(&interrupted.stderr)
    );

    let crash_files = operation_files(&fixture.git_common, &operation_id);
    assert!(
        !crash_files.is_empty(),
        "{point}/{boundary}: interruption retained no artifact bound to operation {operation_id}"
    );
    assert!(
        crash_files.keys().any(|path| path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().contains("journal"))),
        "{point}/{boundary}: interruption retained no durable operation journal; files={:?}",
        crash_files.keys().collect::<Vec<_>>()
    );

    // A new Command creates a new process. The request bytes, including operation identity
    // and one-shot fault declaration, remain unchanged so recovery cannot substitute a new
    // operation for the interrupted one.
    let recovered = invoke_convert(&request);
    assert!(
        recovered.status.success(),
        "{point}/{boundary}: same-operation restart failed; stdout={} stderr={}",
        String::from_utf8_lossy(&recovered.stdout),
        String::from_utf8_lossy(&recovered.stderr)
    );
    let result: Value = serde_json::from_slice(&recovered.stdout).unwrap_or_else(|error| {
        panic!(
            "{point}/{boundary}: recovery stdout is not JSON ({error}): {}",
            String::from_utf8_lossy(&recovered.stdout)
        )
    });
    assert_eq!(
        result.get("status").and_then(Value::as_str),
        Some("completed"),
        "{point}/{boundary}: restart did not reach one coherent completed outcome: {result}"
    );

    assert_crash_artifacts_are_continuous(point, boundary, &crash_files);
    assert_single_semantic_effect(point, boundary, &fixture.git_common);
    assert_remote_effect_is_not_duplicated(point, boundary, &fixture.git_common, &operation_id);
}

fn invoke_convert(request: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_csdlc-conversion-rehearsal"))
        .arg("convert")
        .arg("--request")
        .arg(request)
        .output()
        .expect("conversion rehearsal process must start")
}

fn assert_crash_artifacts_are_continuous(
    point: &str,
    boundary: &str,
    crash_files: &BTreeMap<PathBuf, Vec<u8>>,
) {
    for (path, crash_bytes) in crash_files {
        let final_bytes = fs::read(path).unwrap_or_else(|error| {
            panic!(
                "{point}/{boundary}: retained crash artifact {} disappeared: {error}",
                path.display()
            )
        });
        assert!(
            final_bytes.starts_with(crash_bytes),
            "{point}/{boundary}: restart rewrote retained crash artifact {}",
            path.display()
        );
    }
}

fn assert_single_semantic_effect(point: &str, boundary: &str, git_common: &Path) {
    for issue in CONVERTED_ISSUES {
        let issue_root = git_common
            .join("csdlc-v3/semantic/issues")
            .join(issue.to_string());
        let current: Value = read_json(&issue_root.join("current.json"));
        assert_eq!(
            current.get("generation").and_then(Value::as_u64),
            Some(1),
            "{point}/{boundary}: issue {issue} was applied more than once or not completed"
        );
        assert_eq!(
            regular_file_count(&issue_root.join("intents")),
            1,
            "{point}/{boundary}: issue {issue} has duplicate/missing conversion intents"
        );
        assert_eq!(
            regular_file_count(&issue_root.join("commits")),
            1,
            "{point}/{boundary}: issue {issue} has duplicate/missing semantic commits"
        );

        let projection_root = git_common
            .join("csdlc-v3/local/projections")
            .join(issue.to_string());
        assert!(
            projection_root.join("state.json").is_file(),
            "{point}/{boundary}: issue {issue} projection state is missing"
        );
        assert!(
            projection_root.join("cards/manifest.json").is_file(),
            "{point}/{boundary}: issue {issue} projection manifest is missing"
        );
    }
}

fn assert_remote_effect_is_not_duplicated(
    point: &str,
    boundary: &str,
    git_common: &Path,
    operation_id: &str,
) {
    let mut matching_effects = 0usize;
    for path in all_files(git_common) {
        if !path
            .file_name()
            .is_some_and(|name| name.to_string_lossy().contains("ledger"))
        {
            continue;
        }
        let bytes = fs::read(&path).expect("ledger must remain readable");
        matching_effects += String::from_utf8_lossy(&bytes)
            .matches(operation_id)
            .count();
    }
    assert!(
        matching_effects <= 1,
        "{point}/{boundary}: operation {operation_id} appears {matching_effects} times in effect ledgers"
    );
    if matches!(
        point,
        "fake_remote_request_dispatch"
            | "fake_remote_success_readback"
            | "local_reconciled_success_persistence"
    ) {
        assert_eq!(
            matching_effects, 1,
            "{point}/{boundary}: remote-path recovery did not retain exactly one fake transport effect"
        );
    }
}

fn operation_files(root: &Path, operation_id: &str) -> BTreeMap<PathBuf, Vec<u8>> {
    all_files(root)
        .into_iter()
        .filter_map(|path| {
            let bytes = fs::read(&path).ok()?;
            (String::from_utf8_lossy(&bytes).contains(operation_id)).then_some((path, bytes))
        })
        .collect()
}

fn all_files(root: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    collect_files(root, &mut files);
    files.sort();
    files
}

fn collect_files(root: &Path, files: &mut Vec<PathBuf>) {
    let Ok(entries) = fs::read_dir(root) else {
        return;
    };
    for entry in entries {
        let entry = entry.expect("fixture directory entry must be readable");
        let path = entry.path();
        let file_type = entry
            .file_type()
            .expect("fixture directory entry type must be readable");
        if file_type.is_dir() {
            collect_files(&path, files);
        } else if file_type.is_file() {
            files.push(path);
        }
    }
}

fn regular_file_count(root: &Path) -> usize {
    fs::read_dir(root)
        .unwrap_or_else(|error| panic!("{} must be readable: {error}", root.display()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_ok_and(|kind| kind.is_file()))
        .count()
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(
        &fs::read(path)
            .unwrap_or_else(|error| panic!("{} must be readable: {error}", path.display())),
    )
    .unwrap_or_else(|error| panic!("{} must contain JSON: {error}", path.display()))
}

struct Fixture {
    root: PathBuf,
    primary: PathBuf,
    linked: PathBuf,
    git_common: PathBuf,
    linked_head: String,
}

impl Fixture {
    fn new(point: &str, boundary: &str) -> Self {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock must be after Unix epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "csdlc-v3-issue872-{point}-{boundary}-{}-{nonce}",
            std::process::id()
        ));
        let primary = root.join("primary");
        let linked = root.join("linked");
        fs::create_dir_all(&primary).expect("isolated primary must be created");
        git(&["init", "-q", "-b", "main"], Some(&primary));
        git(
            &["config", "user.email", "issue872@example.invalid"],
            Some(&primary),
        );
        git(&["config", "user.name", "Issue 872 test"], Some(&primary));
        fs::write(primary.join("seed"), b"isolated fixture\n").expect("seed must be written");
        git(&["add", "seed"], Some(&primary));
        git(&["commit", "-qm", "seed"], Some(&primary));
        git(&["branch", "codex/fixture-linked"], Some(&primary));
        git(
            &[
                "worktree",
                "add",
                "-q",
                linked.to_str().expect("linked path must be UTF-8"),
                "codex/fixture-linked",
            ],
            Some(&primary),
        );
        let linked_head = git_stdout(&["rev-parse", "HEAD"], &linked);
        let git_common = primary.join(".git");

        let source =
            Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/issue872-copied-records");
        let copied = primary.join("copied-records");
        for (issue, _) in RECORDS {
            copy_tree(
                &source.join(issue.to_string()),
                &copied.join(issue.to_string()),
            );
        }
        let registry_dir = primary.join("docs/templates/prompts");
        fs::create_dir_all(&registry_dir).expect("registry directory must be created");
        fs::copy(
            Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/templates/prompts/current.json"),
            registry_dir.join("current.json"),
        )
        .expect("active registry must be copied into isolated fixture");
        fs::write(root.join("authority.bytes"), b"isolated-authority\n")
            .expect("authority fixture must be written");

        Self {
            root,
            primary,
            linked,
            git_common,
            linked_head,
        }
    }

    fn write_request(&self, operation_id: &str, point: &str, boundary: &str) -> PathBuf {
        let records = RECORDS
            .iter()
            .map(|(issue, role)| {
                json!({
                    "issue": issue,
                    "role": role,
                    "source": self.primary.join("copied-records").join(issue.to_string()),
                })
            })
            .collect::<Vec<_>>();
        let request = json!({
            "schema": "csdlc.v3.copied_record_conversion.v1",
            "repository": "isolated/rehearsal",
            "operation_id": operation_id,
            "authority_bytes_path": self.root.join("authority.bytes"),
            "git_common": self.git_common,
            "linked_branch": "codex/fixture-linked",
            "linked_head": self.linked_head,
            "linked_worktree": self.linked,
            "records": records,
            "registry_path": self.primary.join("docs/templates/prompts/current.json"),
            "fault_injection": {
                "point": point,
                "boundary": boundary,
                "mode": "once"
            }
        });
        let path = self.root.join("request.json");
        fs::write(
            &path,
            serde_json::to_vec_pretty(&request).expect("request must serialize"),
        )
        .expect("request must be written");
        path
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn git(args: &[&str], cwd: Option<&Path>) {
    let mut command = Command::new("git");
    command.args(args);
    if let Some(cwd) = cwd {
        command.current_dir(cwd);
    }
    let output = command.output().expect("git process must start");
    assert!(
        output.status.success(),
        "git {:?} failed: stdout={} stderr={}",
        args,
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn git_stdout(args: &[&str], cwd: &Path) -> String {
    let output = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .expect("git process must start");
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout)
        .expect("git stdout must be UTF-8")
        .trim()
        .to_owned()
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination)
        .unwrap_or_else(|error| panic!("{} must be created: {error}", destination.display()));
    for entry in fs::read_dir(source)
        .unwrap_or_else(|error| panic!("{} must be readable: {error}", source.display()))
    {
        let entry = entry.expect("source fixture entry must be readable");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        let file_type = entry
            .file_type()
            .expect("source fixture type must be readable");
        if file_type.is_dir() {
            copy_tree(&source_path, &destination_path);
        } else if file_type.is_file() {
            fs::copy(&source_path, &destination_path).unwrap_or_else(|error| {
                panic!(
                    "{} must copy to {}: {error}",
                    source_path.display(),
                    destination_path.display()
                )
            });
        } else {
            panic!("unsupported fixture entry {}", source_path.display());
        }
    }
}
