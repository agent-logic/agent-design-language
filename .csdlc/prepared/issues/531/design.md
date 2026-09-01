# Issue #531 Sprint 3 cloud convergence design

## Intent

Close the Sprint 3 umbrella truthfully after all declared roster children have
independently completed. The umbrella does not implement child work; it records
the current roster, live issue dispositions, publication/merge evidence, review
coverage, residual risks, and closeout readiness for the bounded Sprint 3 cloud
convergence wave.

## Roster

The current declared roster from GitHub issue #531 membership version 4 is:

- #495 XCL-01 Cross-cloud Runtime Terraform conversion
- #489 AWS-F AWS Runtime platform modules
- #496 AWS-G AWS CloudFormation retirement decision
- #494 GCP-E GCP GPU readiness smoke test

## Execution boundary

Sprint 3 umbrella execution is a closeout/evidence workflow. It may inspect
child issue, PR, merge, and lifecycle records, and it may add sprint-level
review/closeout artifacts under the issue #531 lifecycle boundary. It must not
change child implementation, rerun paid cloud launches, delete resources, close
or rewrite child issues, or absorb unresolved child scope.

## Evidence model

The sprint result should retain:

- live GitHub issue state for the sprint umbrella and every roster child
- child PR and merge disposition when available
- local C-SDLC record phase and terminal/cleanup truth when available
- ancestry from child merge commits to the sprint closing revision
- skipped, deferred, or residual proof claims without upgrading them to pass
- no paid/AWS/big-runner activity unless separately authorized

## Review model

The sprint-end review is code-facing and evidence-facing. It should check that
the closeout artifact does not overclaim child completion, cloud parity, paid
proof, production cutover, cleanup, or terminal C-SDLC status beyond retained
evidence.
