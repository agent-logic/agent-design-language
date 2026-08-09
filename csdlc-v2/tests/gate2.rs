use std::fs;
use std::path::Path;
use std::process::{Command, Output};

use csdlc_v2::cards::{PlanStep, ResourceProfile, StepStatus, ValidationLane};
use csdlc_v2::{
    bind_issue, edit_issue, BindRequest, BootstrapRequest, CardKind, EditRequest, InitialCardInput,
    LifecyclePhase, PlanningProfile, SemanticOperation, Store,
};

fn command(root: &Path, program: &str, args: &[&str]) -> Output {
    Command::new(program)
        .current_dir(root)
        .args(args)
        .output()
        .unwrap_or_else(|error| panic!("run {program}: {error}"))
}

fn must_succeed(output: Output) -> String {
    assert!(
        output.status.success(),
        "stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("UTF-8 output")
}

fn git(root: &Path, args: &[&str]) -> String {
    must_succeed(command(root, "git", args))
}

fn copy_directory(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("copy destination");
    for entry in fs::read_dir(source).expect("copy source") {
        let entry = entry.expect("copy entry");
        let destination = destination.join(entry.file_name());
        if entry.file_type().expect("copy file type").is_dir() {
            copy_directory(&entry.path(), &destination);
        } else {
            fs::copy(entry.path(), destination).expect("copy file");
        }
    }
}

fn focused_fixture_repo(root: &Path) {
    fs::create_dir_all(root.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(root.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(root.join("csdlc-v2/tests")).expect("test directory");
    fs::create_dir_all(root.join("design")).expect("design directory");
    fs::write(
        root.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        root.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(root.join("csdlc-v2/tests/gate2.rs"), "// fixture\n").expect("test fixture");
    for issue in [42_u64, 43] {
        fs::write(
            root.join(format!("design/issue-{issue}.md")),
            format!("# Approved design for issue {issue}\n"),
        )
        .expect("design fixture");
        fs::write(
            root.join(format!("design/issue-{issue}.mmd")),
            "flowchart LR\n  Create --> Bind\n",
        )
        .expect("diagram fixture");
    }
    git(root, &["init", "-b", "main"]);
    git(root, &["config", "user.email", "test@example.com"]);
    git(root, &["config", "user.name", "C-SDLC Test"]);
    git(
        root,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "fixture"]);
}

fn focused_request(issue: u64) -> BootstrapRequest {
    let mut value = request();
    value.issue = issue;
    value.design_path = format!("design/issue-{issue}.md");
    value.diagram_path = format!("design/issue-{issue}.mmd");
    value.initial.title = format!("Focused bind issue {issue}");
    value.initial.slug = format!("focused-bind-issue-{issue}");
    value.initial.repo_inputs = vec![value.design_path.clone()];
    value.initial.affected_areas =
        vec![value.design_path.clone(), "csdlc-v2/tests/gate2.rs".into()];
    value
}

fn create_focused_issue(repo: &Path, request_root: &Path, issue: u64) {
    let request_path = request_root.join(format!("create-{issue}.json"));
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(&focused_request(issue)).expect("serialize focused request"),
    )
    .expect("focused create request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo.to_string_lossy(),
            "create",
            "--request",
            &request_path.to_string_lossy(),
        ],
    ));
}

fn apply_edit(
    repo: &Path,
    temp: &Path,
    issue: u64,
    card: &str,
    name: &str,
    operation: serde_json::Value,
) {
    let index: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join(format!(".csdlc/issues/{issue}/index.json"))).expect("issue index"),
    )
    .expect("issue index JSON");
    let request = temp.join(format!("{issue}-{name}.json"));
    fs::write(
        &request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": issue,
            "card": card,
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": format!("issue 5795 shaped fixture: {name}"),
            "operation": operation,
        }))
        .expect("serialize edit request"),
    )
    .expect("edit request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &request.to_string_lossy(),
        ],
    ));
}

fn request() -> BootstrapRequest {
    BootstrapRequest {
        issue: 42,
        repository: "agent-logic/agent-design-language".into(),
        actor: "test-operator".into(),
        design_path: "design/issue-42.md".into(),
        diagram_path: "design/issue-42.mmd".into(),
        design_reviewer: "reviewer".into(),
        design_approved: true,
        initial: InitialCardInput {
            title: "Claim-free issue workflow".into(),
            slug: "claim-free-issue-workflow".into(),
            version: "v0.92".into(),
            goal: "Prove the claim-free issue workflow.".into(),
            required_outcome: "Create, validate, diagnose, and bind one issue.".into(),
            declared_scope: vec!["claim-free workflow".into()],
            authority_boundary: vec!["local test repository".into()],
            operator_constraints: vec!["no network".into()],
            task_boundary: "Exercise only the focused binary path.".into(),
            deliverables: vec![
                "bound issue record".into(),
                "csdlc-v2/tests/gate2.rs".into(),
            ],
            acceptance_criteria: vec![
                "issue creation is claim-free".into(),
                "binding is atomic and idempotent".into(),
            ],
            dependencies: vec!["none".into()],
            repo_inputs: vec!["design/issue-42.md".into()],
            non_goals: vec!["publication".into()],
            plan_summary: "Create, validate, diagnose, and bind.".into(),
            steps: vec![PlanStep {
                id: "step-1".into(),
                action: "run the focused workflow".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                status: StepStatus::Pending,
            }],
            affected_areas: vec![
                "design/issue-42.md".into(),
                "csdlc-v2/tests/gate2.rs".into(),
            ],
            invariants: vec!["Git topology is binding authority".into()],
            risks: vec!["conflicting worktree".into()],
            planning_profile: PlanningProfile::Small,
            stop_conditions: vec!["topology conflict".into()],
            validation_lanes: vec![ValidationLane {
                lane: "focused".into(),
                proof_role: "actual binary workflow".into(),
                acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
                deterministic: true,
                resource_profile: ResourceProfile::Small,
                budget_seconds: 120,
                budget_tokens: 1_000,
                argv: vec![
                    "cargo".into(),
                    "test".into(),
                    "--manifest-path".into(),
                    "csdlc-v2/Cargo.toml".into(),
                    "--test".into(),
                    "gate2".into(),
                ],
                parallel_group: "local".into(),
                defer_reason: None,
            }],
            failure_policy: "Fail closed on invalid input or topology conflict.".into(),
            review_prompts: vec!["Review atomicity and idempotence.".into()],
            review_scope: "claim-free issue creation and binding".into(),
        },
    }
}

fn distributed_deferred_request(issue: u64, source: &str, test_target: &str) -> BootstrapRequest {
    let test_name = Path::new(test_target)
        .file_stem()
        .and_then(|value| value.to_str())
        .expect("named Rust test target");
    let mut value = request();
    value.issue = issue;
    value.design_path = format!("design/issue-{issue}.md");
    value.diagram_path = format!("design/issue-{issue}.mmd");
    value.initial.title = format!("Distributed deferred target issue {issue}");
    value.initial.slug = format!("distributed-deferred-target-{issue}");
    value.initial.repo_inputs = vec![value.design_path.clone()];
    value.initial.affected_areas = vec![source.into(), test_target.into()];
    value.initial.deliverables = vec![
        source.into(),
        test_target.into(),
        "Focused positive and negative tests".into(),
        "Digest-bound execution proof".into(),
        "Reviewed rollback evidence".into(),
    ];
    value.initial.validation_lanes = vec![ValidationLane {
        lane: "exact-child-tests".into(),
        proof_role: "Exact nextest target proves the issue-owned distributed behavior.".into(),
        acceptance_ids: vec!["AC-1".into(), "AC-2".into()],
        deterministic: true,
        resource_profile: ResourceProfile::Small,
        budget_seconds: 120,
        budget_tokens: 1_000,
        argv: vec![
            "cargo".into(),
            "nextest".into(),
            "run".into(),
            "--manifest-path".into(),
            "adl-runtime/Cargo.toml".into(),
            "--test".into(),
            test_name.into(),
            "--no-tests=fail".into(),
        ],
        parallel_group: "child".into(),
        defer_reason: Some(format!(
            "The issue-owned temporary #[path = \"../src/distributed/...\"] harness will route {source} until integration registration."
        )),
    }];
    value.initial.failure_policy =
        "Fail closed on missing targets, zero tests, invalid evidence, or absent proof.".into();
    value
}

