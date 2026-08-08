# Issue 41 design: actionable GitHub issue-read failures

## Decision

Keep the existing `GithubAction::IssueRead` request and successful
`GithubActionResult` unchanged. Add one issue-read-specific Octocrab error
classifier only in the explicit `GithubAction::IssueRead` match arm: that arm
maps an error returned by `read_issue_packet`, while the shared helper itself
and every create/update/comment/close reconciliation call remain unchanged.
The classifier returns stable `ErrorCode` variants and constructs its own
bounded diagnostic from the already-validated repository identity and issue
number. It must never interpolate the Octocrab error display, GitHub response
body, authorization header, token path, or token value.

The minimum failure taxonomy is:

| Observation | Typed code | CLI exit | Safe diagnostic |
| --- | --- | ---: | --- |
| HTTP 404 | `remote_not_found` | 69 | `GitHub issue owner/name#N was not found or is inaccessible; verify the repository, issue number, and token access` |
| HTTP 401 | `remote_authentication` | 77 | Authentication failed while reading `owner/name#N` |
| HTTP 403, non-rate-limit | `remote_authorization` | 77 | Authorization failed while reading `owner/name#N` |
| HTTP 403 with GitHub rate-limit signal, or HTTP 429 | `remote_rate_limited` | 74 | GitHub rate limit prevented reading `owner/name#N` |
| HTTP 5xx | `remote_server` | 74 | GitHub server failure prevented reading `owner/name#N` |
| Octocrab service, Hyper, HTTP, or connection failure | `remote_transport` | 74 | Transport failure prevented reading `owner/name#N` |
| Any unclassified remote error | existing `remote_failure` | 74 | Generic bounded observation failure |

GitHub deliberately uses 404 for both an absent object and some inaccessible
private objects. The diagnostic therefore says `not found or inaccessible`
and tells the operator to verify repository identity, issue number, and token
access; it does not claim that the repository definitely exists.

For 403 classification, use only Octocrab's structured `GitHubError` status,
message, and documentation URL. Normalize the message with ASCII lowercase and
trim only leading/trailing whitespace. The closed rate-limit allowlist is:

- message starts with `api rate limit exceeded` (the primary-limit form may
  append an actor or address);
- message equals `you have exceeded a secondary rate limit. please wait a few
  minutes before you try again.`;
- or a parsed HTTPS URL on host `docs.github.com` has path exactly
  `/rest/using-the-rest-api/rate-limits-for-the-rest-api`;
- legacy documentation is accepted only for path
  `/rest/overview/resources-in-the-rest-api` with fragment exactly
  `rate-limiting` or `secondary-rate-limits`.

HTTP 429 is always rate-limited. An HTTP 403 that matches none of this closed
allowlist is authorization; malformed URLs and merely containing words such as
`rate` or `limit` do not qualify. The emitted diagnostic never includes the
source message or URL.

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
- every allowlisted primary/secondary rate-limit 403 form plus 429 proves
  `remote_rate_limited`, while near-match 403 text and URLs remain
  `remote_authorization`;
- 500 proves `remote_server`;
- a loopback connection dropped before a response proves
  `remote_transport`.

One non-read action whose reconciliation readback returns 404 must retain the
existing `remote_failure` behavior. This proves the contextual mapper is wired
only to an explicit `IssueRead` and not into the shared readback helper.

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
