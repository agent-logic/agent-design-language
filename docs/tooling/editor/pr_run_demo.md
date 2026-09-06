# `pr run` Demo

This is a bounded historical proof surface for the retired `pr run` path.
C-SDLC v2 remains the live lifecycle authority until explicit V3-F/#505
cutover; do not use this demo for live lifecycle routing.

What it proves:

- `adl/tools/pr.sh` exposes a real `pr run` command
- the command delegates to the existing Rust `adl` runtime rather than embedding feature-specific logic
- the command resolves and executes a bounded ADL workflow over the runtime primitives
- the command leaves behind canonical run artifacts that can be inspected deterministically

Historical limitation:

- `pr run` was the supported control-plane run surface when this demo was
  written
- browser/editor direct invocation remains follow-on work

Demo command:

```bash
adl/tools/demo_five_command_run.sh
```

Expected proof surface:

- `<runs_root>/v0-4-demo-deterministic-replay/run.json`
- `<runs_root>/v0-4-demo-deterministic-replay/run_status.json`
- `<runs_root>/v0-4-demo-deterministic-replay/run_summary.json`

Demo note:

- the demo uses an isolated temporary `--runs-root` so it does not leave behind repo-local run artifacts
- historical `pr run` defaulted to the canonical repo-local `.adl/runs/` root
  when `--runs-root` was not supplied

Expected command behavior:

- prints the underlying bounded ADL run
- prints a final `PR RUN ok` summary
- reports the run id, workflow id, and canonical proof artifact paths
