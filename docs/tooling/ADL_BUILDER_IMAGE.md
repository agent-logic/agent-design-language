# ADL builder image owner

Builder images and AWS Spot execution are owned by
[agent-logic-infrastructure](https://github.com/agent-logic/agent-logic-infrastructure/blob/61c2806b6274b1003aa42f02adec99faf5f2a74b/docs/tooling/ADL_BUILDER_IMAGE.md) at reviewed revision `61c2806b6274b1003aa42f02adec99faf5f2a74b`.
Use that owner's reviewed instructions and explicit execution authorization.
The ADL source-tree setup, publication, and import commands have been retired.

The original ADL guide and its command recipes remain in Git history and the
#1223 builder-family rollback archive. Historical instance/image defaults are
not a current deployment or approved-spend claim.

The public Group-D fixture adapter and its legacy setup/import lane were retired
by #1351. Builder validation belongs to Infrastructure; use the reviewed owner
instructions linked above. Original adapter bytes and resource identities remain
in Git custody (`docs/language-cleanup/utility-dispositions.json`). Removing the
ADL adapter neither reruns nor completes owner qualification.

Four original offline qualification commands passed: four Python test methods
plus three shell contracts. The setup test uses fake Docker; publish/import
checks are static. No actual image build, image publication, AWS operation,
provider call, or installed-image acceptance was performed.
