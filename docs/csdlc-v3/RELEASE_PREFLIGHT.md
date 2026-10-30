# Native release candidate preflight

Issue #856 repairs the v0.92.1 version mismatch and the ceremony script's
retired v2 fallback. The release package family is now 0.92.1, including the
inherited ADL workspace packages and retained v2 compatibility owner.
`docs/milestones/v0.92.1/RELEASE_ARTIFACTS.json` inventories every package and
tracked lockfile. Native C-SDLC v3, standalone proof helpers, the transpiler demo
and AWS helper retain independent versions with explicit rationale. Historical
proof-helper locks and superseded workspace-member locks are retained; Cargo
uses the workspace root lockfile for workspace members. They are inventoried,
not presented as current release dependency resolution.

## Operator command for #526

From the exact clean candidate checkout, after installing the reviewed repair:

```sh
bash adl/tools/install_owner_binaries.sh --bin csdlc
bash adl/tools/release_ceremony.sh --request /absolute/path/to/release-preflight.json
```

The stable owner is `.adl/bin/native-v3/csdlc`. Missing owners fail; preflight
never builds or executes an owner from Cargo target and never falls back to v2.
The install wrapper hashes native source/manifest/lock provenance and leaves an
unchanged owner intact. Installation is a separate action before preflight.

The request is JSON with exactly these fields:

```json
{
  "repository": "agent-logic/agent-design-language",
  "version": "v0.92.1",
  "candidate_sha": "FULL_40_CHARACTER_COMMIT_SHA",
  "notes_path": "docs/milestones/v0.92.1/RELEASE_NOTES_v0.92.1.md",
  "notes_digest": "BLAKE3_HEX_OF_EXACT_NOTES_BYTES",
  "gate_path": "/absolute/path/to/native-release-gate.json",
  "gate_digest": "BLAKE3_HEX_OF_EXACT_GATE_BYTES"
}
```

The external gate has schema `csdlc.v3.release_gate.v1`, the same repository,
version, candidate SHA, notes path and notes digest, plus `inventory_digest`
(BLAKE3 of the exact release artifact inventory), `status: ready_for_preflight`,
and `evidence`: exactly three objects with `issue`, `path`, `digest` for #522,
#525 and #526. Each path must identify a distinct candidate-tracked file under
`docs/milestones/v0.92.1/`; each digest is BLAKE3 of those exact bytes. The
operator must select the actual reviewed remediation, planning review and
candidate evidence. Do not manufacture those records from issue status.

Store request and gate in resolved Git metadata or outside the checkout. The
gate is assembled **after** the proposed commit exists; embedding that commit's
SHA in its own tracked contents would create a circular identity. Changing the
candidate, notes, inventory or evidence requires a new explicit gate/request.
Missing or stale native selector/receipt authority suspends preflight.

A successful JSON report means **candidate consistency eligible**. The tool
checks evidence identity, not the semantic adequacy or authenticity of a local
operator-supplied gate. It always reports `release_authorized: false` and
`mutation_allowed: false`. It does not authenticate review by hashing prose.
The gate is neither a release authorization nor proof of release acceptance.
#526 still owns semantic review of #522/#525, exact candidate approval and any
separately authorized release operations. #833 evidence is untouched.

The old `--version`, dirty/SOR bypass and tag/release mutation flags are rejected
by this preflight wrapper. This intentionally removes the unsafe combined
preflight/publication path; it does not replace release publication tooling.
No tag, push, draft release or publication was performed by #856.

## Validation and observability

PVF: deterministic local native-owner contract proof, small CPU/Git fixture disk,
required for this repair. No provider, cloud or network mutation. Run:

```sh
cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test release_preflight
cargo test --locked --manifest-path csdlc-v3/Cargo.toml --test command_manifest
bash adl/tools/test_release_ceremony.sh
bash adl/tools/test_check_milestone_closed_issue_sor_truth.sh
```

The real CLI fixtures exercise eligible inputs and missing/stale identity,
notes, gate, manifest, lockfile and authority failures. Git refs and checkout
status are compared before/after each native call. The shell fixture proves
routing only; it does not substitute for native semantic proof. Structured
reports are stdout; rejection diagnostics are stderr with nonzero exit. Request
parse errors emit only stderr. No credentials, request bodies or gate paths are
logged. Compatibility observability log modes are not claimed by this route.
