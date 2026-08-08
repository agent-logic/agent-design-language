# Structured Task Prompt

Template: 1.0.0

Issue: 41

Repository: agent-logic/agent-design-language

Card: stp

Status: ready

## Task

Add a bounded issue-read failure taxonomy and safe diagnostic mapper plus focused real-CLI loopback proof; do not redesign other GitHub operations.

## Deliverables

- Stable issue-read remote failure codes and exit mapping
- Contextual redacted repository/issue diagnostics
- Focused loopback tests for success, 404, authentication, authorization, rate limit, server, and transport behavior
- Exact JSON, exit-code, stdout/stderr, and secret-redaction assertions

## Acceptance

1. AC-1: HTTP 404 issue reads return remote_not_found with exit 69 and an actionable owner/name#N diagnostic
2. AC-2: HTTP 401 and ordinary HTTP 403 are typed as authentication or authorization failures and never as not-found
3. AC-3: Rate-limit HTTP 403 or 429 is typed separately from authorization, transport, and not-found
4. AC-4: HTTP 5xx and connection failures are typed as server or transport failures and never as not-found
5. AC-5: Successful issue-read request and result JSON remain unchanged
6. AC-6: Failure stdout is valid csdlc.error.v1 JSON, exit codes are stable, and stderr remains empty unless stdout writing fails
7. AC-7: Token value, token path, authorization material, sensitive response-body sentinel, and raw Octocrab error text are absent from stdout and stderr
8. AC-8: Focused real-binary loopback tests and strict Clippy pass with no unresolved review findings

## Dependencies

- agent-logic/agent-design-language#41
- Octocrab 0.53 structured GitHubError status/message/documentation fields
- Existing CSDLC_V2_TEST_GITHUB_API_BASE loopback contract
- Existing csdlc.error.v1 serialization and ErrorCode exit mapping

## Inputs

- csdlc-v2/src/error.rs
- csdlc-v2/src/github.rs
- csdlc-v2/src/bin/csdlc-github-issue.rs
- csdlc-v2/tests/gate_github_actions.rs
- csdlc-v2/src/runner_preflight.rs

## Non Goals

- Repository migration or issue transfer
- Issue creation, update, comment, or close semantic changes
- Pull-request observation changes
- Broad GitHub client or retry refactoring
- Live-network failure tests