fn create_distributed_deferred_issue(repo: &Path, request_root: &Path, request: &BootstrapRequest) {
    fs::write(
        repo.join(&request.design_path),
        format!("# Approved design for issue {}\n", request.issue),
    )
    .expect("distributed design");
    fs::write(
        repo.join(&request.diagram_path),
        "flowchart LR\n  Prepare --> Bind\n",
    )
    .expect("distributed diagram");
    let request_path = request_root.join(format!("distributed-{}.json", request.issue));
    fs::write(
        &request_path,
        serde_json::to_vec_pretty(request).expect("serialize distributed request"),
    )
    .expect("distributed request");
    must_succeed(command(
        repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo.to_string_lossy(),
            "create",
            "--request",
            &request_path.to_string_lossy(),
        ],
    ));
}

fn distributed_fixture_repo(root: &Path) {
    focused_fixture_repo(root);
    fs::create_dir_all(root.join("adl-runtime/src/distributed"))
        .expect("distributed source directory");
    fs::create_dir_all(root.join("adl-runtime/tests")).expect("distributed tests directory");
    fs::write(
        root.join("adl-runtime/Cargo.toml"),
        "[package]\nname = \"adl-runtime\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )
    .expect("runtime manifest");
    fs::write(
        root.join("adl-runtime/src/lib.rs"),
        "// no distributed route\n",
    )
    .expect("runtime crate root");
    git(root, &["add", "."]);
    git(root, &["commit", "-m", "distributed fixture"]);
}

type DeferredRequestMutation = fn(&mut BootstrapRequest);

#[test]
fn initialized_deferred_distributed_targets_bind_only_through_exact_path_harnesses() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    distributed_fixture_repo(&repo);

    for (issue, source, test_target) in [
        (
            5866,
            "adl-runtime/src/distributed/discovery.rs",
            "adl-runtime/tests/distributed_discovery.rs",
        ),
        (
            5871,
            "adl-runtime/src/distributed/capability_advertisement.rs",
            "adl-runtime/tests/distributed_capability_advertisement.rs",
        ),
        (
            5872,
            "adl-runtime/src/distributed/resource_weather.rs",
            "adl-runtime/tests/distributed_resource_weather.rs",
        ),
    ] {
        let request = distributed_deferred_request(issue, source, test_target);
        create_distributed_deferred_issue(&repo, temp.path(), &request);
        let initialized = csdlc_v2::diagnose(&Store::new(&repo), issue);
        assert_eq!(initialized.status, csdlc_v2::doctor::DoctorStatus::Pass);
        assert!(initialized.ready);
        assert!(initialized.findings.is_empty());

        let worktree = temp.path().join(format!("worktrees/issue-{issue}"));
        bind_issue(
            &Store::new(&repo),
            BindRequest {
                issue,
                base_branch: "main".into(),
                branch: format!("codex/{issue}-distributed-deferred"),
                worktree: worktree.to_string_lossy().into_owned(),
                code_repository: Some("agent-logic/agent-design-language".into()),
            },
        )
        .expect("bind initialized deferred targets");

        let bound = csdlc_v2::diagnose(&Store::new(&worktree), issue);
        assert_eq!(bound.status, csdlc_v2::doctor::DoctorStatus::Block);
        for code in [
            "owned_rust_module_unroutable",
            "validator_target_missing",
            "issue_specific_denominator_missing",
        ] {
            assert!(bound.findings.iter().any(|finding| finding.code == code));
        }

        fs::create_dir_all(
            worktree
                .join(source)
                .parent()
                .expect("source parent directory"),
        )
        .expect("materialized source directory");
        fs::create_dir_all(
            worktree
                .join(test_target)
                .parent()
                .expect("test parent directory"),
        )
        .expect("materialized test directory");
        fs::write(worktree.join(source), "pub fn implemented() {}\n").expect("implemented source");
        fs::write(
            worktree.join(test_target),
            format!(
                "#[path = \"../src/distributed/{}\"]\nmod subject;\n#[test]\nfn implemented_target_runs() {{ subject::implemented(); }}\n",
                Path::new(source)
                    .file_name()
                    .and_then(|value| value.to_str())
                    .expect("source filename")
            ),
        )
        .expect("implemented focused test");
        let implemented_targets = csdlc_v2::diagnose(&Store::new(&worktree), issue);
        assert_eq!(
            implemented_targets.status,
            csdlc_v2::doctor::DoctorStatus::Pass
        );
        assert!(implemented_targets.findings.is_empty());
    }
}

#[test]
fn deferred_rust_path_harness_admission_fails_closed_for_each_missing_predicate() {
    let cases: &[(&str, DeferredRequestMutation)] = &[
        ("source-not-deliverable", |request| {
            request.initial.deliverables.remove(0);
        }),
        ("test-not-owned", |request| {
            request.initial.affected_areas.pop();
        }),
        ("test-not-deliverable", |request| {
            request.initial.deliverables.remove(1);
        }),
        ("missing-deferral", |request| {
            request.initial.validation_lanes[0].defer_reason = None;
        }),
        ("placeholder-deferral", |request| {
            request.initial.validation_lanes[0].defer_reason = Some("TBD".into());
        }),
        ("no-path-harness", |request| {
            request.initial.validation_lanes[0].defer_reason =
                Some("The future focused test will be added during execution.".into());
        }),
        ("not-fail-closed", |request| {
            request.initial.failure_policy =
                "Fail closed unless the future validator is missing.".into();
        }),
        ("zero-test-not-enforced", |request| {
            request.initial.validation_lanes[0]
                .argv
                .retain(|argument| argument != "--no-tests=fail");
        }),
        ("multiple-unroutable-sources", |request| {
            let sibling = "adl-runtime/src/distributed/unrelated.rs".to_string();
            request.initial.affected_areas.push(sibling.clone());
            request.initial.deliverables.push(sibling);
        }),
    ];

    for (offset, (name, mutate)) in cases.iter().enumerate() {
        let temp = tempfile::tempdir().expect("tempdir");
        let repo = temp.path().join("repo");
        distributed_fixture_repo(&repo);
        let issue = 6000 + offset as u64;
        let source = "adl-runtime/src/distributed/discovery.rs";
        let test_target = "adl-runtime/tests/distributed_discovery.rs";
        let mut request = distributed_deferred_request(issue, source, test_target);
        mutate(&mut request);
        create_distributed_deferred_issue(&repo, temp.path(), &request);
        let diagnosis = csdlc_v2::diagnose(&Store::new(&repo), issue);
        assert_eq!(
            diagnosis.status,
            csdlc_v2::doctor::DoctorStatus::Block,
            "case {name} unexpectedly passed: {:?}",
            diagnosis.findings
        );
        assert!(
            diagnosis.findings.iter().any(|finding| matches!(
                finding.code.as_str(),
                "owned_rust_module_unroutable"
                    | "validator_target_missing"
                    | "issue_specific_denominator_missing"
            )),
            "case {name} lacked a false-readiness finding: {:?}",
            diagnosis.findings
        );
    }
}

