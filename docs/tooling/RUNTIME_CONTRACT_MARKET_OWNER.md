# Contract-market fixture owner

The v0.90.4 fixture packet, deterministic runner, Markdown renderer and their
three original Bash consumers belong to [Runtime at 7d0db620](https://github.com/agent-logic/agent-logic-runtime/tree/7d0db620ad79c50ff5d8fabf0fb543183583d05c/demos/fixtures/contract_market).
ADL no longer provides these local source files. Select a complete reviewed
Runtime checkout explicitly when inspecting or running this contract.

The exact 28 owner files were qualified together in a fresh Linux container
with no network, credentials, production mounts, Rust builds or native runtime
execution. All three original consumers passed once: fixture protocol,
runner, and summary. Their original repeatability, golden output, negative
fixtures, path refusal and missing/invalid-input assertions were preserved.
This proves the deterministic fixture substrate, not live contract-market
agents, native Runtime execution, providers, or broad owner acceptance.

For an isolated copy of the complete reviewed owner closure:

```bash
case "$RUNTIME_CONTRACT_MARKET_SOURCE_ROOT" in /*) ;; *) exit 2;; esac
cd "$RUNTIME_CONTRACT_MARKET_SOURCE_ROOT" || exit 2
bash adl/tools/test_v0904_contract_market_fixture_set.sh
bash adl/tools/test_v0904_contract_market_runner.sh
bash adl/tools/test_v0904_contract_market_summary.sh
```

The original runner and summary tests use fixed absolute `/tmp` destinations
for negative cases. Use a disposable isolated filesystem, as in the retained
qualification; these instructions do not select a shared or production checkout.
The complete original source and fixture identities remain in #1223 rollback
custody. Historical milestone and review records retain their original paths.
