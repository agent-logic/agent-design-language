# Provider and profile hot loading

ADL can run a production provider reload owner against an explicit
provider-only sidecar. The owner reuses the runtime kernel config reload watcher
instead of adding a second watcher or registry.

The sidecar accepts only:

- `schema: adl.provider_reload_sidecar.v1`
- optional `version`
- `providers`

Workflow, task, tool, authority, executable-step, and credential-value surfaces
are intentionally outside the sidecar boundary. Provider credentials must remain
stable references such as environment-variable names or governed provider auth
objects; raw token, secret, API key, or credential values are rejected before a
candidate can become active.

On each accepted edit, the owner validates and materializes a complete candidate
document, including provider profile expansion and last-known-good promotion,
then publishes one immutable provider snapshot. Invalid edits retain the prior
complete snapshot and record only a bounded redacted diagnostic.

The execution runner consults the current provider reload snapshot immediately
before building the provider for a local step. That means:

- a valid edit is available to later inference without restarting the process;
- an in-flight step keeps the provider snapshot it selected before dispatch;
- concurrent readers see either the old complete snapshot or the new complete
  snapshot, never a mixed provider map.

The reload owner is deliberately provider-scoped. It does not reload signing
keys, database pools, model weights, workflow authority, tools, or executable
workflow content.
