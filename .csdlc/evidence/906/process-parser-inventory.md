# Issue #906 process-status parser inventory

Baseline: `b054a2510c13f39e7d869534087d58a9b94d204d` (`origin/main` at bind).

## Preserved overlapping work

The historical worktree `/Users/daniel/git/agent-design-language/.worktrees/adl-process-status-fanout` remains unchanged on `codex/reduce-process-status-fanout` at `1ea914010e6b96482a95bd6f64c8318f1a19b937`. Its four-file dirty patch was last modified on 2026-06-19 and has SHA-256 `aef64c67a22d74ff5dc4962d5c6f25ab09a06ca64bb1763c48984d73558fbf84`. Neither `origin` nor `legacy-origin` has that branch, and neither repository has a PR for it.

Issue #906 does not copy, reset, cherry-pick, clean, or modify that worktree. It executes from the separately bound FastWork worktree on current `origin/main`; the historical source, CLI-test, and unrelated finish-file bytes remain preserved.

## Before

- `adl/src/cli/process_cmd.rs`: 482 total lines, 431 production lines before `#[cfg(test)]`.
- SHA-256: `873b68779914fb12f2d790f7e220948ed8ff883eba4164a3808ad50c5a279aba`.
- Production function inventory: 18 functions: `real_process`, `real_process_status`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, `validate_loopback_host`, `classify_status`, `read_pid_file`, `pid_report`, `port_report`, `port_accepts_tcp_connection`, `base_report`, the Unix and non-Unix `pid_is_live` implementations, `print_human_report`, `process_usage`, and `status_usage`.
- Lexical production control-site inventory: 28 sites: 16 `if`, 11 `match`, and one `while`.
- Parser ownership: `ParsedStatus`, `parse_status_args`, `take_value`, `parse_pid`, `parse_port`, and `validate_loopback_host` shared the probing/output module.
- Target-selection state: four independent `Option` values (`pid`, `pid_file`, `port`, and `name`) plus `host`.
- Target resolution: build four booleans, count them, branch on cardinality, then repeat target precedence through an `if`/`else if` chain ending in `expect`.
- Production caller: `real_process_status` called `parse_status_args`; PID-file classification reused `parse_pid`.

## After

- `adl/src/cli/process_cmd.rs`: 338 total lines, 287 production lines.
- `adl/src/cli/process_cmd/args.rs`: 313 total lines, 175 production lines.
- SHA-256: `adl/src/cli/process_cmd.rs` = `b8a82745dc1d54f0cc922e40fd4515ec4f3cb6b12c16bcf7b47b1473a21a1f9c`; `adl/src/cli/process_cmd/args.rs` = `ab64ba234d81bf23ef3bcf2430c7369fecc38449d3feed9235843e99f4578c74`; `adl/tests/cli_smoke/process_status.rs` = `5059bd93ee0b159412d4bcacf3b7387b63b22cc27f9e232796f7f751251634ea`.
- Recursive process-command source: 651 lines total and 462 production lines. The 169-line total increase consists of 138 lines of focused parser tests and 31 production lines for the module boundary, typed conflict tracking, and explicit visibility. The parent owner shrinks by 144 production lines.
- Production function inventory: 19 functions. The original 18 responsibilities remain, and the extracted parser adds one small `set_target` helper to centralize same-kind replacement and cross-kind conflict tracking.
- Lexical production control-site inventory: 27 sites: 15 `if`, 11 `match`, and one `while`. The extraction therefore removes one production branch site while replacing the target-count pass, repeated target-selection chain, and panic-bearing terminal selection with typed state.
- Target-selection state: one `Option<Check>` plus one conflict bit. Repeated options of the same target kind replace the value; distinct target kinds set the conflict bit while parsing continues, preserving later value/unknown-option error precedence.
- Target resolution: one conflict check and one missing-target conversion; there is no target-count pass, repeated precedence chain, or panic-bearing `expect`.
- `real_process_status` remains the production caller. PID-file classification imports the same extracted `parse_pid`; probing, syscall/network behavior, report schema, help text, stdout, and stderr behavior stay in the parent owner.

## Counting method

Production lines and function/control-site counts stop before the first `#[cfg(test)]` in each source file. Functions are counted from owned Rust function definitions with bodies, excluding the foreign `kill` declaration inside the Unix implementation. Control sites are the exact lexical matches from `rg -o '\b(if|match|while)\b'`; this is a repeatable structural inventory rather than a cyclomatic-complexity claim. The before inventory applies that method to the baseline `process_cmd.rs`; the after inventory applies it separately to `process_cmd.rs` and `process_cmd/args.rs`, then sums the results.

## Behavior matrix

The parser and installed CLI proof covers all four targets, default and reordered hosts, repeated targets and `--json`, conflicting targets, missing and flag-shaped values, unknown flags, error precedence after a conflict, PID/port zero and maximum boundaries, empty names, loopback-only hosts, and the pre-existing acceptance of a valid `--host` alongside a non-port target.

## PVF classification

| Surface | Lane | Proof role | Determinism | Resources | Release gate |
| --- | --- | --- | --- | --- | --- |
| `process_cmd::args::tests` through `cargo test --bins` | tooling | Pure parser contract and error-order regression | deterministic | local CPU/Rust only | required for #906 |
| `cli_smoke process_status` | tooling | Installed `adl process status` behavior and safety regression | deterministic except bounded owned loopback listener/PID observation | local CPU, filesystem, owned process, loopback only | required for #906 |

No new fixture family or external service is introduced. The tests extend existing Rust unit and `cli_smoke` surfaces.
