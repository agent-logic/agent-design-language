# Issue 515 design

Status: ready for execution after typed approval.

## Objective

Issue #515 implements PROV-B for v0.92.1: one local-model shadow-execution
path that observes the same declared inputs as the authoritative provider path
without gaining authority over returned results, lifecycle state, provider
profile activation, or production routing.

## Scope

The implementation must stay inside the provider surface and the PROV-B evidence
packet:

- `adl/src/provider/mod.rs`
- `adl/src/provider/local.rs`
- `adl/src/provider/profiles.rs`
- issue-owned provider shadow tests under `adl/tests/`
- issue-owned PROV-B evidence under
  `docs/milestones/v0.92.1/evidence/provider/prov-b/`

The issue may add narrowly named helpers only when they preserve the same
authority boundary and are covered by the planned validation lanes.

## Authority model

The authoritative provider path remains the only path that can return the
accepted result. Shadow execution is an observation channel. Its output may be
recorded as a redacted comparison observation, but it must not:

- replace or mutate the authoritative result;
- update lifecycle or provider profile state;
- alter production routing or fallback selection;
- introduce a hidden paid, cloud, or ambient-provider dependency.

Shadow and authority values should therefore be represented by distinct types,
fields, or result channels so review can trace which path produced which
observation.

## Deterministic comparison model

Comparison inputs must be exact and reproducible. The implementation should use
bounded local fixtures or deterministic local-model doubles for PROV-B proof,
then emit a comparison record that identifies:

- the authoritative input digest;
- the shadow input digest;
- the comparison rule set;
- the authoritative outcome class;
- the shadow observation class;
- the redaction result.

The comparison record is evidence, not authority.

## Fallback model

If the shadow path fails, times out, produces malformed output, or violates the
comparison contract, the authoritative result must be preserved exactly. The
failure may be represented in redacted evidence, but it must not mask an
authoritative failure or convert a shadow success into an authoritative success.

## Redaction model

PROV-B evidence must redact credentials, private prompts, provider payloads, and
host-local paths. The evidence should be machine-checkable enough for the
issue-owned redaction harness to reject common leakage markers before review.

## Validation

Execution should prove the four issue PVF lanes:

- `shadow-isolation`
- `deterministic-comparison`
- `fallback`
- `redaction`

The local validation plan intentionally avoids AWS, paid runners, production
provider calls, and `/private/tmp`.
