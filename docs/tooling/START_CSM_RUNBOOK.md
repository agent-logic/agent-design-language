# CSM / Runtime v3 startup runbook

This runbook is the operator path for starting or diagnosing the real CSM /
Runtime v3 service and for running the HTML Observatory as a separate local
static service that can point at any configured Runtime API.

The intended operator behavior is deliberately simple:

```sh
./CSMctl start
./CSMctl observatory open
```

`./CSMctl start` starts only the Runtime service if it is absent, asks macOS
launchd to keep it alive, and probes the documented Runtime v3 `/v1` endpoints.

`./CSMctl observatory open` starts only the local HTML Observatory static server
and opens it with the Runtime API base from `CSMctl.observatory.conf`. That
Runtime may be local, on another machine, or in AWS.

It does not create a throwaway Runtime. It does not invent a second
Runtime. It does not print credential values. It does not force-kill,
surprise-restart, replace, or take over an existing Runtime.

## What `CSMctl` controls

`CSMctl start|status|stop|logs` controls only the Runtime v3 service process:

- Default Runtime API base: `https://localhost:20997`
- Runtime binary: `.adl/runtime-v3-service/generated/bin/adl-runtime-kernel`
- Service package: `.adl/runtime-v3-service/`
- Local service config generated under `.adl/runtime-v3-service/generated/`
- macOS keepalive label: `com.agentlogic.start-csm`
- Runtime process log: `.adl/runtime-v3-service/state/start_CSM.log`

`CSMctl observatory start|status|stop|open|logs` controls only the local static
HTML Observatory server:

- Default Observatory base: `https://localhost:8765`
- Fallback local port: `8766`
- Static document root: `demos/html-observatory/`
- Runtime API target: `ADL_CSM_RUNTIME_BASE` from `CSMctl.observatory.conf`
- macOS launchd label: `com.agentlogic.csm-observatory`

The Runtime and Observatory do not need to be on the same machine. For normal
local testing they can both run here. For AWS or another remote CSM, leave the
Runtime alone and run only:

```sh
./CSMctl observatory open
```

The HTML Observatory is in the repo at:

```text
demos/html-observatory/index.html
```

The static server opens it with:

```text
?runtime=v3&runtimeApiBase=<configured-runtime-api-base>&live=1
```

The Runtime API root itself is not the Observatory UI. A browser pointed at
`https://localhost:20997/` may correctly see HTTP 404 because the Runtime API
serves the `/v1/*` endpoints, not an HTML dashboard at `/`.

## One-command flow

From the repository root:

```sh
./CSMctl start
./CSMctl observatory open
```

Successful Runtime output includes:

```text
CSMctl status=pass runtime_base=https://localhost:20997
CSMctl runtime=https://localhost:20997
```

If a compatible Runtime is already running, the script accepts it and prints
the same URLs. It does not try to start a duplicate service on the same port.
If any Runtime endpoint is responding but the three required probes are not all
HTTP 200, the script fails closed and refuses to kill or replace that Runtime.

Successful Observatory output includes a browser URL similar to:

```text
CSMctl observatory_server=running url=https://localhost:8766/index.html?runtime=v3&runtimeApiBase=https://localhost:20997&live=1
CSMctl opening=https://localhost:8766/index.html?runtime=v3&runtimeApiBase=https://localhost:20997&live=1
```

The selected Observatory port may be `8766` if `8765` is already occupied. Use
the URL printed by `./CSMctl observatory urls`; do not assume the first
configured port is the live one.

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

Print the Runtime base and configured Observatory URL without probing.

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

```sh
./CSMctl observatory start
```

Start only the local HTML Observatory static server. This does not start,
stop, probe, or mutate Runtime. Use this when the Runtime is already running
elsewhere, including AWS.

```sh
./CSMctl observatory open
```

Start only the local HTML Observatory static server and open the selected local
Observatory URL in the browser.

```sh
./CSMctl observatory status
./CSMctl observatory stop
./CSMctl observatory logs
```

Inspect, gracefully stop, or tail the local Observatory static server.

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
- `.adl/runtime-v3-service/CSMctl.observatory.conf`
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

## Runtime config

Use `.adl/runtime-v3-service/CSMctl.conf` for ordinary local service settings,
including browser-trusted TLS paths. This is intentionally like an Apache
startup config: paths and simple settings live in one local file, not in the
script. This config is Runtime-only.

Start by copying the complete template:

```sh
cp docs/tooling/CSMctl.conf.example .adl/runtime-v3-service/CSMctl.conf
chmod 600 .adl/runtime-v3-service/CSMctl.conf
```

The Runtime template lists the normal operator-changeable surface:

- repository and service package paths;
- state, generated, TLS, credential, continuity, quarantine, log, PID, lease,
  probe, runner, and plist paths;
- Runtime API port, bind address, public URL, and server name;
- launchd label, domain, and working directory;
- Runtime kernel, Vector, and Python paths;
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

