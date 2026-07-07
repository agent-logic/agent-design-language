# ADL Platform CLI Binary Taxonomy

Status: current architecture guidance for v0.91.7 and later
Related ADR: [ADR 0043](../adr/0043-adl-platform-cli-binary-taxonomy.md)
Related issues: #4983, #4989, #4995, #4726, #4906, #4977, #4979, #4980

## Purpose

ADL is no longer one command with a few helpers. It is becoming a platform with
separate language, runtime, runtime-administration, C-SDLC, validation, and
future product surfaces. This document records the binary taxonomy and command
ownership rules so new work does not keep dumping unrelated behavior into the
largest available executable.

This document is an architecture and migration guide. It does not implement the
taxonomy by itself. Implementation is tracked separately, including #4995 for
the first ADL Platform CLI taxonomy implementation slice.

## Current Truth

The repository currently has both:

- compatibility surfaces under the monolithic `adl` binary; and
- owner binaries for several hot workflow and validation paths.

The current owner-binary inventory is recorded by the WP-06 owner-binary slice
in `docs/milestones/v0.91.7/review/build_throughput/ADL_OWNER_BINARY_DECOMPOSITION_4726.md`.
That slice proves extraction for `adl-session` and `adl-process`, and records
existing PR lifecycle owner binaries such as `adl-pr-run`, `adl-pr-doctor`,
`adl-pr-finish`, `adl-pr-validation`, `adl-pr-inventory`, `adl-pr-shepherd`,
and `adl-pr-closeout`.

The monolithic `adl` command remains a compatibility surface. It must not be
treated as the permanent owner for every new platform concern.

## Target Taxonomy

| Binary family | Primary responsibility | Audience | Authority boundary |
| --- | --- | --- | --- |
| `adl` | ADL language compiler/manager and stable user-facing ADL entrypoint | ADL authors and maintainers | Owns language/document management, not runtime administration or C-SDLC PR control. |
| `csm` | Cognitive Spacetime runtime daemon and runtime execution surfaces | runtime operators and runtime agents | Owns runtime behavior, daemon state, local runtime API, observability API, and CSM execution. |
| `csmctl` | CSM runtime administration/control-plane client | runtime operators | Owns operator commands against CSM runtime instances; must not become a dumping ground for language, C-SDLC, or unrelated utilities. |
| `csdlc` / `adl-csdlc` | C-SDLC workflow control plane | issue workers, reviewers, release operators | Owns issue execution, cards, PR lifecycle, validation planning, review, release evidence, and closeout surfaces. |
| `adl-pr-*` | Narrow PR lifecycle owner binaries | workflow automation and agents | Own exactly one PR lifecycle concern each and keep validation blast radius small. |
| `adl-session` | Session-ledger claim, heartbeat, status, and release | agents and workflow operators | Owns session coordination, not PR execution or runtime state. |
| `adl-process` | Permission-safe process status checks | agents and workflow operators | Owns bounded PID/port checks, not broad host process scanning. |
| `adl-runtime` | Compatibility/runtime-oriented command surface during migration | runtime maintainers | Transitional owner for runtime command families until `csm`/specific runtime binaries own them directly. |
| `adl-review` | Review tooling surfaces | reviewers and review automation | Owns review helpers, not issue mutation or runtime execution. |
| `adl-remote` / `adl-aws-remote-validation` | Remote validation and AWS remote-build lanes | validation operators | Own remote build/validation orchestration and remote artifacts, not general AWS product control. |
| `tools/*` | Bounded helper scripts and adapters | maintainers and automation | Helpers must have one purpose, a validation lane, and a clear path to owner binaries when promoted. |

## Binary Ownership Rules

1. A binary has one primary owner domain.
2. A command belongs in the binary whose domain owns the operational authority,
   not merely the binary that is easiest to edit.
3. Runtime behavior belongs under `csm` or a runtime-owned binary, not under
   C-SDLC workflow tooling.
4. Runtime administration belongs under `csmctl`, not under `adl` or generic
   helper scripts.
5. C-SDLC issue, card, validation, PR, review, and closeout control belongs
   under `csdlc`, `adl-csdlc`, or the `adl-pr-*` owner binaries.
6. Utility scripts stay in `tools/*` only while they are bounded helpers. If a
   helper becomes part of the normal workflow, it needs an owner-binary decision.
7. New binaries require an explicit ownership boundary, operational need,
   validation profile, documentation target, and migration/compatibility plan.

## Migration Guidance

Use this sequence when moving a command out of a broad compatibility surface:

