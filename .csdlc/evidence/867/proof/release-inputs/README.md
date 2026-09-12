# Historical release metadata fixture correction

PR #948 CI run 34666577805 at de07d3cb1158f06bc6ca60121ae8feb96a8f4cd8
rejected an undeclared local package inherited through the current shared lock.
The earlier path-only fixture isolation did not isolate lock contents or manifest
versions. That earlier supplement remains historical evidence of a partial fix.

The test now synthesizes package and workspace manifests from the unchanged
historical inventory, including inherited workspace versions and member paths.
Each active lock includes every declared owner at its declared version and a
synthetic registry dependency. Later package paths, manifest/workspace versions,
and local shared-lock records are injected before normalization on every host.
After the healthy fixture is established, separate mutations require denial of
an unlisted manifest and an unlisted local lock package. Existing owner, version,
source, authority, candidate, linked-worktree and nonmutation cases remain.

These are release metadata fixtures, not Cargo resolution/build proof or approval
of the current repository as a historical release candidate. Production release
guards and the real historical inventory are unchanged. Current production binary
and authority proof remain live inputs. Historical lock files retain their original
contents; their declared historical disposition is checked by the production guard.

The focused matrix passed (1 test), as did strict all-target Clippy, formatting and
diff checks. The previous 248-test run and 17-attempt corpus remain source-bound
historical proof in the curl-config supplement; neither was rerun or relabeled.
The index binds source files and normalized logs, including a selected CI failure
excerpt with its full original hash. Independent review and fresh current-head CI
remain required. No publication, merge, closure or live activation is claimed.
