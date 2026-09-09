# Issue 762 review remediation

Fresh reviewer `codex:/root/review_762` requested changes at
`f6edd6f942922a629f629da4430557552668d662`.

R762-001 (P1): initial authorization checked the install receipt directory but
not its `receipt.json` leaf. The reviewer reproduced a symlink at that leaf
causing a blocked report only after the binary had already been replaced.
The report therefore incorrectly claimed no mutation.

The repair preflights the exact install receipt and binary output paths as
regular-or-absent endpoints before installation. It applies the same endpoint
rule to proof receipts and rechecks it before durable writes. The topology
test now snapshots an existing installed binary and all issue mutation
surfaces around rejected symlink and directory receipt targets. It also tests
a directory at a proof receipt filename.

The focused topology test passed after the repair. Fresh typed execution logs
are retained under `repair/`; a new exact-head independent review is required
before publication. The historical review is evidence of the detected defect,
not approval of the repaired candidate.
