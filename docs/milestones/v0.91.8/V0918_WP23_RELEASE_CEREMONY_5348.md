# v0.91.8 WP-23 Release Ceremony

## Authority

Issue `#5348` is the final v0.91.8 work package. It owns release documentation,
the annotated `v0.91.8` tag, the published GitHub release, and closure of sprint
umbrella `#5595`. It contains no product implementation or hidden remediation.

## Entry Truth

- WP-22 PR #5811 merged as `703ee31f2c02bb6c8fda7d6bc51ff7963075132e`.
- WP-21 and WP-21A terminal truth is present on current main.
- The #5809 publication-base supplement is included in this packet without
  changing the original WP-21 execution-time evidence.
- No `v0.91.8` tag or GitHub release existed before ceremony execution.

## Release Artifacts

- `RELEASE_PLAN_v0.91.8.md`
- `RELEASE_NOTES_v0.91.8.md`
- `MILESTONE_CHECKLIST_v0.91.8.md`
- `.csdlc/evidence/5362/dependency-verification-publication-base.v1.json`
- the published GitHub release for tag `v0.91.8`

## Execution

After this packet merges, execute `adl/tools/release_ceremony.sh` at the exact
merge commit in its recommended split-step sequence:

1. create the annotated tag;
2. push the tag;
3. create the draft release from the final notes;
4. publish the release;
5. verify tag and release identity;
6. close `#5595` with the exact merge, tag, and release URL.

The script's typed local closeout gate is circular for WP-23 itself before the
tag exists. The ceremony may explicitly use `--skip-sor-gate` only for the
post-merge release mutation; typed #5348 closeout follows immediately and is
not release evidence for any earlier gate.

## Validation

This is a documentation and release-state change. Required proof is limited to
Markdown/link checks, JSON/YAML parsing, `git diff --check`, exact Git identity,
the release script preflight, and live tag/release verification. No Rust build,
Clippy, coverage, or broad test suite is required.

## Non-Claims

- No v0.92 issue is activated or executed.
- No future feature is treated as implemented by virtue of its handoff packet.
- The ceremony does not alter product code or historical review evidence.
