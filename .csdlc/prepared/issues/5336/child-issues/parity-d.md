## Goal

Make Runtime v3 simple to run locally and remotely through one secure,
configuration-driven access model with guardian supervision and live
Observatory consumption.

## Owned Capability Groups

- ACIP/A2A/cloud network boundary;
- local and remote HTTPS/API-gateway access selected in the init file;
- HTML Observatory authenticated HTTP/WebSocket live consumption;
- health/weather and Vector-owned telemetry routing;
- guardian packaging, soak, graceful stop, and rollback.

## Required Outcome

The guardian reads the initialization file, launches and supervises the
canonical kernel, and exposes one secure API configuration on the declared
Runtime v3 port. The HTML Observatory consumes real agent/runtime state without
hard-coded IPs. Soak, pressure serialization, restart, and selector rollback
are retained at exact revisions. Cloud delivery remains a credential-free
boundary proof; no AWS execution is required.

## Deliverables

- Secure local/remote initialization contract and negative configuration tests.
- Live Runtime v3 plus HTML Observatory proof with an admitted agent and
  scale-safe contract evidence.
- Guardian restart/pressure-stop/rollback proof.
- Vector-owned telemetry route proof without custom OTel infrastructure.

## Parent And Dependencies

- Parent acceptance umbrella: #5361.
- Architecture and budgets: #5336.
- Depends on Parity-A process and ingress contracts.
- Port 20997 remains configuration-driven and is never hard-coded in discovery
  output.

## Definition Of Done

- Production code is exercised through `adl-runtime-kernel`; fixture and
  opt-in metadata evidence is insufficient.
- Deterministic positive and negative evidence is retained at an exact
  revision, including graceful shutdown/recovery.
- Duplicate or placeholder code is deleted to preserve the #5336 budget.
- No AWS use, HTTP-only access, hard-coded IPs, default switch, Runtime v2
  deletion, or new product scope.
