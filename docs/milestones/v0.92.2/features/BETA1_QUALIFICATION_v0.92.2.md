# CodeFriend Beta 1 Qualification

Status: planned. Owners: CF-PROOF and CF-INTEGRATE.

Qualification combines documentation, examples, deterministic fixtures, an ADL self-review, and one bounded review of a licensed external open-source repository. The integrated packet maps every Beta 1 exit-bar item to merged implementation and current proof, including explicit failure behavior and residual risks.

Synthetic-only evidence, a zero-test command, or unrelated green CI is non-proving. The proof does not claim customer-scale generality, a defect-free repository, or completion of deferred ATE, Runtime v4, OCI packaging, or modernization work. Bounded MLX/Metal qualification proves only the declared PLAT-MLX surface.

CF-INTEGRATE first connects the complete upstream tasks into the installed product and checks the journey on one exact candidate. CF-PROOF then independently runs a repeatable qualification suite on both repositories, including fresh setup, actual review, second-run comparison and Markdown/HTML/PDF exports. TAIL-01 requires both results. Both require nonzero success and failure scenarios; authoring an evidence packet or handing unfinished functionality to integration cannot close either issue. The [atomic task contracts](../ATOMIC_TASK_CONTRACTS_v0.92.2.md) bind this distinction.
