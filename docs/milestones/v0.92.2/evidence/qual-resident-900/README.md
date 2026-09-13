# Six-resident signed-restore qualification (#900)

The production Runtime executed two distinct `runtime.observe` workloads for
each of six resident roles through the Ollama HTTP adapter, ACC, UTS, signed
dehydration, restore, replay denial, and resumed work. The final qualification
receipt records 12 successful workload executions, six completed-case replay
denials, two checkpoint lineages per resident, an empty pending set, and a
verified generation-1 continuation.

The operator authorized a bounded local Mac substitute for the planned remote
profile: Apple M4 Pro, 64 GiB memory, CPU execution forced with
`LLAMA_ARG_DEVICE=none`, one loaded model, one request at a
time, 32,768 context tokens, 1,024 maximum output tokens, temperature 0, and
existing model blobs only. The task-owned Ollama service used loopback port
11436 because the proposed port 11435 was already occupied by a shared service.
It was stopped after the run. No model download, hosted provider, AWS resource,
paid call, shared-service mutation, or credential entered the evidence.

The reviewed #268 plan retains its historical `r7i.2xlarge` target-host field.
The issue-local `execution-envelope.json` is the authority for this explicitly
approved local execution. This result qualifies the production workload and
continuity behavior on that local profile; it does not claim r7i capacity or
remote-host performance.

The final materialization pins three Q4 model artifacts and the effective
qualification controls. Qwen thinking remains `ollama_server_default` because
the current provider adapter does not transmit an explicit thinking control.
Issue #970 owns the provider redesign needed to make every declared inference
parameter executable and observable.

Seven isolated production-state copies exercised changed signature, changed
payload, removed resident, substituted provider, substituted configuration,
substituted lineage, and stale snapshot. The first six were rejected by existing signature, integrity,
and exact-binding checks. The stale snapshot initially reopened admission,
revealing that restore did not compare the recovered signed generation with the
dehydration receipt generation. The candidate adds that exact comparison. The
final seven-case rerun rejected every scenario before restored work, kept
admission closed, preserved the active-population pointer, and created no new
restored generation.

Public evidence is summarized in `validation.json` and `test-manifest.json`.
The qualification receipt also binds the `local_ollama` provider, each actual
provider status/result and ACC effect receipt, Runtime and CSM binary hashes,
and producer source revision `e9fe1adf7b768dfd5fef234446d7db6cdfbaf37a`.

Detailed run data is retained under the issue-local
`.csdlc/evidence/900/attempt-11/` tree and is intentionally excluded from the
public packet because it contains machine-local paths and operational detail.
The public hashes bind those retained artifacts without publishing them.
