# Design — #632 V3-H.6 real canaries and readiness docs

## Intent

#632 proves the v3 replacement workflow with real issue canaries and prepares
operator-facing readiness material before #505 can cut over authority. v2
remains the live lifecycle authority until #505 is explicitly reviewed and
merged.

## Boundaries

- Use v3 only as construction/canary evidence before #505.
- Use typed v2 for live lifecycle authority, GitHub mutation, publication,
  finish, and cleanup until cutover.
- Capture every canary/tooling defect with a durable disposition.
- Update docs, skills, AGENTS guidance, and the changeover notice so operators
  know what changes before the authority switch.
- Do not merge or close #505 from this issue.

## Execution shape

The work should produce a canary/readiness packet that maps every v3
command-equivalent route to one of three outcomes: real issue canary proof,
focused deterministic fixture proof, or explicit cutover-blocking finding.
Terminal finish and cleanup proof remains deferred until an authorized canary PR
merge exists.
