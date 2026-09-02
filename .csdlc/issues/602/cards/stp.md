# Structured Task Prompt

Template: 1.0.0

Issue: 602

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Implement only dynamic local Runtime v3 Ollama agent admission, durable restart recovery, csmctl client behavior, roster visibility, and their focused proof.

## Deliverables

- csmctl agent add command
- Authenticated Runtime v3 POST /v1/agents admission API
- Atomic versioned dynamic-admission persistence and restart reload
- Immediate synchronized roster projection with Shepherd preserved
- Focused success duplicate conflict invalid unavailable-model and rollback tests
- Live Wuji gemma4:e4b-mlx demonstration

## Acceptance

1. csmctl exposes agent add, list, get, checkpoint, remove, dehydrate, migrate, and rehydrate commands without Runtime init edits or ordinary restart requirements
2. A verified added or rehydrated agent is immediately healthy, available, communication-eligible, and visible while Shepherd remains ready and cannot be removed, checkpointed, or migrated
3. Identical mutations are idempotent, conflicting identities and stale migration commits fail closed, and list/get are authenticated through the configured Runtime trust boundary
4. Invalid authorization, provider, model, endpoint, identity, role, display name, unavailable model, or tampered checkpoint or artifact fails before durable or live mutation with actionable secret-free diagnostics
5. Successful lifecycle mutations and checkpoints are atomically persisted under Runtime state and restored on restart; invalid persisted state fails closed
6. Dehydrate embeds a durable integrity-bound checkpoint in a portable freeze-dried agent artifact and migrate uses a two-phase checkpoint-freeze-write-commit sequence so failure cannot silently erase the source agent
7. Rehydrate verifies checkpoint and artifact integrity, identity continuity, destination provider/model availability, and conflict freedom before atomic destination admission
8. Focused command, API, persistence, checkpoint, roster, artifact, restart, and live Wuji proofs pass at the exact reviewed revision

## Dependencies

- Issue #589 exact implementation branch and governed Runtime v3 lifecycle surfaces
- Existing Runtime v3 TLS and ACIP write-token configuration
- Existing Ollama HTTP provider endpoint and installed gemma4:e4b-mlx model on Wuji

## Inputs

- GitHub issue #602
- adl/src/cli/csmctl_cmd.rs
- adl/src/cli/csm_runtime_v3_cmd.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/control/feeds.rs
- adl-runtime-kernel/src/bin/adl-runtime-kernel.rs
- docs/api/runtime-v3/v1/observatory.openapi.json

## Non Goals

- Multi-node placement or scheduling
- Model download or unrelated model startup
- Providers other than Ollama HTTP
- Static init-file agent entries
- Replacing Shepherd
- Changing #589 or Sprint 7
