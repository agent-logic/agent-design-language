# ADL Owner Binary Installation

ADL operational owner binaries are generated tools. They are not committed to
Git, and they are not owned by Cargo `target/` directories.

The stable local install location is:

```text
.adl/bin/
```

Use:

```sh
bash adl/tools/install_owner_binaries.sh
```

The installer records per-binary provenance under `.adl/bin/.provenance/`.
Re-running it is a no-op when the recorded source hash still matches the current
owner-binary source inputs. This prevents unrelated issue merges or closeouts
from replacing operational binaries.

The source hash includes tracked and untracked non-test owner-binary source
inputs. A feature worktree with newer production Rust inputs must rebuild or use
its own installed binary; primary-checkout `.adl/bin/` binaries are not allowed
to mask newer worktree code.

Cargo `target/` directories remain build/cache output only. They may be deleted
or pruned without taking the operational command surface with them. Wrappers
such as `pr.sh`, `validate_structured_prompt.sh`, and `prompt_template.sh`
prefer `.adl/bin/` before checking `target/debug`.
