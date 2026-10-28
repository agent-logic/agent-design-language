# CSM Polis Shepherd Agent Runtime Packet for #5135

Issue: `#5135 [v0.91.7][WP-07A][runtime] Implement polis Shepherd Agent as CSM operator agent`

## Runtime Role

The Polis Shepherd Agent is a first-class CSM runtime component. It is not a host service manager, shell wrapper, or blind restart loop. It is the agent-backed operator component that watches CSM health, continuity, policy, and evidence surfaces, then emits typed advisory decisions for runtime policy gates to admit or reject.

The Shepherd consumes these runtime inputs:

- daemon lifecycle and runtime health
- checkpoint and continuity status
- lifelog and operator-event state
- observability and OTel retention state
- Freedom Gate, CAV, and constitutional policy status
- cloud-bridge notice status when configured

The Shepherd emits typed decisions using schema `adl.csm.shepherd_agent.decision.v1`:

- `preserve`
- `resume`
- `quarantine`
- `degrade`
- `escalate`
- `quiesce`
- `safe_fail`

Model output is advisory only. A Shepherd recommendation cannot bypass Freedom Gate, CAV, constitutional policy, checkpoint validation, or runtime admission policy.

## Implementation Surface

Implemented runtime surfaces:

- Rust module: `adl/src/csm_shepherd_agent.rs`
- Daemon status capability: `runtime_capabilities.polis_shepherd_agent`
- Retained daemon snapshot: `state/csm_shepherd_agent_status.json`
- Embedded API status: `/status.polis_shepherd_agent`
- Embedded API route: `/shepherd`

The daemon writes the retained Shepherd status snapshot whenever it updates `daemon_status.json`. The API can still synthesize a safe fallback response if the retained snapshot is not yet present, but the proving path is the actual daemon-written artifact.

## Local Model Policy

Resident candidate under test:

| Field | Value |
| --- | --- |
| Model | `gemma4:12b-mlx` |
| Local runtime | Ollama |
| Architecture | `gemma4_unified` |
| Parameters | `12.4B` |
| Context length | `262144` |
| Quantization | `nvfp4` |
| Capabilities | completion, tools, thinking |
| Local observed Ollama version | `0.31.1` |
| Local observed model size | `7.7 GB` |
| License | Apache-2.0 |

Fallback policy:

- Resident fallback: `Qwen3.5:9b`
- Low-memory triage: `FastContext-4B`
- Diagnostic fallback: `qwen3-coder:30b`
- Heavy incident escalation only: `Qwen3.5:35b-a3b`

`gemma4:12b-mlx` is not the default Shepherd authority until Shepherd-specific evaluation proves decision quality, refusal discipline, typed tool-call formatting, latency, memory pressure behavior, and degraded-runtime behavior.

## Gemma 4 MLX Research Note

The Ollama 0.31 MLX MTP announcement dated 2026-06-29 states that Gemma 4 MLX generation on Apple Silicon is nearly 90% faster on average across a coding-agent benchmark, with multi-token prediction enabled by default and no output change. Local metadata confirms `gemma4:12b-mlx` is present and compatible with Ollama 0.31.1.

## Negative-Case Policy

The Shepherd must not:

- blindly restart a corrupted state
- skip checkpoint validation
- advance false cloud progress
- treat model output as policy authority
- hide missing continuity behind a healthy status

Current deterministic classification:

| Observed condition | Advisory decision |
| --- | --- |
| checkpoint continuity missing | `safe_fail` |
| backpressure degraded or critical | `degrade` |
| active uncertain agent state, such as `running_cycle`, `leased`, or `failed` | `quarantine` |
| governed terminal or healthy recoverable state | `preserve` |

## Validation Plan

Focused validation for this issue:

- `cargo test --manifest-path adl/Cargo.toml csm_shepherd_agent -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml csm_runtime_api -- --nocapture`
- `cargo test --manifest-path adl/Cargo.toml --test cli_smoke csm_runtime_api_serves_status_health_ready_metrics_and_events -- --nocapture`
- `ollama --version`
- `ollama show gemma4:12b-mlx`
- `git diff --check`

## Non-Claims

This packet does not claim that Gemma 4 MLX is the default Shepherd model. It records Gemma as the resident candidate under test. It also does not claim that the broader Vector-style runtime topology is complete; #5068 remains the large simplification and crate-backed topology issue.
