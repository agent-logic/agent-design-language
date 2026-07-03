use super::*;
use crate::cli::pr_cmd::github::attach_post_merge_closeout;

#[test]
fn attach_post_merge_closeout_reports_failure_output() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-failure");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\necho 'closeout stdout'\necho 'closeout stderr' >&2\nexit 18\n",
    );

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::set_var(
            "ADL_POST_MERGE_CLOSEOUT_CMD",
            tools_dir.join("attach_post_merge_closeout.sh"),
        );
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
    }

    let err = attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect_err("failing closeout helper should bubble up");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }

    let message = err.to_string();
    assert!(message.contains("post-merge closeout auto-attach failed"));
    assert!(message.contains("closeout stderr"));
    assert!(message.contains("stdout: closeout stdout"));
}

#[test]
fn attach_post_merge_closeout_returns_early_when_disabled() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-disabled");
    let repo = temp.join("repo");

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "1");
        env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
    }

    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("disabled closeout helper should be skipped");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }
}

#[test]
fn attach_post_merge_closeout_invokes_helper_successfully() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-success");
    let repo = temp.join("repo");
    let tools_dir = repo.join("adl/tools");
    let argv_log = temp.join("closeout-args.log");
    fs::create_dir_all(&tools_dir).expect("tools dir");
    write_executable(
        &tools_dir.join("attach_post_merge_closeout.sh"),
        &format!(
            "#!/usr/bin/env bash\nset -euo pipefail\nprintf '%s\\n' \"$*\" > '{}'\n",
            argv_log.display()
        ),
    );

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    unsafe {
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var(
            "ADL_POST_MERGE_CLOSEOUT_CMD",
            tools_dir.join("attach_post_merge_closeout.sh"),
        );
    }

    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("closeout helper should succeed");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
    }

    let argv = fs::read_to_string(&argv_log).expect("closeout args");
    assert!(argv.contains("--repo owner/repo"));
    assert!(argv.contains("--issue 1153"));
    assert!(argv.contains("--branch codex/1153-rust-finish-test"));
    assert!(argv.contains("--pr-url https://github.com/owner/repo/pull/1159"));
}

#[test]
fn attach_post_merge_closeout_uses_rust_owned_watcher_when_command_override_is_blank() {
    let _guard = env_lock();
    let temp = unique_temp_dir("adl-pr-attach-closeout-default-watcher");
    let repo = temp.join("repo");
    let bin_dir = temp.join("bin");
    let codex_log = temp.join("codex-args.log");
    fs::create_dir_all(&bin_dir).expect("bin dir");
    write_executable(
        &bin_dir.join("fake-codex"),
        "#!/usr/bin/env bash\nset -euo pipefail\nprintf 'fake-codex-invoked\\n'\nprintf '%s\\n' \"$*\"\nprintf 'GITHUB_TOKEN=%s\\n' \"${GITHUB_TOKEN:-}\"\nprintf 'GH_TOKEN=%s\\n' \"${GH_TOKEN:-}\"\nprintf 'ADL_GITHUB_TOKEN_FILE=%s\\n' \"${ADL_GITHUB_TOKEN_FILE:-}\"\n",
    );

    let old_disable = env::var("ADL_POST_MERGE_CLOSEOUT_DISABLE").ok();
    let old_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CMD").ok();
    let old_codex_cmd = env::var("ADL_POST_MERGE_CLOSEOUT_CODEX_CMD").ok();
    let old_github_token = env::var("GITHUB_TOKEN").ok();
    let old_gh_token = env::var("GH_TOKEN").ok();
    let old_token_file = env::var("ADL_GITHUB_TOKEN_FILE").ok();
    unsafe {
        env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", "0");
        env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", "   ");
        env::set_var(
            "ADL_POST_MERGE_CLOSEOUT_CODEX_CMD",
            bin_dir.join("fake-codex"),
        );
        env::set_var("GITHUB_TOKEN", "ghp_default_closeout_secret");
        env::set_var("GH_TOKEN", "github_pat_default_closeout_secret");
        env::set_var("ADL_GITHUB_TOKEN_FILE", "/tmp/default-closeout-token-file");
    }

    attach_post_merge_closeout(
        &repo,
        "owner/repo",
        1153,
        "codex/1153-rust-finish-test",
        "https://github.com/owner/repo/pull/1159",
    )
    .expect("blank command override should skip the retired helper path");

    unsafe {
        if let Some(value) = old_disable {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_DISABLE", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_DISABLE");
        }
        if let Some(value) = old_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CMD");
        }
        if let Some(value) = old_codex_cmd {
            env::set_var("ADL_POST_MERGE_CLOSEOUT_CODEX_CMD", value);
        } else {
            env::remove_var("ADL_POST_MERGE_CLOSEOUT_CODEX_CMD");
        }
        if let Some(value) = old_github_token {
            env::set_var("GITHUB_TOKEN", value);
        } else {
            env::remove_var("GITHUB_TOKEN");
        }
        if let Some(value) = old_gh_token {
            env::set_var("GH_TOKEN", value);
        } else {
            env::remove_var("GH_TOKEN");
        }
        if let Some(value) = old_token_file {
            env::set_var("ADL_GITHUB_TOKEN_FILE", value);
        } else {
            env::remove_var("ADL_GITHUB_TOKEN_FILE");
        }
    }

    let artifact_root = repo
        .join(".adl/logs/post-merge-closeout")
        .join("issue-1153");
    assert!(artifact_root.join("input.yaml").exists());
    assert!(artifact_root.join("prompt.md").exists());
    assert!(artifact_root.join("pid").exists());
    let input = fs::read_to_string(artifact_root.join("input.yaml")).expect("input");
    assert!(input.contains("watch_pr_until_merged_then_closeout"));
    assert!(input.contains("pr_url: https://github.com/owner/repo/pull/1159"));
    let prompt = fs::read_to_string(artifact_root.join("prompt.md")).expect("prompt");
    assert!(prompt.contains("post-merge closeout watcher"));
    assert!(prompt.contains("issue #1153"));
    assert!(
        !codex_log.exists(),
        "fake codex stdout should be redirected to the durable watcher log"
    );
    let durable_log = artifact_root.join("codex.log");
    let mut codex_args = String::new();
    for _ in 0..20 {
        codex_args = fs::read_to_string(&durable_log).unwrap_or_default();
        if codex_args.contains("fake-codex-invoked") {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    assert!(codex_args.contains("fake-codex-invoked"));
    assert!(!codex_args.contains("ghp_default_closeout_secret"));
    assert!(!codex_args.contains("github_pat_default_closeout_secret"));
    assert!(!codex_args.contains("/tmp/default-closeout-token-file"));
}
