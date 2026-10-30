// PVF: deterministic local source-shape contract; tiny CPU/disk, no network.
// Regression proof only, not Windows/QTS execution or deployed SSM acceptance.
use std::{fs, process::Command};
#[test]
fn status_scripts_preserve_positive_and_each_missing_fragment_refusal() {
    let root = adl_public_contracts::root();
    let fixture = tempfile::tempdir().unwrap();
    fs::create_dir_all(fixture.path().join("adl/tools")).unwrap();
    let required = [
        "schema_version",
        "generated_at_utc",
        "host_label",
        "os_name",
        "os_version",
        "repo_name",
        "repo_present",
        "git_branch",
        "git_commit_short",
        "ssm_agent_installed",
    ];
    for (kind, extension) in [("qts", "sh"), ("windows", "ps1")] {
        let script = root.join(format!("adl/tools/validate_polis_status_for_ssm_{kind}.sh"));
        let relative = format!("adl/tools/polis_status_for_ssm_{kind}.{extension}");
        let original = fs::read_to_string(root.join(&relative)).unwrap();
        let source = fixture.path().join(&relative);
        let run = || {
            Command::new("bash")
                .arg(&script)
                .current_dir(fixture.path())
                .output()
                .unwrap()
        };
        fs::write(&source, &original).unwrap();
        let good = run();
        assert!(
            good.status.success(),
            "{}",
            String::from_utf8_lossy(&good.stderr)
        );
        assert_eq!(
            String::from_utf8(good.stdout).unwrap(),
            format!("validate_polis_status_for_ssm_{kind}: ok\n")
        );
        let mut fragments = required.to_vec();
        if kind == "windows" {
            fragments.push("ConvertTo-Json");
        }
        for fragment in fragments {
            fs::write(&source, original.replace(fragment, "REMOVED")).unwrap();
            let bad = run();
            assert_eq!(bad.status.code(), Some(1));
            assert!(String::from_utf8_lossy(&bad.stderr)
                .contains(&format!("missing required fragments: {fragment}")));
        }
        fs::remove_file(&source).unwrap();
        assert_eq!(run().status.code(), Some(1));
    }
}
