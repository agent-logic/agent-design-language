# Issue 5780 deletion metrics

Baseline: `a5a16509af05af441369e78eeacfd74017bcf63b`.

| Surface | Before | After | Net deleted | Reduction |
| --- | ---: | ---: | ---: | ---: |
| Production Rust | 28,722 | 18,585 | 10,137 | 35.29% |
| Test Rust | 15,092 | 9,245 | 5,847 | 38.74% |
| Combined Rust | 43,814 | 27,830 | 15,984 | 36.48% |
| Source binaries | 23 | 22 | 1 | 4.35% |

Across every changed tracked file, including lifecycle cards and proof logs,
the implementation adds 4,366 lines and deletes 18,099, for a net deletion of
13,733 lines.

The removed surface includes the `csdlc-closeout` binary and skill, terminal
repair and reconciliation request schemas, readiness and merged-publication
writers, legacy receipt writers, and their writer-specific tests. The retained
compatibility boundary is read-only: all 114 tracked v0.91.8 terminal records
pass the census, historical terminal phases and receipts remain deserializable,
and the 314 existing Git-common receipt files are left immutable. New terminal
authority comes only from the minimal derived envelope written by
`csdlc-finish`.
