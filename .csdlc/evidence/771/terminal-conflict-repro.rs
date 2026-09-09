// Append this focused regression to the exact-source terminal_cleanup_cutover_commands.rs test harness.
// PVF: deterministic local Git fixture, small CPU/disk, required terminal conflict regression.
#[test]
fn review771_conflicting_terminal_receipt_must_preserve_state() {
    let root = std::env::temp_dir().join(format!("review771-terminal-conflict-{}", std::process::id()));
    init_repo(&root);
    write_generation_selector(&root, "v3");
    let head = git_stdout(&root, &["rev-parse", "HEAD"]);
    let mut request = base_request();
    request.expected_head_sha = Some(head.clone());
    request.credential_names = vec!["GITHUB_TOKEN".into()];
    request.terminal_state = Some(TerminalStateWriteRequest {
        repository_root: root.clone(),
        state_path: PathBuf::from(".csdlc/v3/issues/630/terminal.json"),
        receipt_path: PathBuf::from(".csdlc/evidence/630/terminal-receipt.json"),
        expected_state_digest: None,
    });
    let mut adapter = FakeGithubAdapter::new([github_pr_json(641, &head, true, "Closes #630"), github_issue_json(630, "closed")]);
    let first = prepare_terminal_finish_with_github_observation(&request, &mut adapter).unwrap();
    assert_eq!(first.status, TerminalRouteStatus::Ready);
    let state_path = root.join(".csdlc/v3/issues/630/terminal.json");
    let receipt_path = root.join(".csdlc/evidence/630/terminal-receipt.json");
    let before_state = fs::read(&state_path).unwrap();
    let before_receipt = fs::read(&receipt_path).unwrap();
    request.terminal_state.as_mut().unwrap().expected_state_digest = Some(blake3::hash(&before_state).to_hex().to_string());
    // Model a second authenticated merged PR for the same closed issue.
    request.pull_request = Some(642);
    let mut adapter = FakeGithubAdapter::new([github_pr_json(642, &head, true, "Closes #630"), github_issue_json(630, "closed")]);
    let second = prepare_terminal_finish_with_github_observation(&request, &mut adapter).unwrap();
    assert_eq!(second.status, TerminalRouteStatus::Blocked);
    assert!(second.findings.iter().any(|f| f.code == "terminal_receipt_conflict"));
    assert_eq!(fs::read(&receipt_path).unwrap(), before_receipt);
    let after: serde_json::Value = serde_json::from_slice(&fs::read(&state_path).unwrap()).unwrap();
    eprintln!("blocked conflict leaves terminal state PR={}, retained receipt PR=641; fixture={}", after["pull_request"], root.display());
    assert_eq!(fs::read(&state_path).unwrap(), before_state, "blocked finish must preserve state preimage");
}
