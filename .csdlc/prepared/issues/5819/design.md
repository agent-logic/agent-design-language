# WP-02 Agent Logic Repository Copy Design

## Outcome And Authority

Issue #5819 creates five independent company-owned copies under `agent-logic`
without transferring or changing any source repository under `danielbaustin`.

| Order | Read-only source | Destination | Visibility |
| --- | --- | --- | --- |
| 1 | `cognitive-sdlc-paper` | `agent-logic/cognitive-sdlc-paper` | Private |
| 2 | `godel-hadamard-bayes-paper` | `agent-logic/godel-hadamard-bayes-paper` | Private |
| 3 | `general-intelligence-paper-private` | `agent-logic/general-intelligence-paper-private` | Private |
| 4 | `universal-tool-schema` | `agent-logic/universal-tool-schema` | Private |
| 5 | `agent-design-language` | `agent-logic/agent-design-language` | Public |

`asksifu` and `Horust` are untouched negative controls and receive no
destination. The operator authorizes each destination-creation window. WP-02
does not authorize source transfer, push, deletion, rename, archival,
visibility change, settings mutation, or issue movement.

## Preconditions

Execution is blocked until:

- corrective WP-01B PR #5887 is merged and ADL's source snapshot is recaptured;
- the copy-only plan and issue cards are reviewed and current;
- organization owner, recovery, billing, 2FA, private-repository, Actions,
  Pages, package, LFS, and security readiness are confirmed;
- all five destination names remain free;
- package, GitHub App, and organization Actions-policy unknowns have an
  operator-approved disposition; and
- the operator starts the named repository window.

## Copy Protocol

For each repository, serially:

1. Capture a redacted source-before manifest at exact refs and HEAD.
2. Classify Git data, LFS data, and every non-Git GitHub surface separately.
3. Create an empty destination with the exact required visibility.
4. Disable destination Actions and verify it is disabled before any ref push.
5. Create a local mirror from the read-only source, fetch all LFS objects when
   present, verify the push URL names only the expected `agent-logic`
   destination, and copy branches, tags, supported notes, and LFS objects.
   GitHub-owned `refs/pull/*` remain source-authoritative and are not part of
   destination ref parity. Push with explicit branch, tag, and note refspecs;
   do not use `git push --mirror`.
6. Reconstruct or truthfully disposition destination-only settings,
   collaborators, rulesets, environments, packages, Pages, OIDC, webhooks,
   Apps, variables, secret names/scopes, security, releases, and consumers.
7. Keep Actions disabled for a cold mirror. For an active destination,
   re-enable it only after all required destination configuration is present.
8. Verify destination Git/ref/LFS parity and live configuration.
9. Capture a source-after manifest and prove the source remained unchanged.
10. Stop on any unexplained drift before starting the next destination.

Git mirroring copies commits and refs. LFS requires separate fetch/push.
Ordinary duplication does not copy issues, pull requests, discussions,
settings, secrets, or integrations. Those surfaces must never receive a false
copy claim. Open source issues and PRs remain source-authoritative unless a
separately reviewed destination reconstruction preserves attribution and links
without modifying the source.

## ADL And Website Boundary

ADL copies last and remains public at both source and destination throughout
WP-02. Its destination Actions remain disabled until its environment, ruleset,
three secret names, four variable names, OIDC/AWS coupling, packages, Apps,
releases, badges, Codecov coordinates, and source-oriented defaults are
explicitly verified or dispositioned.

The website reference cutover is owned by sidecar issue #5888 in
`agent-logic/agent-logic.ai`. It starts only after the public ADL destination
passes verification. WP-02 records the dependency and receipt but does not edit
that separate repository.

A later issue may archive the public personal ADL source and add a prominent
migration notice. That is outside WP-02. GitHub provides no native redirect
between two independently retained repositories.

## Invariants And Negative Controls

- All seven source repositories remain under `danielbaustin` and unchanged.
- Only the five named `agent-logic` destinations may be created or configured.
- Four destinations are private; ADL is public.
- Destination Actions are disabled before mirror push.
- Secret values never enter logs, chat, plans, or evidence.
- `/private/tmp` is never used.
- `asksifu` and `Horust` receive no copy or settings mutation.
- Their before snapshots precede the first copy window and their after
  snapshots follow the fifth copy window.
- WP-02A CI redesign and downstream implementation remain separate.

## Recovery

Stop at the active destination boundary. Preserve every source unchanged.
Repair, quarantine, or, with explicit operator approval, delete only the
incomplete destination. There is no transfer-back path because ownership never
moves. Rerun destination proof and source-immutability proof before resuming.

## Proof Design

Retain for each copy:

- source-before, destination-after, and source-after manifests;
- exact refs, HEAD, object, default-branch, visibility, and LFS proof;
- per-surface reconstruction or source-authoritative dispositions;
- destination creation time, Actions-disabled receipt, and the actual first
  mirror-push timestamp plus transcript proving disablement preceded ref arrival;
- source immutability comparison;
- complete GitHub-surface dispositions, including package/App/Actions-policy
  evidence and names-only secret/variable records without values; and
- exact-review revision and residual risks.

Each of the 37 named platform surfaces requires a digest-bound `live_api`,
`operator_confirmation`, or valid `not_applicable` proof. The per-repository
operator comment binds the Actions-disabled receipt, first-push receipt, LFS
receipt, platform packet, and source-after manifest; the organization comment
binds owner, billing, recovery, Actions, package, and GitHub App readiness.

The issue validator must fail on a missing or reordered repository, incorrect
visibility, missing source-after proof, source drift, Actions disabled only
after the first push, missing Git/LFS parity, unsupported metadata-copy claims,
secret-like content in any referenced artifact,
value, absent negative control, or unexplained destination drift.

## Owned Paths

- `.adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md`
- `.adl/docs/TBD/V092_SPRINT_5858_FOUNDATION_SESSION_PROMPT.md`
- `.csdlc/evidence/5819`
- `.csdlc/issues/5819`
- `.csdlc/prepared/issues/5819`
- `docs/milestones/v0.92/FEATURE_PROOF_COVERAGE_v0.92.md`
- `docs/milestones/v0.92/QUALITY_GATE_v0.92.md`
- `docs/milestones/v0.92/README.md`
- `docs/milestones/v0.92/SPRINT_v0.92.md`
- `docs/milestones/v0.92/WBS_v0.92.md`
- `docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml`
- `docs/milestones/v0.92/features/README.md`

## Read-Only Inputs

- All seven `danielbaustin` source repositories and settings
- Existing `agent-logic` repositories outside the five named destinations
- Historical #5815/#5816 transfer review, retained as superseded evidence
- Secret values and credential material
