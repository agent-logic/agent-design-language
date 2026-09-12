//! PVF: deterministic local CPU/Rust/Git installed-public-contract proof; required
//! issue #868 acceptance and SIM-07 qualification input. No network or live owner
//! replacement. Discovery checks do not substitute for operational journey proof.
#[path = "support/installed_contract.rs"]
mod installed_contract;
use csdlc_v3::commands::contract::verify_discovery;
use installed_contract::{git, Installation};
use serde_json::Value;
use std::{collections::BTreeSet, fs, path::Path};
const FROZEN: [&str; 27] = [
    "foundation",
    "local",
    "bind",
    "clean",
    "cutover",
    "doctor",
    "edit",
    "eligibility",
    "finish",
    "github",
    "github-issue",
    "github-pr",
    "install",
    "issue",
    "pr-state",
    "proof",
    "publish",
    "review",
    "remote",
    "schedule",
    "shadow",
    "shepherd",
    "soak",
    "sprint",
    "validate",
    "release-preflight",
    "rollback",
];
#[test]
fn installed_discovery_preserves_all_frozen_dispositions_and_rejects_drift() {
    let install = Installation::new();
    let help = install.run(&install.root, &["--help"]);
    assert!(help.status.success());
    assert!(help.stderr.is_empty());
    let help = String::from_utf8(help.stdout).unwrap();
    let output = install.run(&install.root, &["--contract"]);
    assert!(output.status.success());
    let observed: Value = serde_json::from_slice(&output.stdout).unwrap();
    verify_discovery(&observed, &help).unwrap();
    let rows: Vec<_> = observed["commands"]
        .as_array()
        .unwrap()
        .iter()
        .chain(observed["aliases"].as_array().unwrap())
        .collect();
    // Preserve the complete SIM-02 denominator while adding three ordinary
    // intent entrypoints; their successful journeys have separate SIM-03 proof.
    assert_eq!(rows.len(), 30);
    assert_eq!(
        rows.iter()
            .map(|r| r["command"].as_str().unwrap())
            .collect::<BTreeSet<_>>(),
        FROZEN
            .into_iter()
            .chain(["status", "prepare", "recover"])
            .collect()
    );
    for row in rows {
        let name = row["command"].as_str().unwrap();
        assert!(
            help.lines()
                .any(|line| line.trim() == row["usage"].as_str().unwrap()),
            "missing root help: {name}"
        );
        let describe = install.run(&install.root, &[name, "--describe"]);
        assert!(describe.status.success(), "describe {name}");
        assert_eq!(
            serde_json::from_slice::<Value>(&describe.stdout).unwrap(),
            *row
        );
        let route_help = install.run(&install.root, &[name, "--help"]);
        assert!(route_help.status.success(), "help {name}: {route_help:?}");
        assert!(!route_help.stdout.is_empty());
        assert!(!row["input_contract"].as_str().unwrap().is_empty());
        assert_eq!(row["result_schema"], "csdlc.v3.command_result.v1");
    }
    for field in ["effect_class", "input_contract", "result_schema"] {
        let mut mismatched = observed.clone();
        mismatched["commands"][0][field] = Value::String("mismatched-fixture".into());
        assert!(
            verify_discovery(&mismatched, &help).is_err(),
            "accepted {field} mismatch"
        );
    }
    let mut omitted = observed.clone();
    omitted["commands"]
        .as_array_mut()
        .unwrap()
        .retain(|r| r["command"] != "release-preflight");
    assert!(verify_discovery(&omitted, &help).is_err());
    let old_help = help
        .lines()
        .filter(|line| !line.contains("release-preflight"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(verify_discovery(&observed, &old_help).is_err());
}
#[test]
fn installed_bad_requests_in_primary_and_real_linked_checkout_never_fall_back() {
    let install = Installation::new();
    let primary = install.root.join("primary");
    let linked = install.root.join("linked");
    fs::create_dir_all(&primary).unwrap();
    git(&primary, &["init", "-b", "main"]);
    git(
        &primary,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "--allow-empty",
            "-m",
            "fixture",
        ],
    );
    git(
        &primary,
        &["worktree", "add", "--detach", linked.to_str().unwrap()],
    );
    assert!(primary.join(".git").is_dir());
    assert!(linked.join(".git").is_file());
    let request = install.root.join("malformed.json");
    fs::write(&request, b"{broken").unwrap();
    let registry = install.root.join("registry.json");
    fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../docs/templates/prompts/current.json"),
        &registry,
    )
    .unwrap();
    let registrations = install.root.join("registrations.json");
    fs::write(&registrations, b"[]").unwrap();
    let before = installed_contract::inventory(&install.root);
    for cwd in [&primary, &linked] {
        for route in [
            "issue",
            "bind",
            "edit",
            "doctor",
            "validate",
            "eligibility",
            "schedule",
            "shepherd",
            "github-pr",
            "review",
            "publish",
            "finish",
            "clean",
            "rollback",
            "unknown-fixture-command",
        ] {
            let output = install.run(cwd, &[route, "--request", request.to_str().unwrap()]);
            assert!(
                !output.status.success(),
                "{route}: successful fallback: {output:?}"
            );
            let report: Value =
                serde_json::from_slice(&output.stdout).expect("exactly one JSON error envelope");
            assert_eq!(report["envelope"]["command"], route);
            assert_eq!(report["envelope"]["schema"], "csdlc.v3.command_result.v1");
            assert_ne!(report["envelope"]["authority_status"], "verified");
            assert_eq!(report["envelope"]["process_status"], "failed");
            assert_ne!(report["envelope"]["effects"]["outcome"], "performed");
            assert_eq!(
                report["envelope"]["allowed_next_operations"],
                serde_json::json!([])
            );
            assert_eq!(
                installed_contract::inventory(&install.root),
                before,
                "{route} changed fixture bytes"
            );
        }
    }
    // These complete argv cases reach the typed JSON parser, unlike the local
    // missing-registry/registrations cases above. Neither is authority proof.
    for cwd in [&primary, &linked] {
        for route in [
            "issue",
            "bind",
            "edit",
            "doctor",
            "validate",
            "eligibility",
            "schedule",
            "shepherd",
        ] {
            let output = install.run(
                cwd,
                &[
                    route,
                    "--request",
                    request.to_str().unwrap(),
                    "--registry",
                    registry.to_str().unwrap(),
                    "--registrations",
                    registrations.to_str().unwrap(),
                ],
            );
            assert!(!output.status.success(), "{route}: {output:?}");
            let report: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(
                report["envelope"]["reason_code"], "typed_contract_invalid_json",
                "{route}: {report}"
            );
            assert_eq!(report["envelope"]["process_status"], "failed");
            assert_ne!(report["envelope"]["effects"]["outcome"], "performed");
            assert_eq!(installed_contract::inventory(&install.root), before);
        }
    }
}

