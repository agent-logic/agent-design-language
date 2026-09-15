use serde_json::{json, Value};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const ROLES: [&str; 7] = [
    "prepared",
    "bound_dirty",
    "implemented",
    "reviewed",
    "published",
    "terminal",
    "pending_recovery",
];

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = Command::new("chmod")
            .args(["-R", "u+w"])
            .arg(&self.0)
            .status();
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn command(cwd: &Path, program: &str, args: &[&str]) {
    let output = Command::new(program)
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{} {:?}: {}",
        program,
        args,
        String::from_utf8_lossy(&output.stderr)
    );
}

fn write_json(path: &Path, value: &Value) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(
        path,
        format!("{}\n", serde_json::to_string_pretty(value).unwrap()),
    )
    .unwrap();
}

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .to_path_buf()
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).unwrap();
    for entry in fs::read_dir(source).unwrap() {
        let entry = entry.unwrap();
        let target = destination.join(entry.file_name());
        if entry.file_type().unwrap().is_dir() {
            copy_tree(&entry.path(), &target);
        } else {
            fs::copy(entry.path(), target).unwrap();
        }
    }
}

#[test]
fn copied_record_conversion_rehearsal_executes_complete_isolated_denominator() {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let fixture = Fixture(std::env::temp_dir().join(format!("csdlc-872-{nonce}")));
    let primary = fixture.0.join("primary");
    let linked = fixture.0.join("linked");
    fs::create_dir_all(&primary).unwrap();
    command(&primary, "git", &["init", "-q"]);
    command(&primary, "git", &["config", "user.name", "C-SDLC Fixture"]);
    command(
        &primary,
        "git",
        &["config", "user.email", "fixture@example.invalid"],
    );
    fs::write(primary.join("README.md"), "isolated issue 872 fixture\n").unwrap();
    command(&primary, "git", &["add", "README.md"]);
    command(&primary, "git", &["commit", "-qm", "fixture"]);
    command(
        &primary,
        "git",
        &[
            "worktree",
            "add",
            "-qb",
            "fixture-linked",
            linked.to_str().unwrap(),
        ],
    );
    write_json(
        &fixture.0.join(".csdlc-conversion-rehearsal.json"),
        &json!({"isolated": true}),
    );

    let source = primary.join("copied-records");
    let mut role_values = Vec::new();
    for (offset, role) in ROLES.iter().enumerate() {
        let issue = 1001 + offset as u64;
        let role_root = source.join(issue.to_string());
        let phase = match *role {
            "prepared" => "ready",
            "bound_dirty" => "bound",
            "implemented" => "implemented",
            "reviewed" => "reviewed",
            "published" => "published",
            "terminal" => "terminal",
            _ => "bound",
        };
        write_json(
            &role_root.join("index.json"),
            &json!({
                "schema": "csdlc.v3.issue_index.v1", "issue": issue,
                "generation": offset + 1, "digest": format!("digest-{role}"),
                "phase": phase, "role_marker": role,
                "pending_transaction": (*role == "pending_recovery").then_some("operation-pending")
            }),
        );
        for kind in ["sip", "stp", "spp", "vpp", "srp", "sor"] {
            write_json(
                &role_root.join("cards").join(format!("{kind}.values.json")),
                &json!({
                    "schema": format!("csdlc.card.{kind}.values.v1"), "issue": issue,
                    "role": role, "phase": phase
                }),
            );
            fs::write(
                role_root.join("cards").join(format!("{kind}.md")),
                format!("# {kind} for {role}\n"),
            )
            .unwrap();
        }
        let evidence = role_root.join("evidence");
        fs::create_dir_all(&evidence).unwrap();
        fs::write(
            evidence.join(format!("{role}.receipt")),
            format!("immutable-{role}\n"),
        )
        .unwrap();
        role_values.push(json!({"role": role, "issue": issue, "source": role_root}));
    }
    fs::write(
        source.join("1002").join("dirty-tracked.txt"),
        "dirty tracked byte\n",
    )
    .unwrap();
    fs::write(
        source.join("1002").join("untracked.txt"),
        "dirty untracked byte\n",
    )
    .unwrap();

    let old = fixture.0.join("old-csdlc");
    let candidate = fixture.0.join("candidate-csdlc");
    fs::write(&old, format!("#!/bin/sh\nif [ \"$CSDLC_CONVERSION_FENCE\" != absent ]; then exit 23; fi\nprintf control >> '{}'\n", source.join("writer-control.log").display())).unwrap();
    fs::write(&candidate, "#!/bin/sh\nexit 0\n").unwrap();
    command(
        &fixture.0,
        "chmod",
        &["+x", old.to_str().unwrap(), candidate.to_str().unwrap()],
    );

    let output = primary.join(".csdlc/evidence/872/conversion-rehearsal");
    let request_path = fixture.0.join("request.json");
    write_json(
        &request_path,
        &json!({
            "schema": "csdlc.v3.copied_record_conversion_rehearsal_request.v1",
            "fixture_root": fixture.0, "primary": primary, "linked_worktree": linked,
            "source_root": source, "output_root": output,
            "old_executable": old, "candidate_executable": candidate,
            "old_writer_command": [old], "roles": role_values
        }),
    );

    let repo = repository_root();
    let run = Command::new(repo.join("adl/tools/csdlc-conversion-rehearsal"))
        .args(["--request", request_path.to_str().unwrap()])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(
        run.status.success(),
        "runner stdout={} stderr={}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let report: Value = serde_json::from_slice(&run.stdout).unwrap();
    assert_eq!(report["status"], "passed");
    assert_eq!(
        report["proof_denominator"],
        json!({"roles": 7, "scenarios": 12, "fault_cases": 30})
    );

    let validate = Command::new("python3")
        .args([
            repo.join("adl/tools/validate_issue872_conversion_rehearsal.py")
                .to_str()
                .unwrap(),
            "--evidence-root",
            output.to_str().unwrap(),
        ])
        .current_dir(&repo)
        .output()
        .unwrap();
    assert!(
        validate.status.success(),
        "validator stdout={} stderr={}",
        String::from_utf8_lossy(&validate.stdout),
        String::from_utf8_lossy(&validate.stderr)
    );
    if let Some(destination) = std::env::var_os("ISSUE872_RETAIN_EVIDENCE") {
        let destination = PathBuf::from(destination);
        if destination.exists() {
            fs::remove_dir_all(&destination).unwrap();
        }
        copy_tree(&output, &destination);
    }
}
