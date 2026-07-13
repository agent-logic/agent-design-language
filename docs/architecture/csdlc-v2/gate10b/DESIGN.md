# Gate 10B: pre-switch proof design

`csdlc-proof` refuses dirty trees, reads the tracked Gate 10 generation selector, runs one reviewed executable-plus-argv manifest, and emits atomic exact-revision evidence. It never accepts a shell command string, changes the generation selector, publishes, or deletes files. The required proof set builds all revision-current binaries before measurement and covers the full independent v2 suite, executable samples/parity, warning-free quality, and runnable v1 command surface.

Before and after external proof, the runner verifies v1 paths, resolves omitted generation to v1, explicitly selects v2 for the opted-in issue, and rehearses rollback by resolving the unchanged default back to v1. Any missing step, nonzero exit, unavailable executable, v1-path loss, or selector drift denies pass.
