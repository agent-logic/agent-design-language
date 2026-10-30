# Gate 10A: coexistence design

C-SDLC v2 is installed beside v1, never over it. The only accepted stable destination is the dedicated `.adl/bin/csdlc-v2/` generation directory; shared `.adl/bin/` is rejected without mutation. The embedded nine-skill manifest maps operator intent to typed Rust binaries. `csdlc-install` copies only regular executable v2 owners, records BLAKE3 provenance, and verifies both the non-symlink executable v2 set and an explicit v1 availability inventory. The manifest fixes `default_generation` to `v1` for this gate.

Skills are adapters, not state owners. No skill or installer invokes shell/Python lifecycle logic. V2 remains independent of ADL Runtime and incumbent C-SDLC crates. The verifier fails closed if any named v1 path or v2 binary is absent. Gate 10A cannot switch defaults or delete v1.
