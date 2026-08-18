# Issue 287 design: ADR 0071 provider-neutral multi-agent evidence reconciliation

## Boundary

#287 owns only issue-local evidence reconciliation for ADR 0071 under #207. It does not execute providers, read credentials, accept ADR 0071, close #207, serialize shared ADR docs/index/plan/manifest, or claim #288 final ADR serialization.

## Inputs

- Live GitHub issue #287 contract.
- Live GitHub issue #341 WP-18B provider-neutral multi-agent proof umbrella state.
- Repository-local terminal cache path `.git/csdlc-v2/derived-terminal/341.json`.
- Related terminal caches for WP-18B children, where present, recorded only as supporting evidence and never upgraded into terminal #341 umbrella proof.
- #207/#288 issue bodies for parent/final-serialization boundaries.

## Evidence flow

1. Observe #341 live state and whether `.git/csdlc-v2/derived-terminal/341.json` exists.
2. Record the observation in `.csdlc/evidence/287/live-observations.json` with `credentials_read=false` and `provider_execution_run=false`.
3. Record `.csdlc/evidence/287/evidence-manifest.json` with:
   - #341 state and terminal-cache presence.
   - `provider_neutral_multi_agent_proof.terminal=false` when #341 remains open or lacks terminal cache.
   - `provider_neutral_multi_agent_proof.classification=residual_gap` for non-terminal umbrella truth.
   - explicit non-claims for ADR acceptance, provider execution, credentials, #207 closeout, #288 serialization, and WP-18B terminal proof.
4. Record `.csdlc/evidence/287/adr0071-provider-neutral-multi-agent-reconciliation.md` as human-readable evidence for #207/#288.

## Validation

The focused validator `.csdlc/evidence/287/validate_adr0071_provider_neutral_multi_agent_evidence.sh` is deterministic, local, and credential-free. It fails closed unless the retained manifest, observations, report, and actual repository terminal-cache path agree. In the current known state, #341 is open and `.git/csdlc-v2/derived-terminal/341.json` is absent, so the only truthful classification is residual gap.

## Handoff

#287 may publish a residual-gap reconciliation packet when the evidence is exact, validated, and freshly reviewed. #288 consumes the terminal #287 packet later to serialize final ADR index/plan/manifest/review-handoff truth; #287 must not edit those shared ADR surfaces directly.