## Observatory config

Use `.adl/runtime-v3-service/CSMctl.observatory.conf` for the local static
Observatory server and the Runtime API URL it should call. This is separate on
purpose: a laptop Observatory can point at a CSM running in AWS, and a Runtime
machine does not need to host the Observatory.

Start by copying the complete template:

```sh
cp docs/tooling/CSMctl.observatory.conf.example .adl/runtime-v3-service/CSMctl.observatory.conf
chmod 600 .adl/runtime-v3-service/CSMctl.observatory.conf
```

For local testing:

```sh
ADL_CSM_RUNTIME_BASE=https://localhost:20997
ADL_CSM_RUNTIME_PUBLIC_BASE_URL=https://localhost:20997
```

For AWS or another remote CSM:

```sh
ADL_CSM_RUNTIME_BASE=https://runtime.example.com
ADL_CSM_RUNTIME_PUBLIC_BASE_URL=https://runtime.example.com
```

For the local Observatory static server:

```sh
ADL_CSM_OBSERVATORY_DIR=/path/to/agent-design-language/demos/html-observatory
ADL_CSM_OBSERVATORY_ENTRY=/path/to/agent-design-language/demos/html-observatory/index.html
ADL_CSM_OBSERVATORY_HOST=127.0.0.1
ADL_CSM_OBSERVATORY_PORT=8765
ADL_CSM_OBSERVATORY_PORTS='8765 8766'
ADL_CSM_OBSERVATORY_TLS_CERT=/Users/daniel/cert/localhost.pem
ADL_CSM_OBSERVATORY_TLS_KEY=/Users/daniel/cert/localhost-key.pem
ADL_CSM_OBSERVATORY_LAUNCH_LABEL=com.agentlogic.csm-observatory
```

Changing certs or ports is just a config edit plus:

```sh
./CSMctl observatory stop
./CSMctl observatory start
```

## TLS and browser note

The local Runtime uses HTTPS on `localhost`. For local browser trust, put the
mkcert certificate, key, and CA root paths in `CSMctl.conf` as shown above.

For public or investor-demo hosting with real DNS, use the externally issued
certificate and exact DNS names for the Runtime API host. The local Observatory
config should point at that HTTPS origin. This local script does not prove
public DNS, ingress, browser trust-store setup, Unity, or cloud reachability.

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
ADL_CSM_OBSERVATORY_CONFIG_FILE=/path/to/CSMctl.observatory.conf ./CSMctl observatory open
ADL_CSM_RUNTIME_BASE=https://runtime.example.com ./CSMctl observatory open
```

Do not put credential values in command arguments or reusable shell history.

## Cert and key rotation

For Runtime certificate rotation, edit the three TLS path values in
`.adl/runtime-v3-service/CSMctl.conf`, then run:

```sh
./CSMctl stop
./CSMctl start
```

For Observatory certificate rotation, edit the two Observatory TLS path values
in `.adl/runtime-v3-service/CSMctl.observatory.conf`, then run:

```sh
./CSMctl observatory stop
./CSMctl observatory start
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

First confirm the Runtime API that the Observatory config points at:

```sh
./CSMctl status
```

For a remote Runtime, use a direct probe against that configured origin instead
of starting local Runtime:

```sh
curl https://runtime.example.com/v1/ready
curl https://runtime.example.com/v1/observatory
curl https://runtime.example.com/v1/health
```

Then confirm the local Observatory static server:

```sh
./CSMctl observatory status
./CSMctl observatory urls
```

If Runtime probes pass but the browser is not live, the remaining issue is the
static Observatory serving path, browser TLS trust, or CORS/origin setup.

### Browser shows HTTP 404 at `https://localhost:20997/`

That URL is the Runtime API origin, not the Observatory page. Open the URL
printed by:

```sh
./CSMctl observatory open
```

or:

```sh
./CSMctl observatory urls
```

### Browser shows HTTP 404 at `https://localhost:8765/`

Another local service may already own the first configured Observatory port.
Ask `CSMctl` for the selected live URL:

```sh
./CSMctl observatory urls
```

If the script reports `observatory_port_unavailable port=8765 action=try_next`,
open the printed fallback URL, commonly:

```text
https://localhost:8766/index.html?runtime=v3&runtimeApiBase=https://localhost:20997&live=1
```

## Validation used while authoring

The branch validated:

```sh
bash -n CSMctl
./CSMctl status
./CSMctl start
./CSMctl observatory start
./CSMctl observatory status
git diff --check
```

Observed local endpoint state during authoring:

```text
/v1/ready        HTTP 200
/v1/observatory  HTTP 200
/v1/health       HTTP 200
```

That proves the local Runtime v3 service was serving the Runtime API contract
and that the local static Observatory server could serve the HTML entrypoint.
It does not claim public deployment, Unity, or provider proof.
