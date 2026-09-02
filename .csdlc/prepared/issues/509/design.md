# Issue 509 design — DRT-D

This optional sidecar consumes terminal #508, #494, and #495. It uses the
governed GCP company project `cs-poc-cha8mmii0xk0iaw5vpf8mxf` and an
operator-approved local `GOOGLE_APPLICATION_CREDENTIALS` file path to qualify
the same six-resident workload, continuity, cost, and cleanup-zero contract
without replacing or weakening AWS qualification authority.

No launch occurs unless all of these gates are true in the bound implementation
worktree:

- terminal caches for #508, #494, and #495 exist and each merge SHA is
  ancestral to the execution base;
- the active GCP project/billing identity is explicit and provider identity is
  non-ambiguous;
- the operator has authorized the paid DRT-D run with a fixed cost ceiling;
- per-run resource labels/selectors, cleanup trap behavior, and cleanup-zero
  readbacks are retained;
- credential material remains outside the repository and only the approved
  environment-variable contract is recorded.

The implementation packet should reuse the #494 split-resource pattern: stable
support resources are distinct from disposable per-run compute, and proof must
retain both cost-relevant runtime details and zero-residue cleanup readbacks.
