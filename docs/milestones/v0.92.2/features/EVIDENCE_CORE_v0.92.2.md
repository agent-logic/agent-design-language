# Evidence Core

Status: planned. Owner: CF-EVIDENCE.

The evidence core assigns stable artifact identity, binds provenance, applies redaction before retention or model use, and enforces an explicit retention policy. Review findings cite evidence objects rather than mutable display text.

Acceptance requires deterministic identity, tamper detection, negative secret/redaction fixtures, retention and deletion behavior, and explicit unsupported-version states. Public evidence hosting is not included.

CF-EVIDENCE also owns the shared finding/run schema and versioned fixtures consumed by review, memory and renderers. It must define finding identity separately from evidence identity, completeness and unknown states, scope/version compatibility, provenance and digest binding before consumers execute.

Completion requires the adapter to invoke the production admission/store boundary and demonstrate admission plus rejection, redaction before retention/model use, and actual retention/deletion enforcement. A published schema or unused evidence library alone is insufficient. See the [atomic task contract](../ATOMIC_TASK_CONTRACTS_v0.92.2.md).