#[test]
fn actual_binaries_create_validate_doctor_and_bind_without_claims() {
    let temp = tempfile::tempdir().expect("tempdir");
    let repo = temp.path().join("repo");
    let worktree = temp.path().join("worktrees/issue-42");
    let conflict = temp.path().join("worktrees/conflict");
    fs::create_dir_all(repo.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(repo.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(repo.join("csdlc-v2/tests")).expect("test directory");
    fs::create_dir_all(repo.join("design")).expect("design directory");
    fs::write(
        repo.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        repo.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(repo.join("csdlc-v2/tests/gate2.rs"), "// focused fixture\n").expect("gate2 fixture");
    fs::write(
        repo.join("csdlc-v2/tests/gate4.rs"),
        "// unrelated fixture\n",
    )
    .expect("unrelated gate4 fixture");
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n").expect("design");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Create --> Bind\n",
    )
    .expect("diagram");

    git(&repo, &["init", "-b", "main"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "C-SDLC Test"]);
    git(
        &repo,
        &[
            "remote",
            "add",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "fixture"]);

    let invalid_create_request = temp.path().join("invalid-create.json");
    let mut invalid_create = request();
    invalid_create.issue = 40;
    invalid_create.design_path = "generated/invalid-design.md".into();
    invalid_create.diagram_path = "generated/invalid-diagram.mmd".into();
    invalid_create.initial.affected_areas.clear();
    fs::write(
        &invalid_create_request,
        serde_json::to_vec_pretty(&invalid_create).expect("serialize invalid create request"),
    )
    .expect("invalid create request");
    let repo_text = repo.to_string_lossy();
    let invalid_create_text = invalid_create_request.to_string_lossy();
    let invalid_created = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &invalid_create_text,
        ],
    );
    assert!(!invalid_created.status.success());
    assert!(!repo.join("generated/invalid-design.md").exists());
    assert!(!repo.join("generated/invalid-diagram.mmd").exists());

    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;

        let outside = temp.path().join("outside");
        fs::create_dir_all(&outside).expect("outside directory");
        symlink(&outside, repo.join("escape")).expect("symlinked authored parent");
        let symlink_request = temp.path().join("symlink-create.json");
        let mut symlinked = request();
        symlinked.issue = 38;
        symlinked.design_path = "escape/design.md".into();
        symlinked.diagram_path = "escape/diagram.mmd".into();
        fs::write(
            &symlink_request,
            serde_json::to_vec_pretty(&symlinked).expect("serialize symlink request"),
        )
        .expect("symlink request");
        let symlink_text = symlink_request.to_string_lossy();
        let symlink_result = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-issue"),
            &["--root", &repo_text, "create", "--request", &symlink_text],
        );
        assert!(!symlink_result.status.success());
        assert!(!outside.join("design.md").exists());
        assert!(!outside.join("diagram.mmd").exists());
    }

    let non_proving_request = temp.path().join("non-proving-create.json");
    let mut non_proving = request();
    non_proving.issue = 41;
    non_proving.design_path = "generated/non-proving-design.md".into();
    non_proving.diagram_path = "generated/non-proving-diagram.mmd".into();
    non_proving.initial.validation_lanes[0].argv = vec!["true".into()];
    fs::write(
        &non_proving_request,
        serde_json::to_vec_pretty(&non_proving).expect("serialize non-proving request"),
    )
    .expect("non-proving request");
    let non_proving_text = non_proving_request.to_string_lossy();
    let non_proving_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &non_proving_text,
        ],
    );
    assert!(!non_proving_result.status.success());
    assert!(!repo.join("generated/non-proving-design.md").exists());
    assert!(!repo.join("generated/non-proving-diagram.mmd").exists());

    #[cfg(unix)]
    {
        fs::create_dir_all(repo.join("generated")).expect("generated directory");
        fs::write(repo.join("generated/not-executable"), "exit 0\n")
            .expect("non-executable validator");
        let non_executable_request = temp.path().join("non-executable-create.json");
        let mut non_executable = request();
        non_executable.issue = 39;
        non_executable.design_path = "generated/non-executable-design.md".into();
        non_executable.diagram_path = "generated/non-executable-diagram.mmd".into();
        non_executable.initial.validation_lanes[0].argv = vec!["generated/not-executable".into()];
        fs::write(
            &non_executable_request,
            serde_json::to_vec_pretty(&non_executable)
                .expect("serialize non-executable create request"),
        )
        .expect("non-executable create request");
        let non_executable_text = non_executable_request.to_string_lossy();
        must_succeed(command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-issue"),
            &[
                "--root",
                &repo_text,
                "create",
                "--request",
                &non_executable_text,
            ],
        ));
        let non_executable_validation = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-validate"),
            &["--root", &repo_text, "issue", "--issue", "39"],
        );
        assert!(!non_executable_validation.status.success());
    }

    let create_request = temp.path().join("create.json");
    let mut create = serde_json::to_value(request()).expect("serialize create request");
    create["claim"] = serde_json::json!({"id": "ignored-legacy-create-claim"});
    fs::write(
        &create_request,
        serde_json::to_vec_pretty(&create).expect("serialize create request"),
    )
    .expect("create request");
    let create_text = create_request.to_string_lossy();
    let legacy_create = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &create_text],
    );
    assert!(!legacy_create.status.success());
    fs::write(
        &create_request,
        serde_json::to_vec_pretty(&request()).expect("serialize claim-free create request"),
    )
    .expect("claim-free create request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &create_text],
    ));

    let validated = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo_text, "issue", "--issue", "42"],
    ));
    assert!(validated.contains("\"status\":\"pass\""));
    let diagnosed = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "42"],
    ));
    assert!(diagnosed.contains("\"ready\": true"));
    let same_repository_index: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join(".csdlc/issues/42/index.json")).expect("same-repository index"),
    )
    .expect("same-repository index JSON");
    assert!(same_repository_index.get("code_repository").is_none());

    let split_worktree = temp.path().join("worktrees/issue-43");
    let mut split = request();
    split.issue = 43;
    split.repository = "danielbaustin/agent-design-language".into();
    split.design_path = "design/issue-43.md".into();
    split.diagram_path = "design/issue-43.mmd".into();
    split.initial.affected_areas[0] = "design/issue-43.md".into();
    split.initial.repo_inputs[0] = "design/issue-43.md".into();
    fs::write(repo.join("design/issue-43.md"), "# Approved split design\n").expect("split design");
    fs::write(
        repo.join("design/issue-43.mmd"),
        "flowchart LR\n  Issue --> Code\n",
    )
    .expect("split diagram");
    let split_create_request = temp.path().join("split-create.json");
    fs::write(
        &split_create_request,
        serde_json::to_vec_pretty(&split).expect("serialize split create request"),
    )
    .expect("split create request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &split_create_request.to_string_lossy(),
        ],
    ));
    let split_without_contract = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "43"],
    );
    assert!(!split_without_contract.status.success());
    assert!(String::from_utf8_lossy(&split_without_contract.stdout)
        .contains("no explicit code repository was declared"));
    let split_bind_request = temp.path().join("split-bind.json");
    fs::write(
        &split_bind_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 43,
            "base_branch": "main",
            "branch": "issue-43",
            "worktree": split_worktree,
            "code_repository": "agent-logic/agent-design-language",
        }))
        .expect("serialize split bind request"),
    )
    .expect("split bind request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &repo_text,
            "--request",
            &split_bind_request.to_string_lossy(),
        ],
    ));
    let split_repo_text = split_worktree.to_string_lossy();
    let split_diagnosis = must_succeed(command(
        &split_worktree,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &split_repo_text, "--issue", "43"],
    ));
    assert!(!split_diagnosis.contains("repository_identity_drift"));
    let split_index: serde_json::Value = serde_json::from_slice(
        &fs::read(split_worktree.join(".csdlc/issues/43/index.json")).expect("split issue index"),
    )
    .expect("split issue index JSON");
    assert_eq!(
        split_index["code_repository"],
        "agent-logic/agent-design-language"
    );
    let mismatched_split = csdlc_v2::doctor::diagnose_with_code_repository(
        &csdlc_v2::Store::new(&split_worktree),
        43,
        Some("other-owner/other-repository"),
    );
    let mismatch = mismatched_split
        .findings
        .iter()
        .find(|finding| finding.code == "repository_identity_drift")
        .expect("mismatched explicit split must fail closed");
    assert!(mismatch
        .message
        .contains("requested code repository other-owner/other-repository"));
    assert!(mismatch
        .message
        .contains("recorded code repository agent-logic/agent-design-language"));
    git(&repo, &["remote", "remove", "origin"]);
    let missing_origin = csdlc_v2::doctor::diagnose_with_code_repository(
        &csdlc_v2::Store::new(&split_worktree),
        43,
        Some("agent-logic/agent-design-language"),
    );
    assert!(missing_origin.findings.iter().any(|finding| {
        finding.code == "repository_identity_drift" && finding.message.contains("none is available")
    }));
    git(
        &repo,
        &["remote", "add", "origin", "file:///not-a-github-repository"],
    );
    let non_github_origin = csdlc_v2::doctor::diagnose_with_code_repository(
        &csdlc_v2::Store::new(&split_worktree),
        43,
        Some("agent-logic/agent-design-language"),
    );
    assert!(non_github_origin.findings.iter().any(|finding| {
        finding.code == "repository_identity_drift" && finding.message.contains("none is available")
    }));
    git(
        &repo,
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    git(
        &repo,
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/other-owner/other-repository.git",
        ],
    );
    let substituted_bind_request = temp.path().join("substituted-split-bind.json");
    fs::write(
        &substituted_bind_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 43,
            "base_branch": "main",
            "branch": "issue-43",
            "worktree": split_worktree,
            "code_repository": "other-owner/other-repository",
        }))
        .expect("serialize substituted split bind request"),
    )
    .expect("substituted split bind request");
    let substituted_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &repo_text,
            "--request",
            &substituted_bind_request.to_string_lossy(),
        ],
    );
    assert!(!substituted_bind.status.success());
    let post_substitution = csdlc_v2::diagnose(&csdlc_v2::Store::new(&split_worktree), 43);
    assert!(post_substitution.findings.iter().any(|finding| {
        finding.code == "repository_identity_drift"
            && finding
                .message
                .contains("declared code repository agent-logic/agent-design-language")
            && finding
                .message
                .contains("origin repository other-owner/other-repository")
    }));
    git(
        &repo,
        &[
            "remote",
            "set-url",
            "origin",
            "https://github.com/agent-logic/agent-design-language.git",
        ],
    );
    let restored_split = csdlc_v2::diagnose(&csdlc_v2::Store::new(&split_worktree), 43);
    assert!(!restored_split
        .findings
        .iter()
        .any(|finding| finding.code == "repository_identity_drift"));

    let mut issue_5795_shape = request();
    issue_5795_shape.issue = 44;
    issue_5795_shape.repository = "danielbaustin/agent-design-language".into();
    issue_5795_shape.design_path = "design/issue-44.md".into();
    issue_5795_shape.diagram_path = "design/issue-44.mmd".into();
    issue_5795_shape.initial.affected_areas[0] = "design/issue-44.md".into();
    issue_5795_shape.initial.repo_inputs[0] = "design/issue-44.md".into();
    fs::write(repo.join("design/issue-44.md"), "# Approved design\n")
        .expect("issue 5795 shaped design");
    fs::write(
        repo.join("design/issue-44.mmd"),
        "flowchart LR\n  Plan --> Block\n",
    )
    .expect("issue 5795 shaped diagram");
    let issue_5795_request = temp.path().join("issue-5795-shape-create.json");
    fs::write(
        &issue_5795_request,
        serde_json::to_vec_pretty(&issue_5795_shape).expect("serialize issue 5795 shaped request"),
    )
    .expect("issue 5795 shaped request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &issue_5795_request.to_string_lossy(),
        ],
    ));
    fs::create_dir_all(repo.join("adl-runtime-kernel/src")).expect("runtime-kernel source");
    fs::create_dir_all(repo.join("adl-runtime-kernel/tests")).expect("future crate test directory");
    fs::write(
        repo.join("adl-runtime-kernel/Cargo.toml"),
        "[package]\nname = \"adl-runtime-kernel\"\nversion = \"0.1.0\"\n",
    )
    .expect("runtime-kernel manifest");
    let future_module = "adl-runtime-kernel/src/shepherd.rs";
    let present_validator = "adl-runtime-kernel/tests/shepherd_unit.rs";
    let missing_validator = "adl-runtime-kernel/tests/shepherd_live.rs";
    fs::write(repo.join(present_validator), "// present but unselected\n")
        .expect("present issue-owned validator");
    apply_edit(
        &repo,
        temp.path(),
        44,
        "spp",
        "unroutable-owned-module",
        serde_json::json!({
            "operation": "replace_planning_collection",
            "field": "affected_areas",
            "values": [future_module, present_validator, missing_validator]
        }),
    );
    apply_edit(
        &repo,
        temp.path(),
        44,
        "stp",
        "missing-validator-deliverable",
        serde_json::json!({
            "operation": "replace_planning_collection",
            "field": "deliverables",
            "values": [future_module, present_validator, missing_validator]
        }),
    );
    apply_edit(
        &repo,
        temp.path(),
        44,
        "vpp",
        "zero-denominator-lane",
        serde_json::json!({
            "operation": "replace_validation_lanes",
            "lanes": [{
                "lane": "unrelated-existing-gate",
                "proof_role": "reproduce the unrelated existing test denominator",
                "acceptance_ids": ["AC-1", "AC-2"],
                "deterministic": true,
                "resource_profile": "small",
                "budget_seconds": 120,
                "budget_tokens": 1000,
                "argv": ["cargo", "test", "--manifest-path", "csdlc-v2/Cargo.toml", "--test", "gate4"],
                "parallel_group": "local",
                "defer_reason": null
            }]
        }),
    );
    let issue_5795_diagnosis = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "44"],
    );
    assert!(!issue_5795_diagnosis.status.success());
    let issue_5795_diagnosis =
        String::from_utf8(issue_5795_diagnosis.stdout).expect("UTF-8 diagnosis");
    for code in [
        "repository_identity_drift",
        "owned_rust_module_unroutable",
        "validator_target_missing",
        "issue_specific_denominator_missing",
    ] {
        assert!(issue_5795_diagnosis.contains(code), "missing {code}");
    }

    fs::create_dir_all(repo.join("workspace/member/src/foo")).expect("nested product source");
    fs::create_dir_all(repo.join("workspace/member/tests")).expect("product tests");
    fs::create_dir_all(repo.join("docs/src")).expect("non-crate src directory");
    fs::write(
        repo.join("Cargo.toml"),
        "[workspace]\nmembers = [\"workspace/member\"]\nresolver = \"2\"\n",
    )
    .expect("workspace manifest");
    fs::write(
        repo.join("workspace/member/Cargo.toml"),
        "[package]\nname = \"product\"\nversion = \"0.1.0\"\n",
    )
    .expect("product manifest");
    fs::write(repo.join("workspace/member/src/lib.rs"), "pub mod foo;\n")
        .expect("product crate root");
    fs::write(
        repo.join("workspace/member/src/foo/mod.rs"),
        "// parent module\n",
    )
    .expect("parent module");
    fs::write(
        repo.join("workspace/member/tests/focused.rs"),
        "// focused validator\n",
    )
    .expect("focused validator");
    let mut valid_edge_shapes = request();
    valid_edge_shapes.issue = 45;
    valid_edge_shapes.design_path = "design/issue-45.md".into();
    valid_edge_shapes.diagram_path = "design/issue-45.mmd".into();
    valid_edge_shapes.initial.repo_inputs[0] = "design/issue-45.md".into();
    valid_edge_shapes.initial.affected_areas = vec![
        "design/issue-45.md".into(),
        "workspace/member/src/foo/mod.rs".into(),
        "workspace/member/src/foo/bar/mod.rs".into(),
        "workspace/member/src/server.py".into(),
        "docs/src/example.rs".into(),
        "workspace/member/tests/focused.rs".into(),
    ];
    valid_edge_shapes.initial.deliverables = vec![
        "workspace/member/src/foo/bar/mod.rs".into(),
        "workspace/member/src/server.py".into(),
        "docs/src/example.rs".into(),
        "workspace/member/tests/focused.rs".into(),
    ];
    valid_edge_shapes.initial.validation_lanes[0].argv = vec![
        "cargo".into(),
        "test".into(),
        "--manifest-path=workspace/member/Cargo.toml".into(),
        "--test=focused".into(),
    ];
    fs::write(repo.join("design/issue-45.md"), "# Approved design\n").expect("edge-shape design");
    fs::write(
        repo.join("design/issue-45.mmd"),
        "flowchart LR\n  Plan --> Ready\n",
    )
    .expect("edge-shape diagram");
    let valid_edge_request = temp.path().join("valid-edge-shapes-create.json");
    fs::write(
        &valid_edge_request,
        serde_json::to_vec_pretty(&valid_edge_shapes).expect("serialize valid edge request"),
    )
    .expect("valid edge request");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &[
            "--root",
            &repo_text,
            "create",
            "--request",
            &valid_edge_request.to_string_lossy(),
        ],
    ));
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", ".", "--issue", "45"],
    ));
    apply_edit(
        &repo,
        temp.path(),
        45,
        "vpp",
        "package-selected-validator",
        serde_json::json!({
            "operation": "replace_validation_lanes",
            "lanes": [{
                "lane": "package-selected-focused",
                "proof_role": "exercise the issue-owned package validator",
                "acceptance_ids": ["AC-1", "AC-2"],
                "deterministic": true,
                "resource_profile": "small",
                "budget_seconds": 120,
                "budget_tokens": 1000,
                "argv": ["cargo", "test", "-p", "product", "--test", "focused"],
                "parallel_group": "local",
                "defer_reason": null
            }]
        }),
    );
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", ".", "--issue", "45"],
    ));

    fs::create_dir_all(repo.join("validators")).expect("validator directory");
    for (issue, failure_policy, should_pass) in [
        (
            46_u64,
            "Do not under any circumstances ever fail closed when proof is absent.",
            false,
        ),
        (47_u64, "Fail closed when proof is absent.", true),
        (
            48_u64,
            "Fail closed unless the validator is missing.",
            false,
        ),
    ] {
        let design = format!("design/issue-{issue}.md");
        let diagram = format!("design/issue-{issue}.mmd");
        fs::write(repo.join(&design), "# Approved design\n").expect("deferred design");
        fs::write(repo.join(&diagram), "flowchart LR\n  Plan --> Deferred\n")
            .expect("deferred diagram");
        let mut deferred = request();
        deferred.issue = issue;
        deferred.design_path = design.clone();
        deferred.diagram_path = diagram;
        deferred.initial.repo_inputs[0] = design.clone();
        deferred.initial.affected_areas = vec![design, "validators/future-check.sh".into()];
        deferred.initial.deliverables = vec!["validators/future-check.sh".into()];
        deferred.initial.failure_policy = failure_policy.into();
        deferred.initial.validation_lanes[0].argv =
            vec!["bash".into(), "validators/future-check.sh".into()];
        deferred.initial.validation_lanes[0].defer_reason =
            Some("validator is an explicit execution deliverable".into());
        let deferred_request = temp.path().join(format!("deferred-{issue}-create.json"));
        fs::write(
            &deferred_request,
            serde_json::to_vec_pretty(&deferred).expect("serialize deferred request"),
        )
        .expect("deferred request");
        must_succeed(command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-issue"),
            &[
                "--root",
                &repo_text,
                "create",
                "--request",
                &deferred_request.to_string_lossy(),
            ],
        ));
        let diagnosis = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-doctor"),
            &["--repo", &repo_text, "--issue", &issue.to_string()],
        );
        assert_eq!(diagnosis.status.success(), should_pass);
        if !should_pass {
            let diagnosis = String::from_utf8(diagnosis.stdout).expect("UTF-8 diagnosis");
            assert!(diagnosis.contains("validator_target_missing"));
            assert!(diagnosis.contains("issue_specific_denominator_missing"));
        }
    }

    let index_path = repo.join(".csdlc/issues/42/index.json");
    let current_index = || -> serde_json::Value {
        serde_json::from_slice(&fs::read(&index_path).expect("issue index"))
            .expect("issue index JSON")
    };
    let planning_edit_path = temp.path().join("initialized-planning-edit.json");
    let index = current_index();
    fs::write(
        &planning_edit_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove initialized planning repair",
            "operation": {
                "operation": "replace_planning_collection",
                "field": "affected_areas",
                "values": ["design/future/deep.rs", "csdlc-v2/tests/gate2.rs"]
            }
        }))
        .expect("serialize initialized planning edit"),
    )
    .expect("initialized planning edit");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &planning_edit_path.to_string_lossy(),
        ],
    ));
    let stale_edit = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &planning_edit_path.to_string_lossy(),
        ],
    );
    assert!(!stale_edit.status.success());

    let non_planning_path = temp.path().join("initialized-non-planning-edit.json");
    let index = current_index();
    fs::write(
        &non_planning_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove non-planning edit remains blocked",
            "operation": {
                "operation": "replace_plan_steps",
                "steps": [{
                    "id": "step-1",
                    "action": "must remain blocked before binding",
                    "acceptance_ids": ["AC-1", "AC-2"],
                    "status": "pending"
                }]
            }
        }))
        .expect("serialize non-planning edit"),
    )
    .expect("non-planning edit");
    let non_planning = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &non_planning_path.to_string_lossy(),
        ],
    );
    assert!(!non_planning.status.success());

    let ready_path = temp.path().join("advance-ready.json");
    let index = current_index();
    fs::write(
        &ready_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove ready planning repair",
            "operation": {"operation": "advance_phase", "phase": "ready"}
        }))
        .expect("serialize ready transition"),
    )
    .expect("ready transition");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &ready_path.to_string_lossy(),
        ],
    ));

    let ready_edit_path = temp.path().join("ready-planning-edit.json");
    let index = current_index();
    fs::write(
        &ready_edit_path,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "card": "spp",
            "expected_generation": index["generation"],
            "expected_digest": index["digest"],
            "actor": "test-operator",
            "reason": "prove ready planning repair",
            "operation": {
                "operation": "replace_planning_collection",
                "field": "invariants",
                "values": ["Git topology remains binding authority"]
            }
        }))
        .expect("serialize ready planning edit"),
    )
    .expect("ready planning edit");
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo_text,
            "apply",
            "--request",
            &ready_edit_path.to_string_lossy(),
        ],
    ));
    let ready_diagnosis = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-doctor"),
        &["--repo", &repo_text, "--issue", "42"],
    ));
    assert!(ready_diagnosis.contains("\"status\": \"pass\""));
    assert!(ready_diagnosis.contains("\"phase\": \"ready\""));

    let duplicate_request = temp.path().join("duplicate-create.json");
    let mut duplicate = request();
    duplicate.design_path = "generated/duplicate-design.md".into();
    duplicate.diagram_path = "generated/duplicate-diagram.mmd".into();
    fs::write(
        &duplicate_request,
        serde_json::to_vec_pretty(&duplicate).expect("serialize duplicate request"),
    )
    .expect("duplicate request");
    let duplicate_text = duplicate_request.to_string_lossy();
    let duplicate_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-issue"),
        &["--root", &repo_text, "create", "--request", &duplicate_text],
    );
    assert!(!duplicate_result.status.success());
    assert!(!repo.join("generated/duplicate-design.md").exists());
    assert!(!repo.join("generated/duplicate-diagram.mmd").exists());

    let bind_request = temp.path().join("bind.json");
    let bind = serde_json::json!({
        "issue": 42,
        "base_branch": "main",
        "branch": "issue-42",
        "worktree": worktree,
        "claim": {"id": "ignored-legacy-bind-claim"},
    });
    fs::write(
        &bind_request,
        serde_json::to_vec_pretty(&bind).expect("serialize bind request"),
    )
    .expect("bind request");
    let bind_text = bind_request.to_string_lossy();

    let legacy_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!legacy_bind.status.success());
    fs::write(
        &bind_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "issue-42",
            "worktree": worktree,
        }))
        .expect("serialize claim-free bind request"),
    )
    .expect("claim-free bind request");

    fs::write(repo.join("design/issue-42.md"), "# Stale design\n").expect("stale design");
    let invalid_validation = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo_text, "issue", "--issue", "42"],
    );
    assert!(!invalid_validation.status.success());
    let invalid_bind = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!invalid_bind.status.success());
    assert!(!worktree.exists());
    assert!(!git(&repo, &["branch", "--list", "issue-42"]).contains("issue-42"));
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n").expect("restore design");

    let existing = temp.path().join("worktrees/existing");
    git(
        &repo,
        &[
            "worktree",
            "add",
            "-b",
            "issue-42-existing",
            &existing.to_string_lossy(),
            "main",
        ],
    );
    fs::create_dir_all(existing.join(".csdlc/locks/42.lock")).expect("invalid target lock");
    let existing_request = temp.path().join("existing-bind.json");
    fs::write(
        &existing_request,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "issue-42-existing",
            "worktree": existing,
        }))
        .expect("serialize existing bind request"),
    )
    .expect("existing bind request");
    let existing_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &repo_text,
            "--request",
            &existing_request.to_string_lossy(),
        ],
    );
    assert!(!existing_result.status.success());
    assert!(!existing.join(".csdlc/issues/42").exists());
    git(
        &repo,
        &["worktree", "remove", "--force", &existing.to_string_lossy()],
    );
    git(&repo, &["branch", "-D", "issue-42-existing"]);

    let first = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    ));
    assert!(first.contains("\"created\":true"));
    let topology = git(&repo, &["worktree", "list", "--porcelain"]);
    assert!(topology.contains(&format!(
        "worktree {}",
        worktree.canonicalize().unwrap().display()
    )));
    assert!(topology.contains("branch refs/heads/issue-42"));

    let second = must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    ));
    assert!(second.contains("\"created\":false"));

    let index = worktree.join(".csdlc/issues/42/index.json");
    let original_index = fs::read(&index).expect("bound index");
    let mut unsigned: serde_json::Value =
        serde_json::from_slice(&original_index).expect("index JSON");
    unsigned["unsigned_topology"] = serde_json::json!("must fail");
    fs::write(
        &index,
        serde_json::to_vec_pretty(&unsigned).expect("serialize unsigned index"),
    )
    .expect("unsigned index");
    let unsigned_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!unsigned_result.status.success());
    fs::write(&index, &original_index).expect("restore bound index");

    let audit = worktree.join(".csdlc/issues/42/audit.jsonl");
    let original_audit = fs::read(&audit).expect("bound audit");
    let mut forged_audit = original_audit.clone();
    forged_audit.extend_from_slice(b"{}\n");
    fs::write(&audit, forged_audit).expect("forged audit");
    let audit_result = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!audit_result.status.success());
    fs::write(&audit, original_audit).expect("restore bound audit");

    let mut contradictory: serde_json::Value =
        serde_json::from_slice(&fs::read(&index).expect("bound index")).expect("index JSON");
    contradictory["branch"] = serde_json::json!("different-branch");
    fs::write(
        &index,
        serde_json::to_vec_pretty(&contradictory).expect("serialize contradictory index"),
    )
    .expect("contradictory index");
    let inconsistent = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &bind_text],
    );
    assert!(!inconsistent.status.success());

    let conflict_request = temp.path().join("conflict.json");
    let conflict_bind = serde_json::json!({
        "issue": 42,
        "base_branch": "main",
        "branch": "issue-42-conflict",
        "worktree": conflict,
    });
    fs::write(
        &conflict_request,
        serde_json::to_vec_pretty(&conflict_bind).expect("serialize conflict request"),
    )
    .expect("conflict request");
    let conflict_text = conflict_request.to_string_lossy();
    let rejected = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &["--root", &repo_text, "--request", &conflict_text],
    );
    assert!(!rejected.status.success());
    let rejection = String::from_utf8_lossy(&rejected.stdout);
    assert!(rejection.contains("reconciliation_required") || rejection.contains("corrupt_record"));

    git(
        &repo,
        &["worktree", "remove", "--force", &worktree.to_string_lossy()],
    );
}

