// PVF: deterministic local source-inventory contract; small filesystem fixtures;
// no provider/network execution, installed-product admission, or release gate.
use adl_public_contracts::tool_registry::{build, execute};
use std::{fs, process::Command};
fn fixture() -> tempfile::TempDir {
    let temp = tempfile::tempdir().unwrap();
    fs::create_dir_all(temp.path().join("adl/tools")).unwrap();
    fs::create_dir_all(temp.path().join("docs/tooling")).unwrap();
    fs::write(
        temp.path().join("adl/Cargo.toml"),
        "[package]\nname = \"not-a-binary\"\n[[bin]]\nname = \"adl\"\n",
    )
    .unwrap();
    fs::write(
        temp.path().join("adl/tools/existing_helper.sh"),
        "#!/bin/sh\n",
    )
    .unwrap();
    temp
}
#[test]
fn empty_trees_and_nested_assets_preserve_directory_inventory() {
    let temp = fixture();
    let root = temp.path();
    let clean = build(root).unwrap();
    fs::create_dir_all(root.join("adl/tools/mcp/midi-bridge/nested-empty")).unwrap();
    fs::create_dir_all(root.join("adl/tools/empty")).unwrap();
    assert_eq!(clean, build(root).unwrap());
    let asset = root.join("adl/tools/assets/nested/input.json");
    fs::create_dir_all(asset.parent().unwrap()).unwrap();
    fs::write(&asset, "{}").unwrap();
    let populated = build(root).unwrap();
    assert_eq!(populated.matches("| `adl/tools/assets` | tool surface | internal helper | Directory containing bounded helper assets, scripts, or proof inputs. |").count(), 1);
    assert!(!populated.contains("`adl/tools/assets/nested`"));
    assert!(
        populated.contains("| `adl/tools/existing_helper.sh` | tool surface | internal helper |")
    );
    fs::remove_file(asset).unwrap();
    assert_eq!(clean, build(root).unwrap());
}
#[test]
fn classifications_and_explicit_binaries_are_source_only() {
    let temp = fixture();
    let root = temp.path();
    for name in [
        "batched_checks.sh",
        "run_pr_fast_test_lane.sh",
        "test_example.sh",
        "demo_example.sh",
        "mock_example.sh",
        "validate_example.sh",
        "legacy.py",
        "misc",
    ] {
        fs::write(root.join("adl/tools").join(name), "").unwrap();
    }
    for name in ["pr-run", "pr-init", "stp-editor", "other"] {
        let path = root.join("adl/tools/skills").join(name);
        fs::create_dir_all(&path).unwrap();
        fs::write(path.join("SKILL.md"), "").unwrap();
    }
    let doc = build(root).unwrap();
    for (name, class) in [
        ("run_pr_fast_test_lane.sh", "historical evidence"),
        ("batched_checks.sh", "primary"),
        ("test_example.sh", "internal helper"),
    ] {
        assert!(doc.contains(&format!("| `adl/tools/{name}` | tool surface | {class} |")));
    }
    for (name, class) in [
        ("pr-run", "historical evidence"),
        ("pr-init", "primary"),
        ("stp-editor", "supported shim"),
        ("other", "internal helper"),
    ] {
        assert!(doc.contains(&format!("| `adl/tools/skills/{name}` | skill | {class} |")));
    }
    assert!(!doc.contains("not-a-binary"));
    assert!(doc.contains("- `primary`: 7\n"));
    assert!(doc.contains("- `supported shim`: 1\n"));
    assert!(doc.contains("- `historical evidence`: 6\n"));
    assert!(doc.contains("installed skill admission is not inventoried"));
    assert!(doc.contains("This registry does not approve final deletion"));
}
#[test]
fn freshness_refuses_stale_output_and_never_edits_history() {
    let temp = fixture();
    let root = temp.path();
    let historic = root.join("docs/tooling/historical.md");
    fs::write(&historic, "immutable").unwrap();
    assert!(execute(root, true).is_err());
    execute(root, false).unwrap();
    execute(root, true).unwrap();
    let path = root.join("docs/tooling/TOOL_SURFACE_REGISTRY.md");
    fs::write(&path, "stale").unwrap();
    assert!(execute(root, true).is_err());
    assert_eq!(fs::read_to_string(&path).unwrap(), "stale");
    execute(root, false).unwrap();
    assert_eq!(fs::read_to_string(historic).unwrap(), "immutable");
    let output = Command::new(env!("CARGO_BIN_EXE_adl-public-contracts"))
        .args(["tool-registry", "--bad"])
        .env("PATH", "")
        .output()
        .unwrap();
    assert!(!output.status.success());
}
