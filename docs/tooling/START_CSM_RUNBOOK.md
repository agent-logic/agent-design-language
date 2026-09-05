# Runtime v3 service-control runbook

## Purpose and authority

This runbook is for operators starting, stopping, inspecting, or reloading the
permanent Runtime v3 service on a host. The current-generation Rust command is
the sole Runtime service-control authority:

```text
.adl/runtime-v3/current/bin/csm runtime-v3
```

It owns init validation, launchd or systemd service control, Guardian process
ownership, listener convergence, readiness identity, and transactional reload.
The stable `current/bin/csm` path follows the installed Runtime generation; a
Cargo build-output binary or a binary from another worktree does not.

The root `CSMctl` shell does **not** control Runtime. It remains only for the
separate local Observatory static server. The retired shell-controller paths
`.adl/runtime-v3-service/` and `com.agentlogic.start-csm` are not evidence of
the permanent Runtime's state.

## Operator contract

- Run commands from the primary repository root containing the live install.
- Use the absolute active-init path. `csm runtime-v3` rejects a relative path.
- Run `status` before any state-changing command.
- Treat canonical JSON output and the command exit status together. A JSON
  status document followed by a nonzero exit remains a failed operation.
- Never infer ownership from an open port or an HTTP response alone.
- Never force-kill, manually bootstrap, or replace a service that canonical
  status cannot prove it owns.
- Never print credentials, copy them into command arguments, or collect them
  in an incident packet.

These examples define paths once to reduce transcription errors:

```sh
REPO_ROOT="$PWD"
CSM="$REPO_ROOT/.adl/runtime-v3/current/bin/csm"
RUNTIME_INIT="$REPO_ROOT/.adl/runtime-v3/live/runtime-init.toml"
```

Before proceeding, confirm that both files exist and that the init is the one
intended for this host:

```sh
test -x "$CSM"
test -f "$RUNTIME_INIT"
```

## Canonical identity

The normal Wuji installation has this identity:

| Surface | Canonical value |
| --- | --- |
| Service command | `.adl/runtime-v3/current/bin/csm runtime-v3` |
| Active init | `.adl/runtime-v3/live/runtime-init.toml` |
| macOS launchd label | `com.agentlogic.adl-runtime-v3` |
| Local API | `https://127.0.0.1:20997` |

Do not pin automation to a transient process ID, resolved IP address, Cargo
target directory, or issue worktree. Process IDs legitimately change across a
restart; the service label, installed-generation path, and active-init identity
are the durable control surfaces.

## First response: establish state

Always begin with the read-only canonical status command:

```sh
"$CSM" runtime-v3 status --init "$RUNTIME_INIT" --json
```

A healthy managed service exits zero and reports all of the following:

- `service_manager` is `launchd` on macOS or `systemd` on Linux;
- `label` is the configured service label;
- `config_valid` is `true`;
- `service_loaded` and `listener_ready` are both `true`;
- `guardian_process_id` and `runtime_process_id` are nonzero;
- `active_init_hash` is present and matches the active init; and
- `observability_ready` is `true`.

The command verifies that the service-manager process is the Guardian reported
by `/v1/ready` and that the responding Runtime reports the active init's hash.
An unrelated Runtime on the configured listener therefore cannot satisfy
canonical readiness.

### State interpretation

| Observed state | Meaning | Operator action |
| --- | --- | --- |
| Loaded, listener ready, identities agree | Healthy permanent service | No mutation; record status if needed. |
| Loaded, listener not ready | Managed service failed to converge or identity proof failed | Inspect the exact service and its configured logs; do not start a second Runtime. |
| Not loaded, listener not ready | Service is stopped or not installed | After verifying paths and config, use canonical `start`. |
| Not loaded, listener responds | Unowned or conflicting listener | Stop. Identify the listener without taking it over or killing it. |
| Incomplete reload reported | Transaction artifacts require reconciliation | Run canonical `start`; it owns commit-or-rollback recovery. |
| Config, generation, plist, or unit validation fails | Control inputs are invalid or inconsistent | Correct the named input; do not bypass preflight. |

## Normal operations

### Start or converge

```sh
"$CSM" runtime-v3 start --init "$RUNTIME_INIT" --json
```

`start` validates the full init and installed generation before mutation. It
reconciles an interrupted reload, then either reports an already owned and ready
service or cleanly starts the configured service and waits for owned readiness.
It is the normal recovery command after an interrupted reload.

Use `--plist <absolute-plist>` or `--label <label>` only for an intentionally
configured non-default service. Do not use those flags to work around a
canonical service that fails validation.

### Stop gracefully

```sh
"$CSM" runtime-v3 stop --init "$RUNTIME_INIT" --json
```