#[test]
fn installed_provenance_rejects_stale_bytes_metadata_and_source() {
    use csdlc_v3::commands::contract::verify_installation;
    use std::process::Command;
    let install = Installation::new();
    let bytes = fs::read(&install.binary).unwrap();
    let expected_digest = blake3::hash(&fs::read(env!("CARGO_BIN_EXE_csdlc")).unwrap())
        .to_hex()
        .to_string();
    let head = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .unwrap();
    assert!(head.status.success());
    let source = String::from_utf8(head.stdout).unwrap().trim().to_owned();
    let rustc = Command::new("rustc").arg("--version").output().unwrap();
    assert!(rustc.status.success());
    let build = format!(
        "cargo-test candidate; {} {}; {}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        String::from_utf8(rustc.stdout).unwrap().trim()
    );
    let provenance = serde_json::json!({"schema":"csdlc.v3.installation_provenance.v1", "source_revision":source, "build_identity":build, "installed_digest":expected_digest});
    fs::write(
        install.root.join("provenance.json"),
        serde_json::to_vec_pretty(&provenance).unwrap(),
    )
    .unwrap();
    verify_installation(&provenance, &bytes, &source, &build, &expected_digest).unwrap();
    let export = Path::new(env!("CARGO_MANIFEST_DIR")).join("target/sim02-envelope-samples");
    fs::create_dir_all(&export).unwrap();
    fs::write(
        export.join("installation-provenance.json"),
        serde_json::to_vec_pretty(&provenance).unwrap(),
    )
    .unwrap();
    for field in [
        "schema",
        "source_revision",
        "build_identity",
        "installed_digest",
    ] {
        let mut stale = provenance.clone();
        stale[field] = Value::String("stale-fixture".into());
        assert!(
            verify_installation(&stale, &bytes, &source, &build, &expected_digest).is_err(),
            "accepted stale {field}"
        );
    }
    let stale_bytes = b"synthetic stale installed executable";
    assert!(
        verify_installation(&provenance, stale_bytes, &source, &build, &expected_digest).is_err()
    );
    let mut forged = provenance.clone();
    forged["installed_digest"] = Value::String(blake3::hash(stale_bytes).to_hex().to_string());
    assert!(
        verify_installation(&forged, stale_bytes, &source, &build, &expected_digest).is_err(),
        "self-consistent stale bytes must not match independently expected candidate"
    );
}

