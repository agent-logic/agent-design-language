# #1159: Runtime/provider and regression-proof corrections

Group C of the four-issue #919 remediation wave. [Finding/fix/proof mapping](FINDING_FIX_PROOF.json) retains all five original finding IDs and their production-versus-proof classifications. No finding was invalidated by the current-source check. The #918 frozen candidate and #919 review artifacts are unchanged.

Resident health now joins provider accounting using the declared model reference, preserving the native ID for display. HTTP-family consumers enforce a 4 MiB response ceiling before parsing, so small extracted output cannot conceal an oversized unused field. Existing separately bounded Ollama metadata and Bedrock SDK consumers retain their own limits.

The Observatory check binds announcement attributes to the two transcript elements; the cancellation check measures virtual time; and #905 refuses to assert equal weights when base-blob identity is missing or malformed. These three changes correct proof gaps, without claiming current missing UI attributes, missing production cancellation, or differing weights in retained experiments.

## Reproduce the focused checks

From this issue checkout, keep Cargo output and temporary files in this worktree:

```sh
mkdir -p .csdlc/evidence/1159/tmp
export CARGO_TARGET_DIR="$PWD/adl/target"
export TMPDIR="$PWD/.csdlc/evidence/1159/tmp"
cargo test --manifest-path adl-provider-core/Cargo.toml --lib
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --lib resident_health
cargo test --manifest-path adl-runtime-kernel/Cargo.toml --test shepherd resident_shepherd_shutdown_interrupts_pending_probe
PYTHONDONTWRITEBYTECODE=1 python3 adl/tools/test_issue905_runtime_speculative_retest.py
node demos/html-observatory/tests/accessibility_responsive.test.mjs
```

Local results: 138 provider tests, 16 health tests, one cancellation test, 11 offline model-identity tests, and the accessibility proof with two independent missing-attribute negatives. The cancellation mutation temporarily replaced the production pending-probe `tokio::select!` with an unconditional timeout await. The same test failed at its timing assertion (zero passed, one failed); the source was then restored byte-identically. This was virtual time, not a real 600-second wait. The mutation hashes and result are retained in the mapping.

No provider credentials, paid experiments, Runtime restarts, resident changes, deployment or merge occurred. This is local regression proof, not installed Beta 1 qualification. Native proof, independent exact-head review and CI must be inspected separately for the published head.
