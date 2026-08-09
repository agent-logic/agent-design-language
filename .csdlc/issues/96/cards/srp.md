# Structured Review Prompt

Template: 1.0.0

Issue: 96

Repository: agent-logic/agent-design-language

Card: srp

Status: pre_phase

## Scope

.csdlc/prepared/issues/5862/validate-implementation-wave.rb
.csdlc/prepared/issues/5862/test-validate-implementation-wave.rb
.csdlc/prepared/issues/96
.csdlc/issues/96
.csdlc/evidence/96

## Prompts

- Can any post-S product mutation evade the validator?
- Can evidence be replaced or modified after E?
- Are H, merge ancestry, terminal envelopes, and mappings exact and unambiguous?
- Are all sixteen children, paths, DAG edges, and #5878 native receipts still mandatory?
- Does any self-referential fake evidence pass?

## Findings

[
  {
    "id": "R96-01",
    "severity": "p1",
    "summary": "Require the proof path to be a strict descendant of the frozen evidence mapping.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-02",
    "severity": "p2",
    "summary": "Require each declared product path to resolve to exactly one ordinary blob.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-03",
    "severity": "p1",
    "summary": "Correct the frozen authoritative dependency sets for issues 5875 and 5877.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-04",
    "severity": "p1",
    "summary": "Validate canonical nested native receipt command, runner, and artifact structure.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-05",
    "severity": "p1",
    "summary": "Fully validate generic child WP, test, runner, log, artifact, and negative-case proof fields.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-06",
    "severity": "p1",
    "summary": "Model real bounded legacy v2 evidence that predates source and is refreshed exactly once.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-07",
    "severity": "p1",
    "summary": "Contain every proof-referenced evidence file within the exact frozen manifest prefix.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-08",
    "severity": "p1",
    "summary": "Require the exact Cargo nextest command including manifest, target, and nonzero guard.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-09",
    "severity": "p2",
    "summary": "Parse and order proof timestamps and require explicit RFC3339 timezone syntax.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  },
  {
    "id": "R96-10",
    "severity": "p1",
    "summary": "Resolve split code authority from the canonical publication repository with conflict rejection.",
    "actionable": true,
    "in_scope": true,
    "disposition": "fixed",
    "fix_revision": "git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656",
    "route": null
  }
]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The live sixteen-child terminal and GitHub integration path remains future proof when the terminal manifest exists; the generated and current historical validator surface is green.

## Review Result

Revision: Some("git-blake3:652cb88d7e196858491e28f57c73bdd36449ebde:060a48a16e83bb4639b396d8bc28a61c41a08c45c94db5191f5e702699ca7656")

Reviewer: Some("subagent:/root/review_5863/review_5866_exact")

Result: pass
