# CodeFriend host idle controller (#1077)

These are installation inputs, not deployed service evidence. No command here
installs, enables, starts, or powers off a host automatically during testing.
The Terraform directory is the separately reviewed host preparation surface.

## Installation contract

An authorized host installer must create the `codefriend` account, private
`/var/lib/codefriend`, and the fixed root-installed release under
`/opt/codefriend/current`. Its `bin/codefriend-server` and report verifier must
be exact-candidate Linux binaries; `website/` must contain the independently
pinned website release as a root-owned Git checkout (with Git metadata and
`git` installed), plus installed dependencies; `host/idle_stop.py` is this
controller. Install Node 24 at `/usr/bin/node`. The website's provenance check supplies an
invocation-scoped exact `safe.directory` for its root-owned checkout after
checking source ownership and bytes. Do not configure wildcard Git trust or make
source service-writable. Before each release activation, run its provenance
preflight under the service UID:

```
sudo -u codefriend /usr/bin/node --input-type=module -e 'import {websiteCandidate} from "/opt/codefriend/current/website/app/host-control.mjs"; console.log(websiteCandidate())'
```

It must return the pinned installed commit. Dirty, service-writable or missing
source/Git metadata blocks activation. Release paths, symlink parents,
unit files and idle controller/config must be root-owned and not writable by
service users. Never build source on the small runtime host as part of startup.

Install the supplied unit files into `/etc/systemd/system` only under authorized
host deployment. The gateway and website use separate private runtime directories
so stopping one does not remove the other's socket. They share the `codefriend`
UID and private state; this is not an OS security isolation claim.

Provide private service configuration (no secrets are supplied in this repo):

- `/etc/codefriend/gateway.json`: validated gateway configuration with durable
  state under `/var/lib/codefriend`, existing retention promises, and shared
  website-created credential registry.
- `/etc/codefriend/provider.env`: provider environment, readable only by root;
  systemd reads it before dropping gateway privileges.
- `/etc/codefriend/website.json`: private readable-by-codefriend configuration,
  including `controlSocket: "/run/codefriend-website/control.sock"`, real OAuth
  registration, invitation file and report verifier. The website private
  directory must be under `/var/lib/codefriend`.
- `/etc/codefriend/idle-stop.json`: root-owned mode 0600, with the exact shape
  `{"enabled":false,"revisions":{"website":"<40 lowercase hex>","gateway":"<40 lowercase hex>"}}`.
  Fill revisions from authenticated installed release provenance. Enabling
  requires explicit operator deployment/shutdown authorization; false is the
  preparation default. Parent directories must be root-owned, without group
  or other write permissions.

After installation, observation only is:

```
sudo /usr/bin/python3 /opt/codefriend/current/host/idle_stop.py
```

The runtime directory must exist (systemd creates it for the supplied service).
`--execute` additionally requires enabled config and can stop both services and
request real host poweroff. Never use that flag during local tests. Timer
activation is a deployment action, not part of repository validation.
EC2 instance-initiated shutdown must be configured as **stop**, as in the host
plan. Starting EC2 manually starts enabled application services on boot. There
is no automatic AWS start or provider dispatch in this controller.

## Stop protocol and failure handling

After 1,800 seconds of meaningful website inactivity, the controller discovers
both current systemd MainPIDs/InvocationIDs and exact-candidate control instances.
Each bounded Unix RPC verifies Linux SO_PEERCRED's PID, current supervisor state,
response identity and candidate. It drains website admissions first, then gateway
admissions, using a fresh attempt. Before allocating a drain attempt, read-only status from both services must
report `quiescent_without_payloads: true`. Expected retention deferrals therefore
consume no retired attempt IDs. Readiness can race with new work, so after
freezing, both must report the same owned attempt,
current instance and no retained payloads or uncertain work. Website idle time is
checked again after freeze. Existing retention deadlines are preserved: retained
results can delay shutdown past 30 minutes. Polling alone must not reset the
website's activity clock; that behavior belongs to the website implementation.

Both services are rechecked, stopped gracefully through fixed systemd unit names,
and verified inactive before the fixed `systemctl poweroff` request. No shell
commands or service names come from configuration. Explicit stops do not auto
restart services. Concurrent operator starts/deployments are outside this stop
transaction: operators must suspend the timer before maintenance. The controller
checks for observed instance changes but does not claim an atomic lock against
root operators or unrelated systemd clients.

