# Issue 5802 design

Status: operator approved.

Use the existing native recursive mirror implementation as the production path. Bind one approved external Google credential source only for the live command, keep repository content canonical, create or update Drive files without deletion, and accept deployment only after the generated full-tree report and independent Drive readback agree.

The implementation boundary is operational first. Change tracked mirror code only if the current binary cannot complete the declared full-tree contract with valid credentials. Keep the scheduled automation paused until the complete recursive proof passes.
