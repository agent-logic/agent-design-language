# #446 ACC-Governed Resident Tool Execution Design

Long-lived Runtime cycles treat provider tool output as an untrusted UTS proposal. The Runtime binds exactly one proposal to typed resident identity, role, allowed tools, and authority digest; compiles UTS to ACC using production registry/policy inputs; evaluates the Freedom Gate; invokes a sealed governed adapter or denies; and retains a redacted terminal receipt bound to the Runtime cycle and checkpoint lineage.

The `adl` crate owns orchestration to avoid a reverse dependency from `adl-runtime`; `adl-runtime` owns typed resident admission fields. Fixture dispatch remains test-only. The initial production proof uses a harmless read-only Runtime-state adapter. AWS qualification, #269, arbitrary process/network/filesystem authority, and restoration of the retired #5347 demo are excluded.
