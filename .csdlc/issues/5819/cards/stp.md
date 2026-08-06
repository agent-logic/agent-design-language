# Structured Task Prompt

Template: 1.0.0

Issue: 5819

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Create and verify exactly five Agent Logic destination copies without changing any of the seven source repositories; retain asksifu and Horust as untouched controls.

## Deliverables

- Organization readiness and exact five-copy/two-control inventory
- Fixed four-private/one-public destination visibility matrix
- Five source-before, destination-after, and source-after manifest sets
- Exact Git/ref/object and Git LFS parity proof
- Actions-disabled-before-push receipts and destination configuration dispositions
- asksifu and Horust negative-control evidence
- #5888 website handoff receipt
- Final copy report with zero unexplained drift

## Acceptance

1. Organization owners, recovery, billing, 2FA, policy, destination names, exact five-copy allowlist, two-control denylist, and four-private/one-public visibility matrix are verified
2. Each approved source has a redacted exact-ref source-before manifest and every non-Git GitHub surface has a truthful recreate, retain-source, omit, or operator-action disposition
3. Exactly five empty agent-logic destinations are created in order, destination Actions are disabled before any mirrored ref arrives, and every source repository remains unchanged
4. Each destination has exact Git ref and object parity, complete Git LFS parity or a proven no-LFS disposition, the required visibility, and reconstructed or dispositioned destination configuration
5. A source-after manifest proves owner, visibility, default branch, refs, HEAD, and settings inventory remain unchanged for every copied source
6. ADL copies last and remains public; its destination Actions stay disabled until environments, rulesets, secrets/variables names, OIDC, packages, Apps, security, and workflow activation are verified
7. Website reference cutover is delegated to #5888 and starts only after the public ADL destination passes verification
8. danielbaustin/asksifu and danielbaustin/Horust remain unchanged and no corresponding agent-logic destination exists
9. Final evidence contains no secret values, no unsupported metadata-copy claims, no unexplained drift, and one exact-revision bounded review

## Dependencies

- WP-01 issue #5817 and corrective WP-01B PR #5887 terminal before the ADL source snapshot
- Copy-only plan and issue design reviewed and current
- Destination organization owner, billing, recovery, 2FA, private-repository, Actions, Pages, package, LFS, and security readiness
- Five free destination names
- Package, GitHub App, and organization Actions-policy unknowns dispositioned before affected destination activation

## Inputs

- .adl/docs/TBD/AGENT_LOGIC_ACCOUNT_REPO_MIGRATION_PLAN.md
- .csdlc/prepared/issues/5819/design.md
- docs/milestones/v0.92/WP_ISSUE_WAVE_v0.92.yaml
- docs/milestones/v0.92/WBS_v0.92.md
- GitHub repository duplication and LFS documentation
- Live seven-source and destination-name inventory
- agent-logic.ai personal-account reference inventory tracked by #5888

## Non Goals

- Transfer, deletion, rename, archival, visibility change, settings mutation, issue movement, or push to any danielbaustin repository
- Copy or mutation of asksifu or Horust
- Claiming Git mirroring copies issues, pull requests, settings, secrets, packages, or integrations
- Website edits owned by #5888
- WP-02A CI redesign
- Export of secret values or credential material
- Downstream milestone implementation
