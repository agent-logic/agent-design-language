# Agent executable coverage and website deadline repair

PR1066 at696a654dc3 failed the unchanged80% executable line-coverage gate:
codefriend_agent.rs had36/95 covered lines. This repair adds a private CLI
transport seam and deterministic command-driver coverage for private pairing,
once, unpair, forget and argument/private-file denials. The production adapter
continues using the existing validated transport. Network protocol proof remains
in the separate integration tests; no fixture-only installed-product claim.

The website cross-component review also found report retention could exceed a
shorter command deadline. The agent now takes the minimum of command, consent,
pairing and local retention deadlines. A four-lane wire regression asserts the
actual serialized report expiry exactly matches the shorter website deadline.

Validation:20 agent integration tests,14 review integration tests and1 CLI unit
test passed under LLVM instrumentation. Binary line coverage184/203(90.64%),
agent543/611(88.87%), runner417/445(93.71%). Changed-source preflight passed at the
unchanged80% threshold; focused clippy passed. Coverage mappings include both
agent integration and CLI binary tests. Parent1063 repair8ae29a749b is incorporated.

Independent working-tree review reported no blocking findings. Exact-head native
proof/review and publication reconciliation follow the committed candidate.
No live-provider, hosted/installed platform, website or Sprint10 acceptance claim.
