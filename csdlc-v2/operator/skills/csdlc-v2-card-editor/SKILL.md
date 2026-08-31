---
name: csdlc-v2-card-editor
description: Apply typed semantic card operations through the canonical v2 record.
---
C-SDLC v2 remains the live lifecycle authority only until explicit V3-F/#505
cutover. Before that cutover, `csdlc-v3/**` is non-authoritative construction
evidence and must not mutate lifecycle state.

Invoke `csdlc-edit apply`. Never patch rendered Markdown directly; markdown.rs AST validation and atomic regeneration remain binary-owned.
