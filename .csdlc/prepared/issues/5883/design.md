# Issue 5883 design: retire duplicate `csdlc-init`

Status: design approved for preparation.

## Decision

Make `csdlc-issue create` the sole issue-creation entrypoint and delete the
duplicate `csdlc-init` binary and every active requirement for it. Do not add an
alias, wrapper, compatibility mode, or second command path. Historical evidence
remains unchanged.

## Dependency and sequencing

This follows #5861, which established claim-free creation. Execute after #5895
because both issues touch the installer/coexistence inventory and Gate 10A
proof. Rebase onto the terminal #5895 result before implementation. #5881 may
update overlapping operator guidance and should also be rebased when necessary.

## Change boundary

- Delete the Cargo binary declaration and source file for `csdlc-init`.
- Remove it from coexistence/install/proof inventories and focused installer
  fixtures.
- Update active operator skills, command adapters, platform taxonomy, README,
  and current runbooks to use `csdlc-issue --root <repo> create --request <json>`.
- Preserve historical task records, review evidence, and immutable Gate 10
  packets even when they mention the retired name.
- Keep card creation semantics and `csdlc-bind` behavior byte-for-byte
  equivalent outside command routing.

## Proof model

1. An inventory test distinguishes active authority from historical evidence.
2. Focused create tests exercise the real `csdlc-issue create` binary, including
   invalid-before-write, idempotent replay, and conflicting initialization.
3. Installer/coexistence tests prove `csdlc-init` is absent from the declared and
   installed set and reject its reappearance.
4. One installed create/validate/doctor/bind canary proves the operator route.
5. Current docs and installed skills contain no active `csdlc-init` instruction.

## Invariants

- One creation command, one typed request, one card renderer.
- No claims, leases, preparation state, wrappers, or shell lifecycle logic.
- No broad product test suite.
- No historical evidence edits.
- Issue authority remains `danielbaustin/agent-design-language#5883`.
- Code publication targets `agent-logic/agent-design-language`; its PR body
  must use `Closes danielbaustin/agent-design-language#5883`.
- This is split issue/code publication authority, not repository cutover or
  issue migration.

## Failure behavior

Fail closed if any active installer/skill/doc route still names `csdlc-init`, if
the installed set contains the binary, or if `csdlc-issue create` differs in
state/card output from the established contract.
