# #880 implementation and validation inventory

Scope: production CI acquisition delegates immutable Git capture and packet
validation to the accepted #878 local adapter. CI metadata is allowlisted and
bound separately; it cannot override revision/scope or perturb common identity.
The private ingest/ci handler is independent of concurrent #879 ingest/github.

PVF runtime integration, deterministic local Git fixtures, bounded CPU/disk,
no model/network: `codefriend_ci_ingestion::ci_production_route_conformance`
executes ten production CLI scenarios through `ci_smoke.py`, reads emitted
packets through the production reader, and checks receipt revision tampering
and false delivery state rejection. Scenarios cover usable packet, local parity,
relocation, missing/malformed/unresolvable revision, shallow available/missing
revision, partial missing input, path/byte limits, metadata credential/path/key
rejection, failed receipt publication and repository mismatch. This is the
required local #880 gate. Existing local ingestion safety is reused.

PVF deterministic CI contract, no build/network: `test_ci_contract.rb` executes
12 actual aggregate-shell outcomes and five selector cases, inspects required
job dependency, least privileges, installed path and mandatory upload handling.
Existing six-case UTS gate and whole-workflow policy remain passing.

Required hosted gate: codefriend-ci-acquisition builds and installs exact
candidate, executes the same ten-case smoke, records candidate binary SHA256,
source commit and per-artifact digests, and requires actual upload success plus
nonempty transport ID/digest. This hosted execution is pending before PR.
The installed local smoke with precommit baseline revision is developmental
proof only; final exact candidate installed proof is recorded after commit.
No upload, review result, fitness outcome or source-change authority is inferred
from acquisition. A partial write returns failure; retained packet is not a
successful delivery. Negative fixture artifacts remain labeled by filenames
and the case inventory, not represented as successful production receipts.

Stdout contains JSON acquisition/readback responses; stderr contains bounded
category-only errors. No new compatibility logging or telemetry is claimed.
No environment dump, runner absolute path, credential resolution, inference,
cloud deployment or Vector build is performed.
