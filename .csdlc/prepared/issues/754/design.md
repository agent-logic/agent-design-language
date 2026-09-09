# Registry compatibility repair #754

Allow only coherent registry 1.0.3 and 1.0.4; top-level version, semver, legacy version and path agree. Preserve native compact 1.0.0 identity and shape validation. Reject before sample writes.

Scope: csdlc-v2/src/registry.rs and csdlc-v2/tests/gate9.rs. Reviewed by parent agent on commit 516aa3e6db378c1fe9fe490da64492d709c8c620.

Native issue and bind succeeded; known #749 leaves native state in primary. Preserve that state and use explicitly authorized typed-v2 transition bootstrap/adopt in bound worktree with repaired registry guard, no bypass.
