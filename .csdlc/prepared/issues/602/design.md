# Issue 602 design: dynamic Runtime v3 agent admission

Status: approved for bounded implementation.

## Decision

Add one authenticated Runtime v3 admission endpoint and one direct `csmctl
agent add` client. The command reads the existing Runtime v3 init solely to
resolve the TLS trust root, address, and write-token path; it never edits or
reloads that file.

The Runtime validates the complete declaration, proves that the named Ollama
model exists at the declared endpoint, atomically persists the admitted-agent
set beneath the Runtime state root, and updates the in-memory roster in the
same governed operation. Repeating an identical declaration returns
`already_present`; reusing an identity with different fields fails closed.

## Command contract

```text
csmctl agent add \
  --init <runtime-init.toml> \
  --id <agent-id> \
  --name <display-name> \
  --role <role> \
  --provider ollama \
  --model <model> \
  --endpoint <http(s)-url>
```

The first implementation deliberately supports only the existing Ollama HTTP
provider. Adding unrelated providers is outside #602. The client uses the
Runtime's configured CA and exact TLS server name and sends the existing ACIP
write bearer token only in the Authorization header.

## Runtime contract

- `POST /v1/agents` is authenticated with the Runtime write bearer token.
- Unknown fields, unsupported providers, unsafe identifiers, empty model/name/
  role values, credential-bearing endpoints, and unavailable models are
  rejected before persistence or roster mutation.
- Endpoint verification calls the bounded Ollama tags surface and requires an
  exact model name.
- The dynamic admission file is versioned JSON, written through a sibling
  temporary file, synced, and atomically renamed.
- Runtime startup loads and validates the file before serving the API. Invalid
  persisted state fails startup rather than silently dropping agents.
- The resident Shepherd is seeded independently and cannot be replaced by a
  dynamic declaration.
- The roster update is synchronized; readers see either the pre-admission or
  post-admission population, never a partial entry.

## Idempotence and rollback

An identical existing declaration is a successful no-op and does not rewrite
state. A conflicting declaration for the same identity is rejected. If model
verification or durable write fails, the live roster remains unchanged. If an
in-memory update fails after persistence, the previous durable bytes are
restored before returning failure.

Removal/dehydration is not required for the first add demonstration because a
failed add has zero durable effect and an admitted declaration survives an
ordinary restart. A separately governed remove command may be added later if
operators need lifecycle deletion rather than admission rollback.

## Proof

- Focused parser/client tests for required arguments, TLS/write-token handling,
  and secret-free diagnostics.
- Runtime tests for authorization, success, duplicate, conflict, invalid input,
  unavailable model, durable reload, Shepherd preservation, and write failure.
- Live Wuji proof adds one `gemma4:e4b-mlx` agent, repeats the command, confirms
  Shepherd remains ready, and reads the new healthy communication-eligible
  entry from `/v1/agents` without Runtime restart or init-file modification.

## Non-goals

- Multi-node placement or scheduling.
- Downloading models.
- General provider registry mutation.
- Editing Runtime init.
- Replacing the resident Shepherd.
- Changing #589 lifecycle or continuity behavior.
