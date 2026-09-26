# RD12: accepted repository bootstrap policy

Issue #1183 applies the operator approval relayed by Planning #11 to the RD02
contract merged in PR #1201 at `10ae895ed7c3da35f4e1d59fbcdcc463a7700560`.
The exact contract SHA256 is
`0e44e0fa7934cda14850389c38942177d56f4ba9954a4102e913e9d0258115b3`.
`operator-approval.json` retains the four decisions, user response, context and
source task. This is documentary approval evidence, not a native receipt or a
cryptographic authentication of chat. The original RD02 proposal and its
pending-policy validator remain unchanged as historical preparation evidence.

The approved destinations are five private repositories under `agent-logic`:
`cognitive-sdlc`, `agent-logic-runtime`, `codefriend`,
`agent-logic-infrastructure`, and `agent-logic-enterprise-security`.
Inherited organization-member read applies to all five, including enterprise
security. Daniel (`danielbaustin`) is the access, original-evidence and rollback
approver. Private GitHub Releases is the approved delivery service, with
producer write and consumer read. Preserve existing licenses and original
history; no license template or production source is installed by bootstrap.
Existing public ADL and private codefriend.ai remain unchanged.

## Bootstrap disposition

Only identity/README initialization and repository-local administration are
in scope. GitHub's existing repository administration API owns these uncovered
operations; native C-SDLC owns #1183 cards, review, publication and closeout.
No generic repository-administration tool is introduced.

Default branch is an actual `main` ref. Protection requires PRs, prevents
non-fast-forward updates and deletion, and has no bypass actors. The existing
ADL baseline of zero GitHub-required approvals is preserved; independent review
is a separate native lifecycle requirement. Workflow tokens default to read and
cannot approve pull requests. Auto-merge and automatic branch deletion remain
disabled; merge commits and issues remain enabled.

New repositories have no product workflows. No fictional `adl-ci` or
`adl-coverage` status is required. Product required checks must be installed and
observed with each destination's later source/workflow import before product
cutover; bootstrap does not claim product-CI readiness. No paid runners,
security products, deployments, workflow dispatches or release assets are
created. Existing source history must integrate normally with the bootstrap
commit; force replacement is not permitted.

Daniel's effective admin/write access and inherited member read establish the
initial access baseline. Service installations and producer/consumer credentials
must be separately granted and tested when those concrete principals and
artifacts exist. No authenticated private release download is claimed from an
empty release catalog. The accepted distribution policy is not installed-product
qualification.

## Retained evidence and scope

The bootstrap readback packet records each exact repository ID, private flag,
main commit, owner permission, effective main rules and workflow configuration.
It also records the authenticated observation time and raw response hashes.
The three required negative cases use altered copies of this captured evidence:
wrong visibility, unapproved identity and missing owner access. No live access is
removed and no repository is made public to exercise a negative case.

No source extraction, issue transfer, org-wide policy change, shared binary
replacement, release or spending is authorized by this packet. RD13 consumes
accepted bootstrap identities after #1183's reviewed outcome. It does not require
destination native lifecycle initialization: independent portable installation
and lifecycle qualification belong to RD04. No ADL authority receipts, source
journals or merge ancestry were copied into the destinations. Rollback here means halt
and preserve these private repositories/evidence for Daniel's disposition, not
delete them or rewrite their initial history.

## Validation result

The focused offline test target `rd12_1183_bootstrap` passed four tests: one
positive case covering all five actual captured repositories, and the three
required negative families each applied to all five records (15 rejected
alterations). These are evidence-contract checks, not 20 live experiments.
Planning #11 independently compared live metadata/access/main refs: 45/45
assertions passed, retained in `independent-readback.json`. Independent
exact-head review remains a separate publication prerequisite.

One workflow-permission PUT encountered a TLS handshake timeout. Authenticated
GET reconciliation established the observed value before any conditional retry;
all five final readbacks have read-only tokens and no workflow approval power.
No repository-create response was uncertain, and no duplicate was created.
