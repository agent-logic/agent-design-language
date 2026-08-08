# Structured Planning Prompt

Template: 1.0.0

Issue: 5795

Repository: danielbaustin/agent-design-language

Card: spp

Status: ready

## Summary

With issues 5800 and 5820 stable but issue 5832 still unresolved, implement and validate only the bounded local Gemma/MLX Shepherd process-adapter and failure-isolation foundation. Retain authenticated WSS, Observatory projection, browser round-trip, and final issue publication as explicitly pending work after 5832 freezes the command and carrier contract.

## Plan

Revision 14

## Steps

[
  {
    "id": "S1",
    "action": "Confirm issues 5800 and 5820 are stable, identify issue 5832 as unresolved, inventory governed adapter boundaries, and restrict this pass to the three owned Rust foundation paths.",
    "acceptance_ids": [
      "AC-2",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S2",
    "action": "Implement the explicitly configured bounded local process adapter with truthful execution classification, cleared environment, strict bounds, concurrency control, timeout, cancellation, explicit child reaping, and redacted failures.",
    "acceptance_ids": [
      "AC-1",
      "AC-2",
      "AC-3",
      "AC-4",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S3",
    "action": "Prove governed admission and failure isolation deterministically, compile both crate surfaces with warnings denied, and run one explicitly configured real local Gemma smoke; keep WSS and browser proof deferred behind issue 5832.",
    "acceptance_ids": [
      "AC-1",
      "AC-3",
      "AC-4",
      "AC-5",
      "AC-6",
      "AC-7"
    ],
    "status": "pending"
  },
  {
    "id": "S4",
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

Digest: 89fe1dc1292243e40a2ec48d47040e901ac91451a8d5d99945fb154d8b3d1037

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
