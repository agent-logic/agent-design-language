# AWS Spot Remote Validation Lane

The AWS validation implementation is owned by [agent-logic-infrastructure](https://github.com/agent-logic/agent-logic-infrastructure/tree/61c2806b6274b1003aa42f02adec99faf5f2a74b/tools/aws_remote_validation).
Its [owner runbook](https://github.com/agent-logic/agent-logic-infrastructure/blob/61c2806b6274b1003aa42f02adec99faf5f2a74b/docs/tooling/AWS_SPOT_REMOTE_VALIDATION_LANE.md) defines the owner route.

The former ADL source-build installer, lane entrypoint and reusable workflow
were retired by #1223. The retained ADL entrypoint refuses execution; it does
not build a private crate, select a stale binary, launch AWS or report a test pass.
ADL public CI no longer invokes this reusable workflow.

Original launch, cleanup, authorization, redaction, interruption and retained-volume
assertions remain in predecessor Git and the exact rollback archive. The reviewed
owner package preserves bootstrap tests, including 24 observed passing cases in
retained qualification. The isolated adapter compile/install passed separately.
Neither result proves a new live AWS run or complete owner CLI behavioral parity.
The earlier overall Infrastructure qualification failure remains adverse evidence.

Historical usage examples are available in this document at predecessor
`beea825590544b5412d4dcbbdf599d03837e1148`; they describe the source-era ADL route and are not current commands.
