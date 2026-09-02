# Structured Review Prompt

Template: 1.0.0

Issue: 596

Repository: agent-logic/agent-design-language

Card: srp

Status: draft

## Scope

.csdlc/prepared/issues/596/correct-sor-v3-separate-output.json
.csdlc/prepared/issues/596/validate-remediation-regression.sh
csdlc-v3/tests/real_issue_canary.rs

## Prompts

- Does #596 have canonical local lifecycle state before PR #615 closes it?
- Does PR #615 visibly close #596 while keeping #505 and #534 as non-closing Part-Of links?
- Does the branch have zero net csdlc-v2 source/test mutation against origin/main?
- Are observed v2 defects captured as v3 replacement requirements rather than patched in v2?
- Does any evidence claim v3 authority before #505?

## Findings

[]

## Dispositions

Every actionable finding requires a terminal disposition.

## Residual Risk

- The review subagent's only remaining blocker was lifecycle-state absence before this record/publication sequence; publication still must be recorded through typed C-SDLC v2 before PR #615 is merge-ready for #596 closeout.
- C-SDLC v3 remains non-authoritative until explicit #505 cutover; #615 only repairs #596 sprint-remediation proof and does not retire v2.

## Review Result

Revision: Some("git-blake3:c2ef9e14fb3e00c7f23395892065df1d4162ed8c:7e88c4789fc7ab1d4960826d1e09f17b88385c7d5cb975641a3e34721e93afbd")

Reviewer: Some("/root/review_615_c2ef9e14")

Result: pass
