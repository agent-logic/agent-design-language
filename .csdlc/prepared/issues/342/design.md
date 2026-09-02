# WP-24A current-repository preparation design

## Authority and denominator

Issue `agent-logic/agent-design-language#342` is the sole current issue authority and replaces legacy `danielbaustin/agent-design-language#5845`. The retained legacy lifecycle, branch, worktree references, Episode 001 package, audio, review, and evidence are immutable inputs, not current lifecycle authority.

The required denominator is ten complete review-ready episode packages plus integration proof. Current `origin/main` contains only the Episode 001 package directory and its retained final MP3/WAV artifacts. Even though legacy exact-head review recorded Episode 001 as an incremental checkpoint, this preparation makes no new proof claim about its freshness. Episodes 002 through 010 are absent. Therefore the current honest denominator is at most one preserved candidate package out of ten, nine absent packages, and incomplete integration proof; WP-24A is not implemented or publication-ready.

## Dependency graph

- `#342` produces terminal review-ready episode-package input for `#262`.
- `#51` is coordination only and consumes `#342` and closed preview issue `#19` as read-only upstream evidence.
- `#261` owns operator-approved show identity, final artwork/rights, canonical show metadata, and company-mailbox readiness.
- `#262` is serialized behind terminal `#261` and terminal `#342`; it alone owns production hosting, RSS/enclosure publication, byte-range and desktop/mobile production proof.
- Closed `#19` owns only the already-deployed unlinked preview route and retained deployment evidence; it is independent of episode production.

## Scope and collision policy

WP-24A may prepare and validate ten source episode packages and their local package manifests. It must consume the final `#261` identity/artwork packet without taking over that decision. It may prepare feed/enclosure source records for parity validation, but must not deploy or publish them; production route/storage mutation belongs to `#262`.

Current collision census:

- The preserved `codex/5845-v092-wp24a-readiness` branch is an ancestor of current `origin/main`; all legacy `.csdlc/issues/5845`, `.csdlc/evidence/5845`, Episode 001, audio, studio, feed, and launch-readiness artifacts remain preserved on main.
- The closed `#19` worktree has dirty lifecycle/publication records. Those exact `.csdlc/issues/19` and `.csdlc/publication/19.intent.json` paths are excluded from `#342` ownership and must not be touched.
- No current `#342`, `#51`, `#261`, or `#262` worktree/branch was found at preparation time. The `#261` artwork/metadata destination and `#262` feed/enclosure publication destinations remain semantic collision gates: before bind, an operator-approved exact path allocation must separate immutable episode-package inputs from child-owned launch outputs.
- Legacy podcast worktrees for `#5708` and `#5717` carry unrelated lifecycle dirt; all their lifecycle paths are excluded.

## Execution outline

1. Before bind, resolve the exact `#261` artwork/metadata input paths and `#262` publication-output paths; re-run the collision census.
2. Revalidate the preserved Episode 001 package at the exact future execution head without rewriting legacy evidence.
3. Produce complete packages 002 through 010 one checkpoint at a time; a checkpoint may claim only complete packages.
4. Run package, digest, ID3/artwork, feed/enclosure-parity, redaction, rights, and negative validators.
5. Obtain exact-head editorial/audio review for every claimed package and integrated ten-package proof.
6. Stop with local review-ready packages. Hand terminal package inputs to `#262`; do not deploy, submit, publish, or launch.

## Fail-closed gates

Stop before bind or edits if canonical identity drifts, `#261` or `#262` path allocation is unresolved, any active worktree owns an overlapping product path, the Episode 001 retained evidence cannot be revalidated, package completeness is less than the claimed denominator, rights/consent/redaction truth is missing, or any action would mutate deployment/provider state.
