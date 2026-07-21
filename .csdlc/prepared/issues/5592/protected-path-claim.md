# #5592 Protected-Path Claim

## Active Preparation Claim

The typed bootstrap claim protects only:

1. `.csdlc/issues/5592`
2. `.csdlc/locks/5592.lock`
3. `.csdlc/prepared/issues/5592`

These paths are issue-specific and disjoint from #5591, #5589, #5590, and all
product implementation paths. The claim purpose is preparation only.

## Deferred Product Claim

No product path is claimed by this revision. #5591 currently protects
`adl-runtime-kernel`, `adl-runtime`, `infra/runtime-v3`, and its guardian-soak
tooling. A #5592 product-path claim would therefore be overlapping and false.

After #5591 has a clean reviewed Parity-A contract, the implementation actor
must inspect the exact claim ledger and reviewed module boundary. It may then
use typed `csdlc-bind` to add only collision-free Parity-B modules, focused
tests, and `.csdlc/evidence/5592`. Broad directory ownership, Runtime v2 paths,
or any collision remains a stop condition.