Any failure denies host stop and attempts to resume only exact owned live drains.
A lost drain response may have taken effect, so ownership is retained before the
RPC. If a process was already gracefully stopped before a later failure, it stays
stopped; operator investigation/start is required. No blind socket, website lock,
operation reservation, or tombstone deletion occurs. Logs are generic and omit
payloads/secrets. The generic stop-denied outcome requires operator inspection;
it does not establish the reason or imply a retry is safe for unrelated actions.

## Validation and limits

```
python3 -m unittest discover -s tools/codefriend_host -p 'test_*.py' -v
```

The suite substitutes supervisor and transport; it cannot power off a host or
call AWS/providers. It proves protocol decision logic, stop ordering, timeout
after drain effects, inactivity boundaries, retained/uncertain payload refusal,
stale identities, stop failures and observed restart races. The coupled PVF
manifest declares these deterministic bounded component tests. Linux peer
credentials, actual systemd unit behavior, real OAuth/provider journeys, retained
cleanup and EC2 stop/manual restart require separate installed-host acceptance.

## Installed Linux control probe (no real host stop)

`probe_linux_control.py` uses real Linux Unix sockets and `SO_PEERCRED` but an
explicit fake supervisor. It has no subprocess/systemctl/AWS call. Run only on an
already authorized isolated Linux fixture with actual candidate gateway/website
processes bound to the standard private control socket paths above; do not replace
or start any existing runtime to run it. Empty fixture credentials and a provider
endpoint that cannot dispatch are sufficient; no provider call is needed.

```
python3 tools/codefriend_host/probe_linux_control.py \
  --website-pid <fixture-pid> --website-revision <installed-commit> \
  --gateway-pid <fixture-pid> --gateway-revision <installed-commit>
```

Default mode requests status only (services may clean expired payloads during
status); it does not drain admissions or mutate host/supervisor state. For an empty fixture that has actually idled
30 minutes, the additional `--exercise-drain-on-isolated-fixture` flag exercises
real drain/resume with simulated service stops and poweroff; finally it resumes
its exact owned drains. Never use that flag against production. Recovery failure
is a failed probe needing operator inspection. This proves transport and live
control integration only: supplied PIDs/fake invocation IDs cannot prove systemd
ownership, graceful process stop, retention acceptance or EC2 stop/start.

An isolated `codefriend1077` Colima profile (aarch64, 2 CPUs, 2 GiB RAM) subsequently
ran all 30 component/kernel tests successfully, including real Linux SO_PEERCRED
with a synthetic socket service. Source SHA256s are retained with the run evidence.
This does not prove actual gateway/website integration. `systemd-analyze verify`
reported absent gateway and Node service executables, so full unit validation
remains pending. A separate actual website probe on Linux aarch64 at commit
`0e39cb7ddd1a0d884ec623595e9eaac52b468efe` passed two startup/control/drain/resume/
graceful-SIGINT cycles as the service UID using root-owned source and Node24.0.0.
That probe exposed and verified a fix for exact Git trust path normalization.
It did not execute OAuth, providers, the gateway, report verification, real systemd
supervision, host poweroff or production amd64 artifacts. Production installed-stack
acceptance remains separate.

## Fresh-host credential registry ordering

Before starting either service, the authorized installer must create the website
private state directory as codefriend-owned mode0700 and initialize the shared
registry. Gateway `credentials_file` and website `privateDirectory` must select
this same registry path. The gateway starts first; it must not depend on the
website's first startup to create its required credentials file.

```
sudo /usr/bin/python3 /opt/codefriend/current/host/initialize_registry.py --website-private-directory /var/lib/codefriend/website
```

Use the actual configured website directory below `/var/lib/codefriend`. This
explicit installer helper creates `gateway-credentials.json` as mode0600,
codefriend-owned `[]` using exclusive create and fsync. An existing object is never replaced. Existing symlinks, non-regular files,
wrong owners, non-private permissions and malformed/beyond-bound array data fail
closed, including an empty file left by interrupted initialization. Existing entries must match the gateway credential fields, mode, subject,
unique hash and unsigned expiry contract within the same 128 KiB bound. Valid
registries are preserved. Correction requires operator-reviewed configuration. No reservations or prior registry are removed. The component tests
prove creation and preservation; actual first-boot service ordering remains an
installed-host acceptance obligation.
