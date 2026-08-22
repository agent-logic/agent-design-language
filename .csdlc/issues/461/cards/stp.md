# Structured Task Prompt

Template: 1.0.0

Issue: 461

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Remove duplicated lifecycle TLS command inputs and preserve the existing Runtime lifecycle behavior through config-only authority.

## Deliverables

- config-only lifecycle soak implementation
- Guardian config localization repair
- focused Rust and executable lifecycle regression coverage

## Acceptance

1. lifecycle soak rejects the removed TLS path flags
2. certificate chain, private key, and trust roots are loaded only from Runtime init configuration
3. configured TLS files are validated fail closed without leaking path or key material
4. Guardian fixture places TLS paths only in generated configuration
5. HTTPS and WSS lifecycle proof passes with config-owned TLS
6. issue #268 can consume the merged fix without a TLS argv surface

## Dependencies

- current Runtime v3 init configuration schema
- issue #268 paid AWS qualification waits for this merge

## Inputs

- adl-runtime/src/bin/adl-runtime-lifecycle-soak.rs
- adl/tools/validate_v092_runtime_guardian_lifecycle.sh
- infra/runtime-v3/runtime-init.toml

## Non Goals

- certificate issuance or rotation
- public DNS configuration
- CloudFormation provisioning
- issue #269