1. Identify the command's actual authority boundary.
2. Choose the owning binary family from the target taxonomy.
3. Add or update a narrow owner binary before removing compatibility entrypoints.
4. Keep compatibility aliases only long enough to preserve operator workflow.
5. Add focused validation for the owner binary and compatibility alias.
6. Update docs, skills, VPP/PVF lane mapping, and PR finish routing together.
7. Record the migration in the issue SOR and, when architecture-relevant, an ADR
   or ADR addendum.

Compatibility commands should call into the same implementation path as the
owner binary. They must not fork behavior, validation truth, or GitHub/runtime
state interpretation.

## Current Command Placement Guidance

| Current or planned surface | Target owner | Notes |
| --- | --- | --- |
| `adl session ...` | `adl-session` | Compatibility alias only once `adl-session` is available. |
| `adl process ...` | `adl-process` | Compatibility alias only; normal workflow should prefer permission-safe owner binary. |
| `adl pr ...` / `pr.sh` issue lifecycle | `adl-pr-*` / `adl-csdlc` | `pr.sh` remains the taught wrapper while owner binaries mature. |
| GitHub issue/PR metadata | shared PR control-plane client | See `docs/tooling/ADL_CSDLC_GITHUB_CLIENT_BOUNDARY.md`; do not reintroduce raw `gh` as the normal backend. |
| CSM runtime API | `csm` | #4929 proves `csm api serve`; `adl csm api` rejects runtime API ownership. |
| CSM daemon and supervised runtime | `csm` | Runtime behavior and retained runtime artifacts belong to the CSM runtime owner. |
| CSM operator administration | `csmctl` | Start/stop/status/admin commands should land here when they are control-plane client operations rather than daemon internals. |
| AWS runtime signal operations | runtime/AWS owner surfaces | WP-08 AWS signal hooks must not be hidden inside generic language or docs tooling. |
| remote validation | `adl-remote` / `adl-aws-remote-validation` | Remote builders own remote artifact/log behavior and should feed validation manager evidence. |
| prompt/card editors | `adl-csdlc` or specific editor binaries | Editing lifecycle truth is C-SDLC control-plane behavior, not runtime behavior. |
| future `obsmem` | future explicit owner | Requires operational need and storage/retrieval boundary before becoming a binary. |
| future `polis` | future explicit owner | Requires product/runtime authority definition before becoming a binary. |
| future `guild` | future explicit owner | Requires governance/role boundary before becoming a binary. |
| future `aptitude` | future explicit owner | Aptitude Atlas remains post-v0.95 unless explicitly promoted by issue. |

## Anti-Patterns

Avoid these patterns:

- adding every new subcommand to `adl` because it already exists;
- using `csmctl` as a generic operations junk drawer;
- hiding C-SDLC issue/PR/card control inside language tooling;
- giving runtime commands authority to mutate issue lifecycle state;
- duplicating GitHub metadata interpretation in multiple binaries;
- building helper scripts that become required workflow without an owner
  binary, validation lane, and documentation;
- treating compatibility aliases as the canonical architecture.

## Validation And Review Implications

The taxonomy exists to reduce validation blast radius. A change to one command
family should not force unrelated runtime, review, and C-SDLC tests unless the
changed code crosses those boundaries.

Each owner binary should have:

- a focused validation lane;
- explicit docs or help text;
- stdout/stderr and observability expectations when it emits machine-readable
  output;
- a release/readiness policy for whether it is required, optional, or
  compatibility-only;
- clear dependency ownership when it consumes shared libraries.

When a command remains in the compatibility `adl` binary, the issue should state
whether it is:

- `compatibility_alias`: owned elsewhere, retained for users;
- `transitional_owner`: waiting for a named owner-binary issue;
- `monolith_residual`: known remaining surface without an owner yet.

## Relationship To WP-08 And WP-12

This taxonomy does not broaden WP-08 or WP-12. It only says where their command
surfaces should live once implemented.

- WP-08 AWS/signal operations should use runtime/AWS owner surfaces and retain
  Agent Logic AWS-account guardrails.
- WP-12 security/protocol work should not be hidden behind generic workflow
  commands; protocol, security, and runtime authority need explicit owners.

## Relationship To v0.95

This document informs v0.95-era platform cleanup, but it does not claim v0.95
completion. Future binaries such as `obsmem`, `polis`, `guild`, or `aptitude`
remain candidates until an issue proves the operational need, authority
boundary, validation lane, and migration plan.

## Non-Claims

- This document does not implement binary migration.
- This document does not remove compatibility entrypoints.
- This document does not claim `adl` is no longer a monolithic compatibility
  binary.
- This document does not claim WP-08, WP-12, or #4906 completion.
- This document does not promote Aptitude Atlas into MVP scope.
- This document does not make any future binary first-class without a tracked
  implementation issue and proof.
