# Machine-local worktree default: removal handoff

## Operator instruction

Daniel explicitly said: “remove the policy altogether”, then clarified: “the rule for this machine is that we want to put worktrees on fastwork, but it doesn't require a policy that can break”. FastWork is a preferred destination on this machine, not a prerequisite checked by lifecycle authority. Do not replace the removed files with another mandatory path-policy file.

## Assigned correction

The operator assigned this correction to [C-SDLC #304](https://github.com/agent-logic/cognitive-sdlc/issues/304). Its issue scope now includes removal of location-policy enforcement alongside the existing remount-identity fix; the exact remote body was verified after the native update. Keep this handoff here. WorkerBee #18 remains focused on #299 and has not been messaged or assigned this change. Implementation and installed qualification remain outstanding.

## Requested owner change

Remove mandatory worktree-location policy admission from ordinary prepare/start/bind. Allow the caller to select an exact destination; use a machine-local default when omitted. On this host choose `/Volumes/FastWork/codefriend-worktrees/` and `/Volumes/FastWork/agent-logic-enterprise-security-worktrees/` for these two prepared issues. Location preference must not require a tracked policy, authenticated policy digest, fixed parent or symlink-free policy file. Keep ordinary Git safeguards against overwriting an occupied checkout, conflicting issue/branch ownership and destructive operations; these concern actual effects rather than administrative placement preferences.

Preserve the existing unbound native records for CodeFriend #120 and enterprise-security #9. Reconcile their selected destination through the owner instead of hand-editing local index/state or creating duplicate issue identities. The new route must allow the reviewed child card amendments while preparing the sprint; no product execution is requested.

## Current evidence

The two primary repository `.adl/worktree-policy.json` files were removed locally at Daniel's explicit instruction. Exact backups and removal receipt are under `/Volumes/FastWork/v0932-sprint2-preparation/policy-removal/`. Deletions are uncommitted; preserve them, do not restore them silently. Native status remains readable after deletion.

The admitted owner used for preparation is `/Volumes/FastWork/c1223/pr279-bootstrap/cognitive-sdlc/.adl/packages/11873730137/installed/bin/csdlc`. Its bundled fallback still declares `primary_sibling: cognitive-sdlc-worktrees`. Deleting repository files therefore does not remove enforcement.

Inspected implementation references (source explanation, not installed qualification):

- `csdlc-v3/src/authority/installed_tool.rs`: reads policy from the primary checkout and installs it as an external resource; absent file falls back to package resources.
- `csdlc-v3/src/application/intent/local.rs`: prepare/bind derive destination via `worktree_policy::parent`; ordinary semantic edit requires bound context.
- `csdlc-v3/src/worktree_policy.rs`: enforces policy schema, parent and alias conditions.
- #299 candidate `csdlc-v3/src/application/intent/start_delivery.rs`: start composes prepare/bind and retains their destination policy; it does not yet offer an independent explicit path.

WorkerBee #18 currently owns #299; do not modify that worker's checkout or shared binaries. This handoff is ready for coordination, not an instruction to silently add a late change to its reviewed PR.

## Focused acceptance for the owner

1. Missing or stale worktree policy does not prevent ordinary preparation or explicit-path binding.
2. The caller can select a free FastWork destination; the same issue can resume there without creating another issue or competing checkout.
3. Existing internal-path preparation records can be reconciled to the selected destination through the native owner, preserving source, review and outcome truth.
4. Existing occupied/conflicting paths fail with a concrete Git conflict diagnosis, not a preference-policy refusal.
5. After the owner change is delivered and admitted, the Sprint 2 preparer applies and validates the retained child plans; execution dependencies remain separate.

PVF: focused deterministic CLI/Git fixtures with temporary repositories and no paid provider/cloud calls. Verify missing/stale policy and explicit destination behavior, plus occupied-target and duplicate-binding adverse cases. Do not run broad product builds or replace a shared installed binary as part of this handoff.

## Sprint state

Planning #12 conditionally accepted the preparation packet; the umbrella input/plan/validation cards are corrected and validated. Child native card amendments and validator registration remain incomplete. The execution order remains accepted #299 delivery, CodeFriend #120 assessment, then enterprise-security #9. No product worker is dispatched by this handoff.
