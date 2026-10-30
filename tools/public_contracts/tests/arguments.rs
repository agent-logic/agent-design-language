//! PVF local contract: real parsers, literal argv, exact diagnostics and status.
use std::process::Command;
fn run(args: &[&str]) -> (i32, String, String) {
    let o = Command::new(env!("CARGO_BIN_EXE_adl-argument-probe"))
        .arg("workflow")
        .args(args)
        .output()
        .unwrap();
    (
        o.status.code().unwrap(),
        String::from_utf8(o.stdout).unwrap(),
        String::from_utf8(o.stderr).unwrap(),
    )
}
fn options(args: &[&str], changes: &[(&str, &str)]) {
    let mut fields = vec![
        ("adl_path", format!("{:?}", args[0])),
        ("print_plan", "false".into()),
        ("print_prompts", "false".into()),
        ("do_trace", "false".into()),
        ("do_run", "false".into()),
        ("out_dir", "\"out\"".into()),
        ("quiet", "false".into()),
        ("do_open", "false".into()),
        ("allow_unsigned", "false".into()),
        ("allow_embedded_signature_key", "false".into()),
        ("signature_key_path", "None".into()),
        ("resume_path", "None".into()),
        ("steer_path", "None".into()),
        ("overlay_path", "None".into()),
    ];
    for (k, v) in changes {
        fields.iter_mut().find(|(name, _)| name == k).unwrap().1 = (*v).into();
    }
    let expected = format!(
        "WorkflowArgs {{ {} }}\n",
        fields
            .iter()
            .map(|(k, v)| format!("{k}: {v}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
    assert_eq!(run(args), (0, expected, "".into()));
}
#[test]
fn defaults_and_literal_paths() {
    options(&["missing.adl.yaml"], &[]);
    options(
        &["space ; $(not-a-command).yaml", "--out", "out ; literal"],
        &[("out_dir", "\"out ; literal\"")],
    );
}
#[test]
fn boolean_switches_and_aliases() {
    for (flag, field) in [
        ("--print-plan", "print_plan"),
        ("--print-prompts", "print_prompts"),
        ("--print-prompt", "print_prompts"),
        ("--trace", "do_trace"),
        ("--run", "do_run"),
        ("--quiet", "quiet"),
        ("--no-step-output", "quiet"),
        ("--open", "do_open"),
        ("--open-artifacts", "do_open"),
        ("--allow-unsigned", "allow_unsigned"),
        (
            "--allow-embedded-signature-key",
            "allow_embedded_signature_key",
        ),
    ] {
        options(&["plan.yaml", flag], &[(field, "true")]);
    }
}
#[test]
fn values_and_repetition() {
    options(
        &[
            "plan.yaml",
            "--resume",
            "r.json",
            "--steer",
            "s.json",
            "--overlay",
            "o.json",
            "--signature-key",
            "key.pem",
        ],
        &[
            ("resume_path", "Some(\"r.json\")"),
            ("steer_path", "Some(\"s.json\")"),
            ("overlay_path", "Some(\"o.json\")"),
            ("signature_key_path", "Some(\"key.pem\")"),
        ],
    );
    options(
        &["plan.yaml", "--out", "first", "--out", "last"],
        &[("out_dir", "\"last\"")],
    );
    options(
        &["plan.yaml", "--out", "--run"],
        &[("out_dir", "\"--run\"")],
    );
}
#[test]
fn help_short_circuits() {
    for flag in ["--help", "-h"] {
        assert_eq!(
            run(&["plan.yaml", flag, "--unknown"]),
            (0, "legacy usage sentinel\n".into(), "".into())
        );
    }
}
#[test]
fn missing_unknown_exit_two() {
    let mut cases=vec![(vec![],"missing ADL yaml path\nTry: adl examples/v0-3-concurrency-fork-join.adl.yaml --print-plan\n".to_owned()),(vec!["plan.yaml","--unknown"],"Unknown arg: --unknown\nRun 'adl --help' for usage.\n".into())];
    for (flag, message) in [
        ("--resume", "a run.json path"),
        ("--steer", "a path to steering patch JSON"),
        ("--overlay", "a path to overlay.json"),
        ("--out", "a directory path"),
        ("--signature-key", "a public key path"),
    ] {
        cases.push((
            vec!["plan.yaml", flag],
            format!("{flag} requires {message}\n"),
        ));
    }
    for (args, message) in cases {
        assert_eq!(
            run(&args),
            (2, "".into(), format!("{message}legacy usage sentinel\n"))
        );
    }
}
