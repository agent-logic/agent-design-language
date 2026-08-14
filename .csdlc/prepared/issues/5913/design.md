# Issue 5913 design

## Problem

The installed `adl-review` binary advertises read-only CodeFriend/CodeBuddy
review commands such as `code-review` and `verify-repo-contract`, but those
dispatch paths still route through the removed v1 tooling multiplexer. The
observable failure is the sunset diagnostic:

`the v1 tooling multiplexer was removed; use the independent C-SDLC v2 binaries`

This blocks Sprint 6 tooling closeout because operators can discover the
documented/repo-native review entrypoints, but cannot run the smallest
non-credentialed smoke proof.

## Scope

Repair only the `adl-review` review-tooling surface needed for a safe
read-only operational proof:

- `adl-review verify-repo-contract --review <markdown>` validates a repository
  review artifact without routing through `adl tooling`.
- `adl-review code-review --out <dir> --backend fixture --visibility read-only-repo`
  produces or routes to a deterministic local fixture proof without provider
  credentials.
- Unsupported lifecycle/runtime commands invoked through `adl-review` continue
  to fail closed with clear diagnostics.
- The focused regression script is updated so its proof no longer uses
  `adl tooling verify-repo-review-contract` as an oracle.

## Non-goals

- No provider credential execution and no live OpenAI/Anthropic/Gemini call.
- No lifecycle write path, PR publication, issue closeout, or C-SDLC card
  mutation through `adl-review`.
- No resurrection of the v1 tooling multiplexer or historical shell lifecycle
  wrappers.
- No broad CodeFriend product completion claim; this is a deterministic
  operational smoke route only.

## Design

Keep `adl-review` as a review-only command boundary. Move the two advertised
read-only review commands off the removed multiplexer and into small explicit
handlers:

1. `verify-repo-contract` parses `--review <path>`, reads the markdown review,
   and validates the required repository-review structure used by the tracked
   examples. It emits deterministic machine-readable success/failure text and
   returns non-zero for malformed review packets.
2. `code-review` accepts only the local deterministic fixture backend in this
   issue. It writes the fixture smoke output under the caller-provided `--out`
   directory and does not inspect, print, or require provider keys.
3. All commands that are not review tooling remain explicitly rejected from the
   `adl-review` surface.

The implementation should stay surgical. If the handlers outgrow the existing
CLI module, add one small review-command module rather than pushing provider,
lifecycle, or runtime concepts into the dispatcher.

## Validation

Focused proof is the updated compatibility script:

`bash adl/tools/test_adl_review_compatibility.sh`

The script must prove:

- help advertises only supported review behavior,
- `verify-repo-contract` succeeds on the good fixture and fails on a malformed
  fixture,
- `code-review` fixture mode produces deterministic local smoke artifacts,
- `adl-review` still rejects lifecycle/runtime command attempts,
- no proof route calls removed `adl tooling` multiplexer behavior.

Strict relevant Rust lint proof:

`cargo clippy --manifest-path adl/Cargo.toml --bin adl-review -- -D warnings`

## Risks

- Over-implementing provider-backed CodeFriend behavior would widen the issue.
- Keeping the old compatibility test oracle would mask the regression.
- Silently weakening `adl-review` command rejection would create a lifecycle
  bypass. The test must retain negative coverage for that boundary.
