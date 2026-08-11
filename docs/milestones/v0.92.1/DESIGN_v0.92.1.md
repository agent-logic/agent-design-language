# v0.92.1 Design

## Parallel Program Structure

The milestone uses three independent execution lanes and one integration tail.

```mermaid
flowchart LR
  P["Planning issue #146"] --> O["WP-01 milestone opening"]
  O --> A["Corporate and IP lane"]
  O --> B["C-SDLC v3 lane"]
  O --> C["Distributed Runtime qualification"]
  A --> D["INT-01 Demo convergence"]
  B --> D
  C --> D
  D --> Q["INT-02 Quality gate"]
  Q --> L["INT-03 Docs and review alignment"]
  L --> I["INT-04 Internal review"]
  I --> E["INT-05 External review"]
  E --> F["INT-06 Remediation and final preflight"]
  F --> N["INT-07 Next-milestone planning"]
  N --> H["INT-08 Next-milestone review"]
  H --> X["INT-09 Release ceremony and lifecycle closeout"]
```

## Milestone Opening

Issue `#146` defines planning truth only. WP-01 creates the future live issues,
cards, exact mapping, readiness evidence, and start gate. No execution lane may
bind directly from this planning PR.

## Lane A

Lane A freezes a critical-asset schedule, obtains counsel-reviewed assignment and corporate acceptance evidence, audits provenance and licensing, establishes company account custody, migrates infrastructure, and produces a redacted due-diligence index. Private agreements and secrets remain outside the repository.

## Lane B

Lane B implements the reviewed C-SDLC v3 Rust architecture without changing its accepted contract. Its hard gates include the construction spike, all eleven operator decisions, Decision 11 before V3-08, deterministic state and recovery, exact review, remote idempotency, parity, canary operation, and single-writer cutover. V3-R01 remains deferred beyond the rollback window.

## Lane C

Lane C consumes terminal `#142` and WP-04.16 production proof. It qualifies exactly three voters in one polis, three governed agents, one non-voting shepherd, and exactly one quorum-leased Observatory. It runs serial Wuji-only and Wuji-plus-private-AWS windows, then validates quorum, fencing, snapshots, restart, partition, replay, security, observability, resources, soak, and cleanup.

## Integration

The integration tail follows the standard nine-step sequence: demo convergence,
quality gate, docs and review alignment, internal review, external review,
remediation and final preflight, next-milestone planning, next-milestone review,
and operator-authorized release ceremony with lifecycle closeout. It cannot
substitute one lane's evidence for another or collapse independent gates.
