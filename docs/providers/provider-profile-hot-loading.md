# Provider and profile hot loading

ADL can run a production provider reload owner against an explicit
provider-only sidecar. The CSM `adl_workflow` production cycle starts the owner
when `workflow.run_args.provider_reload_sidecar_path` is set. Relative sidecar
paths resolve against the ADL workflow file's parent directory. The owner reuses
the runtime kernel config reload watcher instead of adding a second watcher or
registry.

The sidecar accepts only:

- `schema: adl.provider_reload_sidecar.v1`
- optional `version`
- `providers`

Workflow, task, tool, authority, executable-step, and credential-value surfaces
are intentionally outside the sidecar boundary. Provider credentials must remain
stable references such as environment-variable names or governed provider auth
objects. Raw token, secret, password, API key, client-secret, private-key,
bearer-token, or credential-shaped values are rejected before a candidate can
become active, including suspicious values under neutral containers such as
`auth.value`.

On each accepted edit, the owner validates and materializes a complete candidate
document, including provider profile expansion and last-known-good promotion,
then publishes one immutable provider snapshot with a provider-level generation.
Invalid edits retain the prior complete snapshot and record only a bounded
redacted diagnostic tied to the current provider-level generation.

The execution runner consults the current provider reload snapshot immediately
before building the provider for a local step. That means:

- a valid edit is available to later inference without restarting the process;
- an in-flight step keeps the provider snapshot it selected before dispatch;
- concurrent readers see either the old complete snapshot or the new complete
  snapshot, never a mixed provider map.

The reload owner is deliberately provider-scoped. It does not reload signing
keys, database pools, model weights, workflow authority, tools, or executable
workflow content.
