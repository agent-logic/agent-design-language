# CSM / Runtime v3 startup runbook

This runbook is the operator path for starting or diagnosing the real local
CSM / Runtime v3 service and then opening the existing HTML Observatory against
that live Runtime API.

The intended operator behavior is deliberately simple:

```sh
./start_CSM.sh start
```

The script starts the Runtime service if it is absent, asks macOS launchd to
keep it alive, probes the documented Runtime v3 `/v1` endpoints, and prints the
Observatory path with the correct Runtime query parameters.

It does not create a throwaway Runtime. It does not invent a second
Observatory service. It does not print credential values. It does not
force-kill, surprise-restart, replace, or take over an existing Runtime.

## What `start_CSM.sh` controls

`start_CSM.sh` controls only the Runtime v3 service process:

- Default Runtime API base: `https://localhost:20997`
- Runtime binary: `.adl/runtime-v3-service/generated/bin/adl-runtime-kernel`
- Service package: `.adl/runtime-v3-service/`
- Local service config generated under `.adl/runtime-v3-service/generated/`
- macOS keepalive label: `com.agentlogic.start-csm`
- Runtime process log: `.adl/runtime-v3-service/state/start_CSM.log`

The HTML Observatory is already in the repo at:

```text
demos/html-observatory/index.html
```

The script prints that path with:

```text
?runtime=v3&runtimeApiBase=https://localhost:20997&live=1
```

If a browser blocks `file://` fetches, serve `demos/html-observatory/` using any
normal local/static HTTPS path already approved for the demo environment. That
is a browser/static-hosting concern, not a second Runtime.

## One-command flow

From the repository root:

```sh
./start_CSM.sh start
```

Successful output includes:

```text
start_CSM status=pass runtime_base=https://localhost:20997
start_CSM runtime=https://localhost:20997
start_CSM observatory=/path/to/demos/html-observatory/index.html?runtime=v3&runtimeApiBase=https://localhost:20997&live=1
```

If a compatible Runtime is already running, the script accepts it and prints
the same URLs. It does not try to start a duplicate service on the same port.
If any Runtime endpoint is responding but the three required probes are not all
HTTP 200, the script fails closed and refuses to kill or replace that Runtime.

## Commands

```sh
./start_CSM.sh start
```

Start the real Runtime v3 service if needed, probe it, and print the Runtime
base plus Observatory path.

```sh
./start_CSM.sh up
```

Alias for `start`.

```sh
./start_CSM.sh status
```

Probe the Runtime without starting anything.

```sh
./start_CSM.sh urls
```

Print the Runtime base and Observatory path without probing.

```sh
./start_CSM.sh logs
```

Show recent log lines for the Runtime process started by this script.

```sh
./start_CSM.sh stop
```

Gracefully stop the launchd-owned Runtime. The generated runner forwards TERM
to the kernel and waits for the Runtime checkpoint shutdown path so agent state
can dehydrate before the process exits.

## Runtime endpoint contract

The script probes:

```text
GET https://localhost:20997/v1/ready
GET https://localhost:20997/v1/observatory
GET https://localhost:20997/v1/health
```

The local service is considered usable when:

- `/v1/observatory` returns HTTP 200;
- `/v1/health` returns HTTP 200; and
- `/v1/ready` returns HTTP 200.

`/v1/ready` is the strict gate. A 503 is not accepted as a successful startup.
During boot the script may print transitional 503s, but it reports success only
after all three endpoints return HTTP 200.

## Files used

The script reads:

- `.adl/runtime-v3-service/runtime.env`
- `adl-runtime/tests/support/tls-fixtures/server-cert.pem`
- `adl-runtime/tests/support/tls-fixtures/server-key.pem`
- `adl-runtime/tests/support/tls-fixtures/root-ca.pem`
- `.adl/runtime-v3-service/generated/bin/adl-runtime-kernel`
- `.adl/bin/vector`

The script may generate local operational artifacts under:

- `.adl/runtime-v3-service/generated/`
- `.adl/runtime-v3-service/state/`

Do not commit generated credential fragments, token files, PID files, probe
files, or logs.

When run from a FastWork issue worktree that lacks ignored `.adl/bin` or
`.adl/runtime-v3-service` artifacts, the script falls back to the primary
checkout's repo-local service package and stable binaries through Git
common-dir discovery.

## TLS and browser note

The local Runtime uses HTTPS on `localhost`. The script probes with `curl -k`
because this is local operator bring-up using local TLS material.

For public or investor-demo hosting with real DNS, use the externally issued
certificate and exact DNS names for the Runtime and Observatory hosts. This
local script does not prove public DNS, ingress, browser trust-store setup,
Unity, or cloud reachability.

## Overrides

Use overrides only for local routing:

```sh
ADL_CSM_RUNTIME_PORT=20997 ./start_CSM.sh start
ADL_CSM_RUNTIME_BASE=https://localhost:20997 ./start_CSM.sh status
ADL_CSM_RUNTIME_ADDRESS=127.0.0.1:20997 ./start_CSM.sh start
ADL_CSM_SERVICE_DIR=/path/to/.adl/runtime-v3-service ./start_CSM.sh start
ADL_CSM_KERNEL_BIN=/path/to/adl-runtime-kernel ./start_CSM.sh start
ADL_CSM_TLS_CERT=/path/to/fullchain.pem ADL_CSM_TLS_KEY=/path/to/privkey.pem ./start_CSM.sh start
ADL_CSM_OBSERVATORY_ENTRY=/path/to/demos/html-observatory/index.html ./start_CSM.sh urls
```

Do not put credential values in command arguments or reusable shell history.

## Troubleshooting

### `missing_service_env`

The service package is missing or incomplete. Confirm the file exists without
printing it:

```sh
ls .adl/runtime-v3-service/runtime.env
```

### `missing_or_not_executable_kernel_binary`

The generated Runtime kernel binary is absent. Build or install the current
Runtime kernel into `.adl/runtime-v3-service/generated/bin/adl-runtime-kernel`,
then retry.

### `runtime_present_but_not_all_200`

Something is already answering on one or more Runtime endpoints, but startup is
not healthy. The script will not replace it. Inspect:

```sh
./start_CSM.sh status
./start_CSM.sh logs
```

### `runtime_not_ready_or_not_serving`

The Runtime did not satisfy the endpoint contract. Run:

```sh
./start_CSM.sh logs
./start_CSM.sh status
```

If another service owns port `20997`, stop that service or choose another
Runtime port with `ADL_CSM_RUNTIME_PORT`.

### Browser opens but dashboard is not live

First confirm the Runtime:

```sh
./start_CSM.sh status
```

If Runtime probes pass but the browser is not live, the remaining issue is the
static Observatory serving path, browser TLS trust, or CORS/origin setup. The
Runtime service itself is already up.

## Validation used while authoring

The branch validated:

```sh
bash -n start_CSM.sh
./start_CSM.sh status
./start_CSM.sh start
git diff --check
```

Observed local endpoint state during authoring:

```text
/v1/ready        HTTP 200
/v1/observatory  HTTP 200
/v1/health       HTTP 200
```

That proves the local Runtime v3 service was serving the Observatory API and
strict readiness endpoint. It does not claim public deployment, static hosting,
Unity, or provider proof.
