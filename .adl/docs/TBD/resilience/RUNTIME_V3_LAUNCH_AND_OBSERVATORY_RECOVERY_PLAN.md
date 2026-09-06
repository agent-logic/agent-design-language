# Runtime v3 stable CSM route

Issue #678 makes the stable operator command `.adl/bin/csm` a launcher for the
active Runtime v3 generation. The launcher resolves from its own location to
`.adl/runtime-v3/current/bin/csm` and then `exec`s that generation-owned CSM
with the original arguments.

The stable path is therefore not a separately copied Runtime-control binary.
Installing a new generation and rolling back both switch the same
`.adl/runtime-v3/current` symlink that owns CSM, Guardian, and kernel artifacts.
If the active generation is missing or incomplete, the stable launcher fails
before it can dispatch any Runtime service-control command.

Local validation for this issue uses isolated fixture generations under
`.csdlc/evidence/678`. It does not restart, reload, stop, or otherwise mutate a
live Runtime service.
