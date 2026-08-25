# Longitudinal Review Memory

Status: planned. Owner: CF-MEMORY.

Beta 1 retains enough governed review state to compare a later run with a baseline. It classifies added, resolved, and changed findings using stable identity and an explicit schema-compatibility policy.

Acceptance requires a deterministic two-run fixture, missing-baseline behavior, version-mismatch handling, and collision detection. It does not create unbounded organizational memory.
