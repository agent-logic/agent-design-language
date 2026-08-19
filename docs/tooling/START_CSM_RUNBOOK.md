# CSM / Runtime v3 startup runbook

This runbook is the operator path for starting or diagnosing the real local
CSM / Runtime v3 service and then opening the existing HTML Observatory against
that live Runtime API.

The intended operator behavior is deliberately simple:

```sh
./CSMctl start
```

The script starts the Runtime service if it is absent, asks macOS launchd to
keep it alive, probes the documented Runtime v3 `/v1` endpoints, and prints the
Observatory path with the correct Runtime query parameters.

It does not create a throwaway Runtime. It does not invent a second
Observatory service. It does not print credential values. It does not
force-kill, surprise-restart, replace, or take over an existing Runtime.

## What `CSMctl` controls

`CSMctl` controls only the Runtime v3 service process:

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
./CSMctl start
```

Successful output includes:

```text
CSMctl status=pass runtime_base=https://localhost:20997
CSMctl runtime=https://localhost:20997
CSMctl observatory=/path/to/demos/html-observatory/index.html?runtime=v3&runtimeApiBase=https://localhost:20997&live=1
```

If a compatible Runtime is already running, the script accepts it and prints
the same URLs. It does not try to start a duplicate service on the same port.
If any Runtime endpoint is responding but the three required probes are not all
HTTP 200, the script fails closed and refuses to kill or replace that Runtime.

## Commands

```sh
./CSMctl start
```

Start the real Runtime v3 service if needed, probe it, and print the Runtime
base plus Observatory path.

```sh
./CSMctl up
```

Alias for `start`.

```sh
./CSMctl status
```

Probe the Runtime without starting anything.

```sh
./CSMctl urls
```

Print the Runtime base and Observatory path without probing.

```sh
./CSMctl logs
```

Show recent log lines for the Runtime process started by this script.

```sh
./CSMctl stop
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
- `.adl/runtime-v3-service/CSMctl.conf`
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
checkout's repo-local service package and stable binaries through Git common-dir
discovery.

## Operator config

Use `.adl/runtime-v3-service/CSMctl.conf` for ordinary local service settings,
including browser-trusted TLS paths. This is intentionally like an Apache
startup config: paths and simple settings live in one local file, not in the
script.

Start by copying the complete template:

```sh
cp docs/tooling/CSMctl.conf.example .adl/runtime-v3-service/CSMctl.conf
chmod 600 .adl/runtime-v3-service/CSMctl.conf
```

The template lists the normal operator-changeable surface:

- repository and service package paths;
- state, generated, TLS, credential, continuity, quarantine, log, PID, lease,
  probe, runner, and plist paths;
- Runtime API port, bind address, public URL, and server name;
- launchd label, domain, and working directory;
- Runtime kernel, Vector, Python, and Observatory entrypoint paths;
- browser-facing API TLS cert/key/trust-root paths;
- private Runtime continuity TLS paths; and
- probe/recovery policy toggles.

Minimal local example:

```sh
ADL_CSM_RUNTIME_PORT=20997
ADL_CSM_RUNTIME_ADDRESS=127.0.0.1:20997
ADL_CSM_RUNTIME_BASE=https://localhost:20997
ADL_CSM_RUNTIME_PUBLIC_BASE_URL=https://localhost:20997
ADL_CSM_RUNTIME_SERVER_NAME=localhost

ADL_CSM_API_TLS_CERT=/Users/daniel/cert/localhost.pem
ADL_CSM_API_TLS_KEY=/Users/daniel/cert/localhost-key.pem
ADL_CSM_API_TLS_TRUST_ROOTS='/Users/daniel/Library/Application Support/mkcert/rootCA.pem'
```

Keep credential values out of `CSMctl.conf`. Secret tokens and signing keys
belong only in `.adl/runtime-v3-service/runtime.env`, and that file should not
be printed.

`CSMctl` copies the configured TLS material into local runtime state before
starting the service. If `ADL_CSM_API_TLS_TRUST_ROOTS` is present, probes use
that trust root instead of `curl -k`, so a normal browser and normal `curl`
should agree about the localhost certificate.

## TLS and browser note

The local Runtime uses HTTPS on `localhost`. For local browser trust, put the
mkcert certificate, key, and CA root paths in `CSMctl.conf` as shown above.

For public or investor-demo hosting with real DNS, use the externally issued
certificate and exact DNS names for the Runtime and Observatory hosts. This
local script does not prove public DNS, ingress, browser trust-store setup,
Unity, or cloud reachability.

## Overrides

Prefer `CSMctl.conf` for repeated local settings. One-shot environment
overrides are still available:

```sh
ADL_CSM_RUNTIME_PORT=20997 ./CSMctl start
ADL_CSM_RUNTIME_BASE=https://localhost:20997 ./CSMctl status
ADL_CSM_RUNTIME_ADDRESS=127.0.0.1:20997 ./CSMctl start
ADL_CSM_SERVICE_DIR=/path/to/.adl/runtime-v3-service ./CSMctl start
ADL_CSM_KERNEL_BIN=/path/to/adl-runtime-kernel ./CSMctl start
ADL_CSM_API_TLS_CERT=/path/to/fullchain.pem ADL_CSM_API_TLS_KEY=/path/to/privkey.pem ADL_CSM_API_TLS_TRUST_ROOTS=/path/to/ca.pem ./CSMctl start
ADL_CSM_OBSERVATORY_ENTRY=/path/to/demos/html-observatory/index.html ./CSMctl urls
```

Do not put credential values in command arguments or reusable shell history.

## Cert and key rotation

For certificate rotation, edit the three TLS path values in
`.adl/runtime-v3-service/CSMctl.conf`, then run:

```sh
./CSMctl stop
./CSMctl start
```

For continuity signing-key rotation, update the key in `runtime.env` without
printing it, then preserve the old signed continuity stores and prepare fresh
state:

```sh
./CSMctl rotate-continuity-state
./CSMctl start
```

The rotation command moves the old continuity directories into
`.adl/runtime-v3-service/state/quarantine/`; it does not delete them.

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
./CSMctl status
./CSMctl logs
```

### `runtime_not_ready_or_not_serving`

The Runtime did not satisfy the endpoint contract. Run:

```sh
./CSMctl logs
./CSMctl status
```

If another service owns port `20997`, stop that service or choose another
Runtime port with `ADL_CSM_RUNTIME_PORT`.

### Browser opens but dashboard is not live

First confirm the Runtime:

```sh
./CSMctl status
```

If Runtime probes pass but the browser is not live, the remaining issue is the
static Observatory serving path, browser TLS trust, or CORS/origin setup. The
Runtime service itself is already up.

## Validation used while authoring

The branch validated:

```sh
bash -n CSMctl
./CSMctl status
./CSMctl start
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
