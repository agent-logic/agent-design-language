# Issue #674 design: New Polis citizen Welcome Package

## Decision

Deliver #674 as one versioned, agent-readable Markdown document. It is orientation material for new Axioma Polis residents, not executable configuration, a hidden system prompt, or a new Runtime capability.

## Audience and tone

The reader is a newly admitted model-backed resident agent. The package should be calm, welcoming, concise, and exact. It may use the established Polis vocabulary, but it must not imply personhood, independent authority, or capabilities the Runtime does not provide.

## Content contract

The document must explain:

1. where the agent is: Axioma Polis and the Runtime;
2. its configured identity and role;
3. the Polis Shepherd and how to ask for help;
4. other residents and Layer 8 governed communication;
5. conditional agent-to-agent initiation through Runtime policy;
6. forbidden actions and external-side-effect boundaries;
7. credential, privacy, and private-state boundaries;
8. safe refusal, clarification, and escalation paths.

Every capability statement must be conditional on current Runtime admission, communication eligibility, Layer 8 authority, and configured provider availability. The package grants no authority.

## Scope

- `docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md`
- one issue-owned documentation validator
- issue-local C-SDLC records

No Rust, Runtime API, OpenAPI, Observatory, provider, cloud, configuration, or live-service change is in scope.

## Validation

A lightweight deterministic validator checks required headings, conditional-governance language, forbidden-action boundaries, help paths, version marker, absence of host-specific paths or credential material, and Markdown hygiene. Independent exact-head documentation review remains required before publication.
