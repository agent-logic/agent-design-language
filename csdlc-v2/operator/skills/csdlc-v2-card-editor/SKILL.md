---
name: csdlc-v2-card-editor
description: Apply typed semantic card operations through the canonical v2 record.
---
Invoke `csdlc-edit apply`. Never patch rendered Markdown directly; markdown.rs AST validation and atomic regeneration remain binary-owned.

## C-SDLC v3 transition boundary

C-SDLC v3 is construction evidence only until an explicit operator-reviewed
V3-F cutover changes root authority. Continue using this v2 edit route for live
card/state projection changes; do not hand-edit Markdown or route issue-card
mutation through v3 construction code.
