# Issue #521: independent external review

## Goal

Retain one independent external review of the unchanged candidate and #520
packet, preserving reviewer identity, limitations, raw findings, and exact
revision.

The reviewed denominator is derived from the digest-verified #520 repository,
issue/PR, and acceptance inventories. `scope.json` may project that denominator
for readability, but it cannot define or narrow it. Those artifacts are read
from the exact merged #520 commit after live issue/PR verification; synthetic
local files are not predecessor authority. A passing exact-candidate #520
semantic-validation receipt is mandatory.

## Boundary

#521 reviews; it does not remediate findings or approve release.
