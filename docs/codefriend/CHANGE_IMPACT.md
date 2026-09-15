# Bounded change impact

`adl codefriend architecture impact` reports potential module impact from an
admitted structure graph. It follows incoming dependencies: if A references B,
a declared change to B may affect A. It retains one deterministic shortest
edge-by-edge witness for every affected module and changed root. Cycles terminate;
multiple roots retain separate witnesses. Risk indicators identify transitive
paths, crossed layers and participation in a cycle with the changed root.
These are syntactic inferences, not predictions of runtime failures.

First create a graph using `adl codefriend architecture report`. Supply the
repository, revision and digest from that graph in an explicit change file:

```json
{
  "schema": "codefriend.impact.v1",
  "repository": "https://example.com/owner/repo",
  "revision": "<exact graph revision>",
  "graph_digest": "<exact graph digest>",
  "targets": [{"kind": "module", "name": "crate::storage"}]
}
```

```sh
adl codefriend architecture impact --store store --graph structure.json --changes changes.json --out impact.json
adl codefriend architecture impact-read --store store --input impact.json
```

Changes are declared inputs. The command does not calculate or verify a Git diff,
apply edits, compile inspected repositories, or execute their scripts. Exact
module names select graph nodes. A `symbol` target is accepted as an explicit
unsupported request and makes the report partial; symbols are never silently
resolved to modules. Names absent from the graph are likewise explicit unknowns.
The graph's own unknown evidence and acquisition completeness remain attached.

Every impact carries the canonical change digest, graph digest, graph revision,
edge evidence, locations and inference. Shared CF-EVIDENCE Run and Finding records
are persisted with the graph. Readback recomputes against the live evidence store:
deleted, expired, altered or incompatible source evidence cannot authorize an old
report. The digest binds a declared input; it does not authenticate an operator's
claim that these are all actual changes.

`analysis_complete` means complete reachability for the supported inputs within
this bounded syntactic graph. `scoped_unimpacted_nodes` means no such path was
found, never semantic independence or runtime safety. It is empty whenever graph
knowledge or change resolution is partial. A successful partial command emits
JSON with `analysis_complete: false`; consumers must inspect that field. Invalid
inputs return an error and do not create an output artifact.

Limits: 1–64 unique change targets, 512 bytes per target name, inherited graph
limits of 512 nodes and 4096 edges, 1000 affected root/module pairs, and 512 edges
per witness. Exceeding a bound fails instead of silently truncating. Change JSON
is limited to 128 KiB; persisted impact artifacts are limited to 32 MiB and are
create-only. Existing files, traversal and artifact symlinks are rejected. CLI
stdout is a JSON summary; stderr events contain counts and status, not source
content or local paths. Repository content remains admitted redacted evidence.

## Validation and calibration

`codefriend_cf_cog_impact` exercises the production reporter with admitted inert
Git fixtures. `adl/tools/codefriend_impact_installed_proof.py` drives the installed
product through acquisition, graph creation, impact, persistence and readback.
It checks exact expected sets for a chain and cycle, explicit partial output for
unknown edges and unsupported input, stale revisions, malformed/bounded inputs,
tampering, overwrite denial, deletion and source immutability.

Calibration measures sampled potential syntactic dependents against manually
specified fixture expectations. Zero false positives/negatives on these small
fixtures does not establish runtime impact accuracy, general repository recall,
or externally qualified support. Independent review must inspect the expected
sets and paths; executed proof records report their actual denominators and
platform. No provider or external repository execution is required.
