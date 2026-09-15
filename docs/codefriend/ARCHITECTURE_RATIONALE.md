# Architecture rationale from admitted evidence

The rationale reporter relates graph layers to declared deployment services and
recorded ADR choices. It retains the three evidence levels separately: observed
layer membership and deployment configuration, inferred candidate quanta, and
human-recorded rationale. A directory, an ADR title or a candidate decision never
establishes independent deployability or accepted design authority.

Generate a structure graph first, then select its revision and digest:

```json
{
  "schema": "codefriend.rationale.v1",
  "graph_digest": "<digest from structure report>",
  "revision": "<exact admitted revision>",
  "boundaries": [{
    "boundary": "core",
    "deployment_path": "compose.json",
    "service": "api",
    "rationale_paths": ["docs/adr/001-api.md"]
  }]
}
```

```sh
adl codefriend architecture rationale --store store --graph graph.json --selection selection.json --out rationale.json
adl codefriend architecture rationale-read --store store --input rationale.json
```

All source paths must already belong to the graph's admitted evidence packet.
Select Rust files as analysis and deployment/ADR documents as context during
acquisition. Missing or redacted documents remain unavailable; no remote fetch
or source execution occurs. Unknown graph boundaries and mismatched ADR
boundary/service references reject the request. Stale graph identity rejects it.

The supported deployment input is Docker Compose JSON with a literal image and
a matching boundary label:

```json
{"services":{"api":{"image":"example/api:1","labels":{"codefriend.boundary":"core"}}}}
```

The reporter observes this service declaration and infers a **candidate** quantum.
It does not build the image, deploy it, test rollout independence, or treat a label
as runtime proof. For the selected service, fields beyond `image` and `labels`
(including dependencies and shared namespaces) produce an unknown deployment
relationship. Unsupported top-level Compose constructs likewise remain unknown.
Literal images are required; interpolated configuration is not resolved.

An ADR is Markdown with bounded TOML metadata between `+++` lines:

```markdown
+++
status = "accepted"
boundary = "core"
service = "api"
decision_key = "api-deployment"
choice = "separate-service"
+++
# API deployment
Keep the API in a separate service to permit independent rollout of its image.
```

The original status and explanation are retained with evidence location and
revision. `candidate` and `superseded` records cannot supply accepted rationale.
Unknown statuses remain visible and make analysis partial. Two accepted records
with different choices for the same decision key yield a conflict; distinct keys,
identical choices and candidate/superseded alternatives do not. This comparison
is explicit-choice matching, not semantic contradiction detection over prose.
Unstructured or unsupported ADR documents remain unknown rather than being
silently interpreted as decisions. Empty rationale sets report no accepted record.

`analysis_complete` means that the selected relationships and explicit choices
were analyzed without missing, unsupported or conflicting evidence. It never
certifies the architecture or measured runtime deployability. CLI consumers must
inspect this field: supported partial analysis returns a JSON summary and a
persisted report, while invalid input returns an error. Shared Run/Findings retain
evidence references and inference limits. Readback fully recomputes from the live
store, denying modified reports and deleted or expired admissions.

Limits are 32 unique boundaries, 64 unique selected ADR paths per boundary,
64 KiB per source document, 8 KiB each for ADR metadata and prose, 128 KiB for
selection JSON, and 32 MiB for a persisted artifact. Identifiers are bounded
ASCII names. Output is create-only; artifact traversal and symlinks reject.
Stdout is machine-readable JSON, stderr contains counts and status. Inspected
source remains unchanged; its instructions are evidence, never commands.

The `codefriend_cf_cog_rationale` tests exercise production parsing, traces,
status/conflict handling, unavailable evidence, bounds and retention. The installed
proof runner drives actual acquisition, graph creation, rationale and readback.
Its fixed conflict sample distinguishes an explicit conflict from different-key,
candidate and superseded alternatives. Reported sample accuracy is restricted to
those fixtures; final external qualification remains separate.