#[test]
fn implemented_sip_scope_correction() {
    let target = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("target");
    fs::create_dir_all(&target).expect("target temp parent");
    let temp = tempfile::Builder::new()
        .prefix("issue-63-scope-")
        .tempdir_in(&target)
        .expect("repo-local tempdir");
    let repo = temp.path().join("repo");
    fs::create_dir_all(repo.join("docs/templates/prompts")).expect("registry directory");
    fs::create_dir_all(repo.join("csdlc-v2/operator")).expect("manifest directory");
    fs::create_dir_all(repo.join("csdlc-v2/tests")).expect("test directory");
    fs::create_dir_all(repo.join("design")).expect("design directory");
    fs::write(
        repo.join("docs/templates/prompts/current.json"),
        include_bytes!("../../docs/templates/prompts/current.json"),
    )
    .expect("registry fixture");
    fs::write(
        repo.join("csdlc-v2/operator/native-card-shape.json"),
        include_bytes!("../operator/native-card-shape.json"),
    )
    .expect("shape fixture");
    fs::write(repo.join("csdlc-v2/tests/gate2.rs"), "// focused fixture\n").expect("gate2 fixture");
    fs::write(repo.join("design/issue-42.md"), "# Approved design\n").expect("design");
    fs::write(
        repo.join("design/issue-42.mmd"),
        "flowchart LR\n  Edit --> Validate\n",
    )
    .expect("diagram");
    git(&repo, &["init", "-b", "main"]);
    git(&repo, &["config", "user.email", "test@example.com"]);
    git(&repo, &["config", "user.name", "C-SDLC Test"]);
    git(&repo, &["add", "."]);
    git(&repo, &["commit", "-m", "fixture"]);

    let store = Store::new(&repo);
    let record = csdlc_v2::initialize_native_json(
        &store,
        &serde_json::to_vec(&request()).expect("serialize bootstrap"),
    )
    .expect("bootstrap");
    let _ready = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: record.generation,
            expected_digest: record.digest,
            actor: "test-operator".into(),
            reason: "fixture ready".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Ready,
            },
            fail_after_backup: false,
        },
    )
    .expect("ready");
    git(&repo, &["switch", "-c", "issue-42"]);
    bind_issue(
        &store,
        BindRequest {
            issue: 42,
            base_branch: "main".into(),
            branch: "issue-42".into(),
            worktree: ".".into(),
            code_repository: None,
        },
    )
    .expect("bind");
    let bound = store.load_record(42).expect("bound record");
    assert_eq!(bound.phase, LifecyclePhase::Bound);

    let request_path = |name: &str| temp.path().join(format!("{name}.json"));
    let write_edit = |path: &Path,
                      index: &serde_json::Value,
                      card: &str,
                      actor: &str,
                      reason: &str,
                      values: Vec<&str>| {
        fs::write(
            path,
            serde_json::to_vec_pretty(&serde_json::json!({
                "issue": 42,
                "card": card,
                "expected_generation": index["generation"],
                "expected_digest": index["digest"],
                "actor": actor,
                "reason": reason,
                "operation": {
                    "operation": "correct_declared_scope_before_publication",
                    "values": values
                }
            }))
            .expect("serialize correction request"),
        )
        .expect("write correction request");
    };
    let current_index = || -> serde_json::Value {
        serde_json::from_slice(
            &fs::read(repo.join(".csdlc/issues/42/index.json")).expect("issue index"),
        )
        .expect("issue index JSON")
    };
    let bound_request = request_path("bound-correction");
    write_edit(
        &bound_request,
        &current_index(),
        "sip",
        "test-operator",
        "bound is too early",
        vec!["src/new.rs"],
    );
    let bound_rejection = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &bound_request.to_string_lossy(),
        ],
    );
    assert!(!bound_rejection.status.success());

    let executed = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sor,
            expected_generation: bound.generation,
            expected_digest: bound.digest,
            actor: "test-operator".into(),
            reason: "record fixture implementation".into(),
            operation: SemanticOperation::RecordExecution {
                summary: "implemented fixture".into(),
                changes: vec!["src/old.rs".into()],
                artifacts: vec!["fixture".into()],
            },
            fail_after_backup: false,
        },
    )
    .expect("execution");
    let implemented = edit_issue(
        &store,
        EditRequest {
            issue: 42,
            card: CardKind::Sip,
            expected_generation: executed.generation,
            expected_digest: executed.digest,
            actor: "test-operator".into(),
            reason: "advance implemented fixture".into(),
            operation: SemanticOperation::AdvancePhase {
                phase: LifecyclePhase::Implemented,
            },
            fail_after_backup: false,
        },
    )
    .expect("implemented");

    for (name, card, actor, reason, values) in [
        ("empty-actor", "sip", "", "reason", vec!["src/new.rs"]),
        ("empty-reason", "sip", "operator", "", vec!["src/new.rs"]),
        ("empty-scope", "sip", "operator", "reason", vec![]),
        (
            "wrong-card",
            "stp",
            "operator",
            "reason",
            vec!["src/new.rs"],
        ),
    ] {
        let path = request_path(name);
        write_edit(&path, &current_index(), card, actor, reason, values);
        let output = command(
            &repo,
            env!("CARGO_BIN_EXE_csdlc-edit"),
            &[
                "--repo",
                &repo.to_string_lossy(),
                "apply",
                "--request",
                &path.to_string_lossy(),
            ],
        );
        assert!(!output.status.success(), "{name} unexpectedly succeeded");
        assert_eq!(store.load_record(42).unwrap().digest, implemented.digest);
    }

    let stale_digest_path = request_path("stale-digest");
    let mut stale_index = current_index();
    stale_index["digest"] = serde_json::Value::String("0".repeat(64));
    write_edit(
        &stale_digest_path,
        &stale_index,
        "sip",
        "operator",
        "stale digest",
        vec!["src/new.rs"],
    );
    assert!(!command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &stale_digest_path.to_string_lossy(),
        ],
    )
    .status
    .success());

    let correction_path = request_path("correction");
    write_edit(
        &correction_path,
        &current_index(),
        "sip",
        "review-fix-operator",
        "replace stale declared scope path",
        vec!["src/new.rs", "tests/new.rs"],
    );
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &correction_path.to_string_lossy(),
        ],
    ));
    let corrected = store.load_record(42).expect("corrected record");
    assert_eq!(corrected.generation, implemented.generation + 1);
    assert_ne!(corrected.digest, implemented.digest);
    let operation: serde_json::Value =
        serde_json::from_str(&corrected.audit.last().expect("correction audit").operation)
            .expect("audit operation JSON");
    assert_eq!(
        operation["previous_values"],
        serde_json::json!(["claim-free workflow"])
    );
    assert_eq!(
        operation["new_values"],
        serde_json::json!(["src/new.rs", "tests/new.rs"])
    );
    let event = corrected.audit.last().expect("correction audit");
    assert_eq!(event.actor, "review-fix-operator");
    assert_eq!(event.reason, "replace stale declared scope path");
    let sip_values: serde_json::Value = serde_json::from_slice(
        &fs::read(repo.join(".csdlc/issues/42/cards/sip.values.json")).expect("SIP values"),
    )
    .expect("SIP values JSON");
    assert_eq!(
        sip_values["content"]["values"]["declared_scope"],
        serde_json::json!(["src/new.rs", "tests/new.rs"])
    );
    let sip_path = repo.join(".csdlc/issues/42/cards/sip.md");
    let rendered = fs::read_to_string(&sip_path).expect("rendered SIP");
    assert!(rendered.contains("src/new.rs"));
    assert!(rendered.contains("tests/new.rs"));
    must_succeed(command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
    ));

    let stale_generation = command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-edit"),
        &[
            "--repo",
            &repo.to_string_lossy(),
            "apply",
            "--request",
            &correction_path.to_string_lossy(),
        ],
    );
    assert!(!stale_generation.status.success());
    let before_drift = fs::read(&sip_path).expect("SIP before drift");
    fs::write(
        &sip_path,
        [before_drift.as_slice(), b"\nmanual drift\n"].concat(),
    )
    .expect("inject Markdown drift");
    assert!(!command(
        &repo,
        env!("CARGO_BIN_EXE_csdlc-validate"),
        &["--root", &repo.to_string_lossy(), "issue", "--issue", "42"],
    )
    .status
    .success());
    fs::write(sip_path, before_drift).expect("restore rendered SIP");
}

