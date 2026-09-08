# PR #669 CI refresh evidence

Timestamp: 2026-09-03

PR #669 and stale superseded PR #644 both pointed at commit
`d230bcc30ec8777dc065a783a178cc1daae009f5`. GitHub therefore reported the
failed hosted coverage contexts from the old `pull/644/merge` run on the shared
commit, even though #669 is the governed `main`-base publication for #631.

This file is a small #631-owned evidence-only change to produce a fresh #669
head and force a clean current-topology CI readback without touching v2 source
or changing the v3 proof/parity/install implementation.
