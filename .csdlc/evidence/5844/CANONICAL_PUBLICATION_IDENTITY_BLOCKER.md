# WP-24 Canonical Publication Identity Blocker

## Observed State

- Canonical code and push authority is `agent-logic/agent-design-language`.
- The shared `origin` fetch and push URL names the canonical repository.
- Branch `codex/5844-v092-wp24-article-series` has an explicit branch-local
  `pushRemote=origin` safeguard.
- Existing issue `#5844` remains on the preserved
  `danielbaustin/agent-design-language` tracker.
- The canonical cutover contract requires new branches and pull requests in
  the Agent Logic repository and qualified closure text:
  `Closes danielbaustin/agent-design-language#5844`.

## Typed Publication Conflict

The current `csdlc-publish` contract has one `repository` field. It requires
that value to match all of the following:

1. `.csdlc/issues/5844/index.json` repository identity;
2. the Git remote URL used for the push;
3. the repository queried for an existing or newly created pull request;
4. the observed base and head repository identities; and
5. the repository used by the closing-keyword validator.

The issue record correctly retains the legacy issue tracker, while canonical
code publication requires the Agent Logic repository. Therefore:

- a canonical publication request fails record identity matching; and
- a legacy publication request targets the prohibited legacy code repository.

## Safety Boundary

Do not:

- push WP-24 to `legacy-origin`;
- create or update another legacy pull request;
- hand-edit typed card or index repository identities;
- rewrite the legacy issue into a canonical issue number;
- bypass typed review or publication with an unrecorded GitHub mutation; or
- widen WP-24 into an implementation of publication tooling.

The already-created legacy PR `#5902` is retained as observed erroneous remote
state. This issue worktree will not mutate it further.

## Required Capability

Typed publication needs distinct, validated identities for:

- canonical code repository and Git remote;
- preserved issue-tracker repository and issue number; and
- qualified cross-repository closing linkage.

Publication can resume only after an approved typed migration or publisher
contract represents those identities without weakening exact-head review,
remote identity checks, or closing-linkage verification.

The required capability is tracked canonically as
`agent-logic/agent-design-language#3`.