#[test]
fn bind_topology_scan_uses_canonical_record_identity() {
    let temp = tempfile::tempdir().expect("tempdir");

    // Reproduce the reported failure through the real bind binary: an unrelated
    // retained record stores `worktree: "."` and references intentionally absent
    // local artifacts in the issue worktree being bound.
    let success_repo = temp.path().join("success-repo");
    let success_worktree = temp.path().join("success-worktree");
    focused_fixture_repo(&success_repo);
    git(
        &success_repo,
        &[
            "worktree",
            "add",
            "-b",
            "candidate-42",
            &success_worktree.to_string_lossy(),
            "main",
        ],
    );
    create_focused_issue(&success_worktree, temp.path(), 42);
    let retained_5791 = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root")
        .join(".csdlc/issues/5791");
    copy_directory(&retained_5791, &success_worktree.join(".csdlc/issues/5791"));
    assert!(!success_worktree
        .join(".adl/local-artifacts/5791-bootstrap/design.md")
        .exists());
    let success_bind = temp.path().join("success-bind.json");
    fs::write(
        &success_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "candidate-42",
            "worktree": success_worktree,
        }))
        .expect("serialize success bind"),
    )
    .expect("success bind request");
    let success = must_succeed(command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    ));
    let success: serde_json::Value = serde_json::from_str(&success).expect("bind result JSON");
    assert_eq!(success["branch"], "candidate-42");
    assert_eq!(
        success["worktree"],
        success_worktree
            .canonicalize()
            .expect("canonical worktree")
            .to_string_lossy()
            .as_ref()
    );

    // A genuinely relevant same-issue record is still fully verified.
    fs::remove_file(success_worktree.join("design/issue-42.md")).expect("remove issue design");
    let corrupt_same_issue = command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    );
    assert!(!corrupt_same_issue.status.success());
    fs::write(
        success_worktree.join("design/issue-42.md"),
        "# Approved design for issue 42\n",
    )
    .expect("restore issue design");

    // An exact projection must not short-circuit a later conflicting copy of
    // the same issue. Build that conflicting projection through the real bind
    // binary, then place it in a worktree listed after the exact projection.
    let conflicting_source_repo = temp.path().join("conflicting-source-repo");
    let conflicting_source_worktree = temp.path().join("conflicting-source-worktree");
    focused_fixture_repo(&conflicting_source_repo);
    git(
        &conflicting_source_repo,
        &[
            "worktree",
            "add",
            "-b",
            "other-42",
            &conflicting_source_worktree.to_string_lossy(),
            "main",
        ],
    );
    create_focused_issue(&conflicting_source_worktree, temp.path(), 42);
    let conflicting_source_bind = temp.path().join("conflicting-source-bind.json");
    fs::write(
        &conflicting_source_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "other-42",
            "worktree": conflicting_source_worktree,
        }))
        .expect("serialize conflicting source bind"),
    )
    .expect("conflicting source bind request");
    must_succeed(command(
        &conflicting_source_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflicting_source_worktree.to_string_lossy(),
            "--request",
            &conflicting_source_bind.to_string_lossy(),
        ],
    ));
    let later_projection = temp.path().join("later-projection");
    git(
        &success_repo,
        &[
            "worktree",
            "add",
            "-b",
            "later-projection",
            &later_projection.to_string_lossy(),
            "main",
        ],
    );
    copy_directory(
        &conflicting_source_worktree.join(".csdlc/issues/42"),
        &later_projection.join(".csdlc/issues/42"),
    );
    let later_conflict = command(
        &success_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &success_worktree.to_string_lossy(),
            "--request",
            &success_bind.to_string_lossy(),
        ],
    );
    assert!(!later_conflict.status.success());
    assert!(String::from_utf8_lossy(&later_conflict.stdout).contains("reconciliation_required"));

    // A retained exact record is not sufficient for idempotence when Git no
    // longer registers the stored branch/worktree pair.
    let stale_repo = temp.path().join("stale-repo");
    let stale_worktree = temp.path().join("stale-worktree");
    focused_fixture_repo(&stale_repo);
    git(
        &stale_repo,
        &[
            "worktree",
            "add",
            "-b",
            "stale-42",
            &stale_worktree.to_string_lossy(),
            "main",
        ],
    );
    create_focused_issue(&stale_worktree, temp.path(), 42);
    let stale_bind = temp.path().join("stale-bind.json");
    fs::write(
        &stale_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "stale-42",
            "worktree": stale_worktree,
        }))
        .expect("serialize stale bind"),
    )
    .expect("stale bind request");
    must_succeed(command(
        &stale_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &stale_worktree.to_string_lossy(),
            "--request",
            &stale_bind.to_string_lossy(),
        ],
    ));
    copy_directory(
        &stale_worktree.join(".csdlc/issues/42"),
        &stale_repo.join(".csdlc/issues/42"),
    );
    fs::create_dir_all(stale_repo.join("design")).expect("stale design directory");
    fs::copy(
        stale_worktree.join("design/issue-42.md"),
        stale_repo.join("design/issue-42.md"),
    )
    .expect("retain stale design");
    git(
        &stale_repo,
        &[
            "worktree",
            "remove",
            "--force",
            &stale_worktree.to_string_lossy(),
        ],
    );
    let stale_result = command(
        &stale_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &stale_repo.to_string_lossy(),
            "--request",
            &stale_bind.to_string_lossy(),
        ],
    );
    assert!(!stale_result.status.success());
    assert!(String::from_utf8_lossy(&stale_result.stdout).contains("reconciliation_required"));

    // Build one valid owner record, retain its projection in an unrelated
    // primary checkout, and unregister its original worktree. Collision
    // decisions must use the stored branch/worktree predicates, not the branch
    // or path of the projection being scanned.
    let conflict_repo = temp.path().join("conflict-repo");
    let owner_worktree = temp.path().join("owner-worktree");
    focused_fixture_repo(&conflict_repo);
    git(
        &conflict_repo,
        &[
            "worktree",
            "add",
            "-b",
            "claimed-branch",
            &owner_worktree.to_string_lossy(),
            "main",
        ],
    );
    create_focused_issue(&owner_worktree, temp.path(), 43);
    let owner_bind = temp.path().join("owner-bind.json");
    fs::write(
        &owner_bind,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 43,
            "base_branch": "main",
            "branch": "claimed-branch",
            "worktree": owner_worktree,
        }))
        .expect("serialize owner bind"),
    )
    .expect("owner bind request");
    must_succeed(command(
        &owner_worktree,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &owner_worktree.to_string_lossy(),
            "--request",
            &owner_bind.to_string_lossy(),
        ],
    ));
    copy_directory(
        &owner_worktree.join(".csdlc/issues/43"),
        &conflict_repo.join(".csdlc/issues/43"),
    );
    git(
        &conflict_repo,
        &[
            "worktree",
            "remove",
            "--force",
            &owner_worktree.to_string_lossy(),
        ],
    );
    create_focused_issue(&conflict_repo, temp.path(), 42);

    let branch_collision = temp.path().join("branch-collision.json");
    fs::write(
        &branch_collision,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "claimed-branch",
            "worktree": temp.path().join("branch-candidate"),
        }))
        .expect("serialize branch collision"),
    )
    .expect("branch collision request");
    fs::remove_file(conflict_repo.join("design/issue-43.md")).expect("remove owner design");
    let contextual_rejection = command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &branch_collision.to_string_lossy(),
        ],
    );
    assert!(!contextual_rejection.status.success());
    let contextual_output = format!(
        "{}{}",
        String::from_utf8_lossy(&contextual_rejection.stdout),
        String::from_utf8_lossy(&contextual_rejection.stderr)
    );
    assert!(contextual_output.contains("issue 43 topology"));
    assert!(contextual_output.contains("design/issue-43.md"));
    fs::write(
        conflict_repo.join("design/issue-43.md"),
        "# Approved design for issue 43\n",
    )
    .expect("restore owner design");
    let branch_rejection = command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &branch_collision.to_string_lossy(),
        ],
    );
    assert!(!branch_rejection.status.success());
    assert!(String::from_utf8_lossy(&branch_rejection.stdout).contains("reconciliation_required"));

    let worktree_collision = temp.path().join("worktree-collision.json");
    fs::write(
        &worktree_collision,
        serde_json::to_vec_pretty(&serde_json::json!({
            "issue": 42,
            "base_branch": "main",
            "branch": "different-candidate-branch",
            "worktree": owner_worktree,
        }))
        .expect("serialize worktree collision"),
    )
    .expect("worktree collision request");
    let worktree_rejection = command(
        &conflict_repo,
        env!("CARGO_BIN_EXE_csdlc-bind"),
        &[
            "--root",
            &conflict_repo.to_string_lossy(),
            "--request",
            &worktree_collision.to_string_lossy(),
        ],
    );
    assert!(!worktree_rejection.status.success());
    assert!(String::from_utf8_lossy(&worktree_rejection.stdout).contains("reconciliation_required"));
}