#[test]
fn installed_valid_local_input_rejects_stale_authority_but_explicit_inspection_is_non_operational()
{
    use csdlc_v3::commands::local::{required_local_commands, LocalPreparationRequest};
    let install = Installation::new();
    let primary = install.root.join("primary");
    let linked = install.root.join("linked");
    fs::create_dir_all(primary.join("csdlc-v3/operator")).unwrap();
    git(&primary, &["init", "-b", "main"]);
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("operator");
    for name in ["authority-selector.json", "native-authority-receipt.json"] {
        fs::copy(
            source.join(name),
            primary.join("csdlc-v3/operator").join(name),
        )
        .unwrap();
    }
    // Preserve the selector's expected digest while changing receipt bytes.
    let receipt = primary.join("csdlc-v3/operator/native-authority-receipt.json");
    let mut bytes = fs::read(&receipt).unwrap();
    bytes.push(b' ');
    fs::write(&receipt, bytes).unwrap();
    fs::create_dir_all(primary.join(".adl")).unwrap();
    fs::write(primary.join(".adl/worktree-policy.json"), serde_json::to_vec(&serde_json::json!({"schema":"adl.worktree_policy.v1", "required_parent":install.root.join("worktrees")})).unwrap()).unwrap();
    git(&primary, &["add", "."]);
    git(
        &primary,
        &[
            "-c",
            "user.name=Fixture",
            "-c",
            "user.email=fixture@example.invalid",
            "commit",
            "-m",
            "stale receipt fixture",
        ],
    );
    git(
        &primary,
        &["update-ref", "refs/remotes/origin/main", "HEAD"],
    );
    git(
        &primary,
        &["worktree", "add", "--detach", linked.to_str().unwrap()],
    );
    let request = LocalPreparationRequest {
        issue: 868,
        title: "Installed authority fixture".into(),
        repository: "agent-logic/agent-design-language".into(),
        branch: "codex/868-installed-fixture".into(),
        worktree: install
            .root
            .join("worktrees/issue868")
            .to_string_lossy()
            .into_owned(),
        registry_version: "1.0.5".into(),
        expected_lifecycle_digest: None,
        schedule_readiness: None,
        shepherd_routing: None,
        commands: required_local_commands().to_vec(),
        card_updates: std::collections::BTreeMap::new(),
    };
    let request_path = install.root.join("valid.json");
    fs::write(&request_path, serde_json::to_vec(&request).unwrap()).unwrap();
    let registry = install.root.join("registry.json");
    fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../docs/templates/prompts/current.json"),
        &registry,
    )
    .unwrap();
    let registrations = install.root.join("registrations.json");
    fs::write(&registrations, serde_json::to_vec(&serde_json::json!([{"branch":request.branch, "worktree":request.worktree, "primary":false}])).unwrap()).unwrap();
    let before = installed_contract::inventory(&install.root);
    for cwd in [&primary, &linked] {
        for route in ["issue", "doctor", "local"] {
            let output = install.run(
                cwd,
                &[
                    route,
                    "--request",
                    request_path.to_str().unwrap(),
                    "--registry",
                    registry.to_str().unwrap(),
                    "--registrations",
                    registrations.to_str().unwrap(),
                ],
            );
            let report: Value = serde_json::from_slice(&output.stdout).unwrap();
            if route == "local" {
                assert!(output.status.success(), "{report}");
                assert_eq!(report["operational_authority"], false);
                assert_eq!(report["envelope"]["effects"]["outcome"], "none");
            } else {
                assert!(!output.status.success(), "{report}");
                assert_eq!(
                    report["envelope"]["reason_code"], "operational_context_required",
                    "{report}"
                );
                assert_eq!(report["envelope"]["status"], "blocked");
            }
            assert_eq!(
                installed_contract::inventory(&install.root),
                before,
                "{route} mutated fixture"
            );
        }
    }
}

