# Issue 5912 Design: Runtime-Owned Birth-Witness Boundary

## Context

Issue #5833 delivered the deterministic birth-witness algorithm and receipt contract. The trusted policy constructor remains crate-private and the crate has no non-test Runtime owner that provisions policy and invokes the build/validate path.

## Design

Add a small Runtime-owned service in `adl-runtime-kernel` that is constructed from trusted roster configuration inside the crate. The service retains the opaque `BirthWitnessPolicy`, accepts candidate context plus attestations, calls `build_birth_witness_packet`, validates the resulting packet with `validate_birth_witness_packet`, canonically serializes the receipt, and emits it through an injected receipt sink.

The public integration boundary exposes trusted configuration inputs and an output sink, but never exposes the policy internals or creates a caller-selectable bypass around validation. Existing cryptographic, canonicalization, privacy, rejection, and non-authority semantics remain unchanged.

## Proof

A focused external integration test constructs the Runtime service through its production constructor, submits a valid witness set, verifies build and validation are both exercised, and confirms the exact canonical receipt is emitted by the sink. Focused unit/integration tests cover sink failure and invalid witness rejection without emission.

## Non-Goals

- Changing the witness algorithm or receipt schema.
- Adding broad Runtime orchestration.
- Claiming birthday, citizenship, governance, legal, or launch authority.
- Modifying historical #5833 records or evidence.
