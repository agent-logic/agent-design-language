# Structured Planning Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

Implement and prove the bounded Shepherd foundation on Mac with the configured MLX model, retain one immutable portable Ollama/CUDA model bundle in versioned S3 for AWS and distributed Polis hosts, and run the fixed g6.xlarge CUDA proof after quota approval. Keep authenticated WSS, Observatory projection, and browser round-trip explicitly deferred behind issue 5832.

## Plan

Revision 17

## Steps

[
  {
    "id": "S1",
    "action": "Confirm issues 5800 and 5820 are stable, identify issue 5832 as unresolved, and restrict this pass to the bounded Shepherd foundation and portable model proof surfaces.",
    "acceptance_ids": [
      "AC-2",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S2",
    "action": "Implement the explicitly configured bounded local process adapter with truthful execution classification, cleared environment, strict bounds, concurrency control, timeout, cancellation, child reaping, runner-byte pinning, and redacted failures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S3",
    "action": "Complete focused Rust and Mac MLX proof, retain the portable Ollama/CUDA model bundle by exact S3 object versions, and validate the no-launch AWS GPU preflight with zero residual resources.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "completed"
  },
  {
    "id": "S4",
    "action": "After the us-west-2 On-Demand G and VT quota reaches four vCPUs, run the fixed g6.xlarge CUDA proof against the exact model manifest and exact source head, then prove automatic instance and volume cleanup.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S5",
    "action": "After issue 5832 freezes the command and WSS contract, complete authenticated Runtime and Observatory integration, live browser proof, exact-head review, and final publication.",
    "acceptance_ids": [
      "AC-2",
      "AC-5",
      "AC-6",
      "AC-8"
    ],
    "status": "pending"
  }
]

## Invariants

- Unsigned, unauthorized, malformed, oversized, or wrong-runtime messages fail before model invocation
- No cloud fallback or silent model substitution
- Real, deterministic-test, retained, and unavailable outcomes remain distinguishable
- Timeout/cancellation releases bounded permits and preserves Runtime usability
- Prompts, tokens, model paths, and private response content obey redaction policy

## Risks

- A fake or cached response could be misreported as real
- Local process invocation could escape timeout or cancellation
- Model absence could make startup incorrectly fail
- Browser transport could bypass signed governed ingress
- Sensitive prompt/response or model path data could enter logs

## Estimates

{
  "elapsed_seconds": 43200,
  "total_tokens": 140000,
  "validation_seconds": 7200
}

## Design

.csdlc/prepared/issues/5795/design.md

Digest: 714e32d250ea000b158c6f2940af60db47219a38ada6ff2c6b01d438be5a76c9

## Diagram

.csdlc/prepared/issues/5795/diagram.mmd

Digest: 7db206956bb1d4ac5d32aae7445ca356c16a90e7d8cc28bb40054e448880a2dd

## Stop Conditions

- Issues 5800 or 5820 are not stable for integration
- Issue 5832 contract changes would make the adapter route speculative
- The implementation requires cloud fallback or global default mutation
- Real and fake execution cannot be distinguished in retained evidence
- A model timeout or crash can take down Runtime

## Handoff

Proceed only after doctor readiness.
