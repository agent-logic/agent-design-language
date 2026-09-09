# Issue 771 independent review blocker

At source fba2bdf5c7168faa9dcd2d957c121e33a63c0fd6, native terminal finish writes the terminal state before comparing the immutable retained terminal receipt. A first successful finish for PR641 followed by an authenticated finish for PR642 at the same source SHA, with the correct expected state digest, returns terminal_receipt_conflict but leaves state naming PR642 and receipt naming PR641.

Independent reviewer reproduced this through a temporary Rust harness linked to the exact detached-suite csdlc_v3 library. See terminal-conflict-repro.rs and terminal-conflict-repro.log. The regression failed as expected, exit101.

Proposed bounded repair: compare the prospective receipt with any retained receipt before replacing terminal state; add a focused regression asserting the rejected second finish preserves both files. No source repair has been applied. Issue771 requires separate approval for additional product changes.

Known schema-path defect #776 remains in this reviewed source; green reviewed PR790 is available as a dependency repair. A new frozen combined source requires fresh independent review and full detached locked validation before all four V3-F rows can be resolved.
