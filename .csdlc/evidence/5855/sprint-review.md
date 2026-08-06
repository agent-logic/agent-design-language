# Sprint 2 Review: Runtime, Observatory, Polis, And Protocol

## Review State

- Sprint issue: `#5855`
- Status: `in progress`
- Review boundary: coordination and dependency truth only
- Prepared revision: `ccce8f96145e57d9461d843a83169b05d40a07cf`

This record is intentionally incomplete until every child has terminal issue,
PR, validation, review, and typed lifecycle evidence. Waiting or open states are
not completion evidence.

## Current Gate Review

| Child | Current state | Gate review | Next proving event |
|---|---|---|---|
| `#5800` | open, prepared, unbound | `#5801` is terminal and ancestral; typed binding still waits for `#5896` to migrate the legacy bound/null-topology record | `#5896` terminal and ancestral, then current-generation doctor and bind pass |
| `#5820` | open | launch gate satisfied; child lifecycle remains separate from this coordination lane | child bind and issue-owned execution evidence |
| `#5821` | open | no completion claim reviewed | child terminal architecture/security gate evidence |
| `#5832` | open | remains downstream of `#5821` and the separate `#5862` implementation reconciliation | stable terminal protocol/WSS contract evidence |
| `#5795` | open, prepared, implementation-gated | waits for terminal `#5800`, `#5820`, and `#5832`; current design correctly separates deterministic tests from real nondeterministic inference | dependency requalification plus production extension-point inventory |
| `#5837` | open | waits for terminal `#5820`, `#5832`, and WP-18 readiness | dependency and consumer integration evidence |

## Prepared Findings

### Issue #5800

- Certificate reuse must validate time bounds, SAN identity, key permissions,
  and Rustls compatibility before preserving or replacing the active pair.
- Trust installation, verification, reissue, and issue-created trust removal
  require explicit operator-visible outcomes.
- Browser proof must use a pinned repository-native Playwright route and must
  reject certificate interstitials and TLS errors.
- Independent trusted curl proof must cover the Observatory page and Runtime
  health, readiness, and feed endpoints.
- The Observatory and Runtime listeners must present one supported localhost
  identity without falling back to plaintext or deleting the last valid pair.

### Issue #5795

- Real local inference is nondeterministic and cannot retain a deterministic
  Shepherd execution classification.
- The dependency heads must expose production assembly, governed admission,
  Runtime response projection, and Observatory WSS extension points before the
  child binds; otherwise its typed ownership/design boundary must be revised.
- The production-path test and browser validator must prove the configured
  local adapter, correlation identity, truthful `real_local_model`
  classification, timeout/cancellation behavior, and post-failure usability.
- `gemma4:12b-mlx` is only a prepared candidate. Availability and callability
  must be verified at execution time, with no silent cloud or model fallback.

## Integration Review Checklist

- [ ] Trusted local certificate identity is shared by ports 8765 and 20997.
- [ ] Runtime launch, retry, timeout, cancellation, and recovery behavior is terminally proved.
- [ ] Distributed Runtime and protocol contracts are terminal and reconciled.
- [ ] Shepherd real-model admission and browser correlation are proved without retained private content.
- [ ] Observatory and Unity consumers use the final versioned API/WSS contract.
- [ ] Authentication, logging, redaction, and OpenTelemetry claims match retained evidence.
- [ ] Every child has exact-head review, merged PR evidence, typed terminal truth, and current ancestry.

## Residual Risks

- `#5896` is an active external lifecycle migration and currently prevents a
  truthful `#5800` bind.
- Child write sets may change after dependencies land; any collision with
  Runtime startup, TLS, admission, or protocol ownership must collapse to
  serial execution.
- No sprint completion, integrated Runtime/Observatory proof, or local-model
  success is claimed by this interim review.