#[test]
fn every_frozen_installed_route_dispatches_and_required_flags_are_enforced() {
    let install = Installation::new();
    let request = install.root.join("malformed.json");
    fs::write(&request, b"{broken").unwrap();
    let historical = install.root.join("historical.json");
    fs::write(&historical, serde_json::to_vec(&serde_json::json!({"issue":868, "repository":"agent-logic/agent-design-language", "cutover_issue":505, "evidence_root":install.root, "operator_approval":"synthetic historical fixture", "proof":null, "shadow":null, "soak":null, "install":null})).unwrap()).unwrap();
    let registry = install.root.join("registry.json");
    fs::copy(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../docs/templates/prompts/current.json"),
        &registry,
    )
    .unwrap();
    let registrations = install.root.join("registrations.json");
    fs::write(&registrations, b"[]").unwrap();
    let contract: Value =
        serde_json::from_slice(&install.run(&install.root, &["--contract"]).stdout).unwrap();
    let before = installed_contract::inventory(&install.root);
    let mut count = 0;
    for row in contract["commands"]
        .as_array()
        .unwrap()
        .iter()
        .chain(contract["aliases"].as_array().unwrap())
        .filter(|row| FROZEN.contains(&row["command"].as_str().unwrap()))
    {
        let name = row["command"].as_str().unwrap();
        let variant = &row["input_variants"][0];
        let flags = variant["required_flags"].as_array().unwrap();
        let mut argv = vec![name.to_owned()];
        for flag in flags {
            let flag = flag.as_str().unwrap();
            argv.push(flag.into());
            argv.push(match flag {
                "--request" if matches!(name, "shadow" | "soak") => {
                    historical.to_str().unwrap().into()
                }
                "--request" => request.to_str().unwrap().into(),
                "--registry" => registry.to_str().unwrap().into(),
                "--registrations" => registrations.to_str().unwrap().into(),
                "--repo-root" if name == "foundation" => install
                    .root
                    .join("absent-repository")
                    .to_str()
                    .unwrap()
                    .into(),
                "--repo-root" => install.root.to_str().unwrap().into(),
                _ => panic!("unclassified required flag {flag}"),
            });
        }
        let args: Vec<_> = argv.iter().map(String::as_str).collect();
        let output = install.run(&install.root, &args);
        if name == "remote" {
            assert!(output.status.success());
            assert!(String::from_utf8_lossy(&output.stdout).contains("routes: github"));
        } else {
            assert!(!output.status.success(), "{name}: {output:?}");
            let report: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert_eq!(report["envelope"]["command"], name);
            let rendered = report.to_string();
            let expected = match name {
                "local" | "issue" | "bind" | "edit" | "doctor" | "validate" | "eligibility"
                | "schedule" | "shepherd" => "typed_contract_invalid_json",
                "shadow" | "soak" => "historical_route_disabled",
                "install" | "proof" => "invalid request json",
                "github" | "github-issue" | "github-pr" | "pr-state" | "review" | "publish" => {
                    "typed_remote_request_invalid_json"
                }
                "finish" | "clean" | "cutover" | "rollback" => {
                    "typed_terminal_request_invalid_json"
                }
                "sprint" => "RequestInvalidJson",
                "release-preflight" => "release request malformed",
                "foundation" => "absent-repository",
                _ => panic!("unclassified route {name}"),
            };
            assert!(
                rendered.contains(expected),
                "{name} did not reach intended stage {expected}: {report}"
            );
            assert_eq!(report["envelope"]["process_status"], "failed");
            assert_ne!(report["envelope"]["effects"]["outcome"], "performed");
        }
        assert_eq!(
            installed_contract::inventory(&install.root),
            before,
            "{name} changed fixture"
        );
        // Remove each declared required flag/value independently; successful
        // request dispatch must not silently supply missing operator inputs.
        for (index, flag) in flags.iter().enumerate() {
            let mut missing = argv.clone();
            missing.drain(1 + index * 2..3 + index * 2);
            let missing: Vec<_> = missing.iter().map(String::as_str).collect();
            let output = install.run(&install.root, &missing);
            assert!(!output.status.success(), "{name} accepted missing {}", flag);
            let report: Value = serde_json::from_slice(&output.stdout).unwrap();
            assert!(
                report.to_string().contains("usage:"),
                "{name} missingflag stage: {report}"
            );
            assert_eq!(installed_contract::inventory(&install.root), before);
        }
        count += 1;
    }
    assert_eq!(count, 27);
    for operation in ["create", "close"] {
        for help in ["--help", "-h"] {
            let output = install.run(&install.root, &["github-issue", operation, help]);
            assert!(output.status.success(), "{operation} {help}: {output:?}");
            assert!(String::from_utf8_lossy(&output.stdout)
                .contains(&format!("github-issue {operation}")));
            assert!(output.stderr.is_empty());
        }
    }
    assert_eq!(installed_contract::inventory(&install.root), before);
}
