---
name: csdlc-v2-closeout
description: Record terminal truth and safely close out a green integrated issue.
---
Invoke `csdlc-closeout`. Fail closed on incomplete readiness, stale generation,
missing terminal evidence, a conflicting terminal receipt, or unsafe prune
scope.

`closeout` atomically retains the closed issue record and all six typed cards
under the repository's Git common directory before reporting success. This
receipt is the immediate terminal authority after an implementation PR merges.
Use `reconcile-terminal --request <request.json>` from a dedicated closeout
branch to materialize that authority into the tracked `.csdlc/issues/<issue>`
projection. Never patch the primary checkout or card Markdown directly.

`repair-sor-validation --request <request.json>` atomically replaces one exact
terminal SOR validation result under a distinct active repair authority. The
target must remain closed-out and claim-free, the authority must protect the
target issue path, and authority, target, receipt, and old-result identities
must all match. Replacement commands and evidence references must be portable;
the operation regenerates the tracked projection and retained receipt together
or rolls both back.

`digest-projection --issue <issue>` computes a byte-exact digest of every
regular file under a corrupt issue projection without accepting that projection
as valid lifecycle authority. `recover-corrupt-historical-merged --request
<request.json>` is the only recovery route for a parseable or unparseable
nonterminal projection whose canonical digest no longer verifies. It requires a
distinct active authority issue covering the target, the digest above as an
exact compare-and-swap, a canonical source projection pinned to an ancestor
commit of the merged PR head, explicit nonempty required-check names, the
repository-declared remote-review requirement, current closed-issue/merged-PR
evidence, exact typed review evidence,
and passing typed validation. The operation journals the complete issue tree,
requires current authored artifacts to equal the pinned source, and writes the
closed-out projection plus retained receipt atomically. Never use
`repair-identity` to normalize a corrupt projection.

`prune` requires closed-out canonical state and revalidates the same retained
receipt before removing the issue worktree.
