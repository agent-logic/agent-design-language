# Issue 41 design: actionable GitHub issue-read failures

## Decision

Keep the existing `GithubAction::IssueRead` request and successful
`GithubActionResult` unchanged. Add one issue-read-specific Octocrab error
classifier at the boundary where `read_issue_packet` observes GitHub. The
classifier returns stable `ErrorCode` variants and constructs its own bounded
diagnostic from the already-validated repository identity and issue number. It
must never interpolate the Octocrab error display, GitHub response body,
authorization header, token path, or token value.

The minimum failure taxonomy is:

| Observation | Typed code | CLI exit | Safe diagnostic |
| --- | --- | ---: | --- |
| HTTP 404 | `remote_not_found` | 69 | `GitHub issue owner/name#N was not found; verify the repository and issue number` |
| HTTP 401 | `remote_authentication` | 77 | Authentication failed while reading `owner/name#N` |
| HTTP 403, non-rate-limit | `remote_authorization` | 77 | Authorization failed while reading `owner/name#N` |
| HTTP 403 with GitHub rate-limit signal, or HTTP 429 | `remote_rate_limited` | 74 | GitHub rate limit prevented reading `owner/name#N` |
| HTTP 5xx | `remote_server` | 74 | GitHub server failure prevented reading `owner/name#N` |
| Octocrab service, Hyper, HTTP, or connection failure | `remote_transport` | 74 | Transport failure prevented reading `owner/name#N` |
| Any unclassified remote error | existing `remote_failure` | 74 | Generic bounded observation failure |

GitHub deliberately uses 404 for both an absent object and some inaccessible
private objects. The diagnostic therefore describes the observation as not
found and tells the operator to verify both the repository and issue identity;
it does not claim that the repository definitely exists.

For 403 classification, use only Octocrab's structured `GitHubError` status,
message, and documentation URL. A 403 is rate-limited only when those
structured fields contain a recognized GitHub rate-limit marker; otherwise it
is authorization. The emitted diagnostic never includes those source fields.

## Boundary and flow

1. `csdlc-github-issue` parses the existing typed request.
2. `execute_github_action` validates `owner/name` and the positive issue
   number, resolves the approved token source, and calls Octocrab.
3. Successful reads return the existing packet byte shape.
4. Failed reads pass only the Octocrab error plus validated repository and
   issue identity to the classifier.
5. The CLI serializes the existing `csdlc.error.v1` envelope on stdout and
   exits with the code owned by the classified `ErrorCode`; stderr remains
   empty unless stdout itself cannot be written.

## Proof design

Extend the existing loopback GitHub fixture in
`csdlc-v2/tests/gate_github_actions.rs`. Drive the real
`csdlc-github-issue run --request ...` binary against deterministic responses:

- 200 proves the successful issue packet remains unchanged;
- 404 proves `remote_not_found`, exit 69, and the exact safe repository/issue
  diagnostic;
- 401 and ordinary 403 prove authentication and authorization are not labeled
  not-found;
- rate-limit 403 and 429 prove `remote_rate_limited`;
- 500 proves `remote_server`;
- a loopback connection dropped before a response proves
  `remote_transport`.

Every failure assertion parses stdout as JSON, checks the exact code and exit
status, requires empty stderr, and scans both streams for the fake token, token
file path, and a deliberately sensitive response-body sentinel. No live GitHub
network call is part of this proof.

## Invariants

- Successful issue-read JSON is unchanged.
- Only issue reads receive the new contextual classification in this issue.
- Repository identity and issue number are the only remote-context values
  permitted in diagnostics.
- Authentication, authorization, rate limiting, transport, and server failures
  cannot be mislabeled as not-found.
- No token, token path, authorization header, raw response body, or Octocrab
  debug/display string reaches stdout or stderr.
- The control plane remains Rust-only and uses the existing Octocrab client and
  loopback fixture.

## Non-goals

- Repository migration or issue transfer.
- Issue create, update, comment, or close behavior changes.
- Pull-request observation changes.
- A broad GitHub client or retry-policy redesign.
- Live-network integration tests.

## Execution estimate

Use the native `small` planning profile: 7,200 elapsed seconds, 40,000 total
tokens, and 1,200 validation seconds. Declared validation lanes consume at
most 960 seconds and 9,000 tokens. Replan before widening beyond the four Rust
files and issue-local lifecycle artifacts.
