# Live resident management

Runtime agent operations use authenticated `/v1/agents` APIs. Use the installed
`csmctl agent` commands; no Runtime restart is part of agent management.
A newly built implementation is not installed behavior until separately deployed.

## Add or replace a binding

Save this as an operator-owned YAML file, substituting the existing resident's
identity and the absolute active Runtime init path. Reuse the resident ID and
canonical name to preserve conversation continuity. `agent add` replaces the
binding for an existing identity; it does not rename it.

```yaml
schema: adl.csm.agent_config.v1
runtime:
  init: /absolute/operator/runtime-init.toml
identity:
  id: delta
  name: delta.axioma
  display_name: Delta Axioma
office: frontier reasoning agent
provider:
  kind: openai
  model: gpt-5.4
  credential_ref: keyfile:openai2.key
  required_capabilities: [conversation, agent_to_agent]
```

```sh
csmctl agent add --config /absolute/operator/delta.yaml
csmctl agent get --init /absolute/operator/runtime-init.toml --id delta
csmctl agent list --init /absolute/operator/runtime-init.toml
```

`keyfile:NAME` is a non-secret leaf filename under the Runtime service user's
`$HOME/keys`. OpenAI and Anthropic resolve it on each invocation, so replacing the
key file changes the next invocation without changing process environment.
Anthropic can use `keyfile:claude2.key`. Preserve restrictive operator file
permissions. Unix reads pin the directory and reject symlinks/non-regular files;
unsupported platforms fail closed. Missing, empty, oversized or malformed files
return redacted errors. Key values are never stored in agent declarations.
Existing `env:OPENAI_API_KEY` references remain supported. Other provider types
must not accept a key-file reference and silently fall back to a different key.
An explicit key-file reference takes precedence by excluding environment-based
auth in that binding; ambiguous sidecar auth references are rejected.

## Shepherd lifecycle

The bootstrap primary shepherd has ID `shepherd`. Its first durable roster
initialization seeds it from Runtime init. Subsequent management uses that durable
roster, including absence after removal. Reuse its canonical name and office when
changing its model or endpoint. The primary shepherd and other configured
`resident shepherd` offices remain on loopback Ollama; ordinary residents may
use their supported providers. Changes invalidate old readiness and restart only
the affected resident's recovery task. Unrelated residents and the Runtime process
remain running.

```sh
csmctl agent add --config /absolute/operator/beacon.yaml
csmctl agent remove --init /absolute/operator/runtime-init.toml --id shepherd
```

Removal leaves the shepherd absent until explicitly re-added; bootstrap config
must not resurrect it on a later service start. It does not imply that the flock
is supervised while no shepherd is present. The independent health-response and
SNS escalation work is tracked separately in #1108.

The recovery task refreshes its observation every ten seconds while ready, without
periodic inference. Actual provider failures invalidate readiness and trigger
bounded recovery. Roster freshness is enforced in Observatory as well as agent
lookup. Replacement epochs fence old recovery callbacks and provider inference
observations, including same-model rebinds and removal/re-addition.

## Verification

Admission proves configuration acceptance, not inference readiness. Perform one
bounded real conversation through Runtime, then read `/v1/health/providers` and
the agent detail. Keep provider reachability, model availability and generated
inference observations distinct. Machine-readable command results stay on stdout;
redacted `adl_event` diagnostics remain on stderr. No new compatibility log-file
or OpenTelemetry behavior is claimed.

PVF: required deterministic local runtime regression proof, mock metadata/network
fixtures, disposable key files and virtual clocks; no paid provider calls.
Focused tests cover key rotation/revocation, path confinement, lifecycle epochs,
shepherd replacement/removal/re-addition and restart persistence, plus fresh
supervisor observations without additional inference. Deployment and live proof
are separate from those regression results.