`stop` acts through the configured service manager and waits for the Guardian,
Runtime listener, and managed service to stop. The Guardian coordinates Runtime
shutdown and continuity handling. A stopped result should report
`service_loaded: false` and `listener_ready: false`.

### Reload configuration transactionally

Do not edit the active init in place. Create and review a separate candidate,
then provide its absolute path:

```sh
RUNTIME_CANDIDATE="/absolute/path/to/runtime-init.candidate.toml"
"$CSM" runtime-v3 reload \
  --init "$RUNTIME_INIT" \
  --candidate "$RUNTIME_CANDIDATE" \
  --json
```

Reload validates the active and candidate configurations before service
mutation. It stops the current service, preserves the last-known-good init,
starts the candidate through the service manager, and commits only after owned
readiness succeeds. An ordinary candidate failure stops the candidate, restores
the last-known-good init, and starts it again. A later `start` reconciles durable
transaction artifacts left by interruption.

After any start, stop, or reload, rerun canonical `status --json` and compare the
result with the intended state. For reload, also confirm that `active_init_hash`
changed to the candidate's identity.

## Diagnosis and recovery

1. Capture canonical `status --json` and its exit status.
2. Classify the result with the state table above.
3. Inspect only the configured service-manager unit.

   macOS:

   ```sh
   launchctl print "gui/$(id -u)/com.agentlogic.adl-runtime-v3"
   ```

   Linux:

   ```sh
   systemctl status com.agentlogic.adl-runtime-v3.service
   ```

4. Compare the service-manager process identity, Guardian PID, Runtime PID,
   active-init hash, and listener state. A PID by itself is not authority.
5. Correct a concrete path, config, generation, plist, or unit defect before
   retrying. Do not repeatedly restart an unexplained failure.
6. Use canonical `start` to reconcile an interrupted transaction or start a
   verified stopped service.
7. Escalate if ownership remains ambiguous. Preserve evidence and leave the
   competing process untouched.

Do not substitute a direct HTTPS probe with disabled certificate verification.
Canonical status uses the init-declared listener and trust roots and also proves
service-manager ownership. Public DNS, Caddy, AWS edge routing, browser trust,
provider inference, and model health are separate surfaces and require their
own authenticated checks.

### Incident evidence

Record the minimum useful, non-secret evidence:

- UTC timestamp and host identity;
- invoked operation and command exit status;
- canonical status JSON;
- configured service label and active-init path;
- exact installed `csm` provenance if available;
- exact service-manager status for the configured unit; and
- the first concrete error, not only later retry failures.

Do not attach credential files, environment dumps, private keys, tokens, or an
unfiltered process listing. Do not mutate the service merely to gather evidence.

## Persistence expectations

The installed launchd service uses `KeepAlive` and `RunAtLoad`; the equivalent
Linux installation is managed by systemd. Persistence is proven by canonical
status reporting both the loaded service and matching owned readiness. An HTTP
200 response alone does not prove that the permanent service is installed or
that it will survive logout, reboot, or process failure.

## Local Observatory

The Observatory is a browser client, not the Runtime or its supervisor. These
are the only supported root-shell commands:

```sh
./CSMctl observatory start
./CSMctl observatory status
./CSMctl observatory open
./CSMctl observatory urls
./CSMctl observatory logs
./CSMctl observatory stop
```

No Observatory command may start, stop, reload, replace, or claim ownership of
Runtime.

## Migration from the retired shell interface

Legacy Runtime invocations intentionally fail before touching service state and
print the canonical replacement. Use these mappings:

| Retired invocation | Canonical replacement |
| --- | --- |
| `./CSMctl`, `./CSMctl open` | Runtime: canonical `status`; UI: `./CSMctl observatory open` |
| `./CSMctl start`, `./CSMctl up`, `./CSMctl restart` | `csm runtime-v3 start` |
| `./CSMctl status` | `csm runtime-v3 status` |
| `./CSMctl stop` | `csm runtime-v3 stop` |
| `./CSMctl rotate-continuity-state` | No shell replacement; use the governed Runtime lifecycle path |
| `./CSMctl logs`, `./CSMctl urls` | Use canonical status and the configured service/observability surfaces |

Do not translate a legacy `restart` into an unconditional stop followed by
start. Canonical `start` is convergent and preserves the ownership checks.

## Post-operation checklist

- The canonical command exited with the expected status.
- `service_loaded` and `listener_ready` match the intended state.
- Guardian and Runtime process identities are present when running.
- `active_init_hash` matches the intended active init.
- `observability_ready` is true when running.
- No unowned listener or second Runtime was created.
- No credential or secret material was emitted or copied.

This runbook controls one host-local permanent Runtime service. It does not by
itself validate remote reachability, cloud infrastructure, provider execution,
agent admission, model preloading, or agent-to-agent behavior.
