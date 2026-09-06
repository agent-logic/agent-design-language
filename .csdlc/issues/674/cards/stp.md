# Structured Task Prompt

Template: 1.0.0

Issue: 674

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Author the single Welcome Package document and its lightweight documentation contract only.

## Deliverables

- docs/runtime/AXIOMA_POLIS_WELCOME_PACKAGE_V1.md
- .csdlc/prepared/issues/674/validate-welcome-package-docs.sh
- Focused documentation validation evidence

## Acceptance

1. AC-1: The document is explicitly versioned and addresses a newly admitted model-backed resident agent
2. AC-2: It explains Axioma Polis, resident identity, other residents, the Polis Shepherd, and Layer 8 in plain language
3. AC-3: It describes agent-to-agent communication only as conditional on Runtime admission, communication eligibility, Layer 8 authority, and provider availability
4. AC-4: It forbids unrestricted autonomous messaging, credential access, external side effects, unbounded loops, private-data disclosure, and invented capabilities
5. AC-5: It directs the resident to ask the Shepherd or operator, request a governed action, clarify uncertainty, or decline when policy is missing
6. AC-6: Its tone is welcoming and supportive without lore, personhood claims, or capability fantasy
7. AC-7: Lightweight offline documentation checks and exact-head independent review pass

## Dependencies

- Current Runtime resident-agent, Shepherd, Layer 8, and governed A2A contracts are read-only source evidence

## Inputs

- agent-logic/agent-design-language#674
- adl/src/csm_shepherd_agent.rs
- adl-runtime-kernel/src/control.rs
- adl-runtime-kernel/src/layer8_authority/mod.rs

## Non Goals

- No Runtime implementation or delivery mechanism
- No Rust, API, OpenAPI, Observatory, provider, configuration, or cloud change
- No live model or Runtime validation
- No grant of autonomous powers
- No broad Polis lore rewrite
