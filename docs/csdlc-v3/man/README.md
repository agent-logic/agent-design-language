# Native operator manual

Operational manuals and request examples are delivered by the authenticated
Cognitive-SDLC owner package. This ADL directory retains package lookup guidance
and its isolated lookup/refusal test; the historical 34-page source suite,
18 example fixtures and their generator inputs moved out of ADL in #1223.
Their exact original bytes remain in Git and retirement custody.

The selected package determines its command and page inventory. The earlier
11808226974 package had 42 pages whose actual manual lookup and rendering passed;
that does not execute request examples or establish native lifecycle authority.
See [issue intent commands](../INTENT_COMMANDS.md) for repository routing.

## Installed owner manuals

Operational lookup uses the manuals delivered with the exact admitted C-SDLC
package, not this ADL source tree. Native `package-install` already includes the
roff pages under `installed/resources/docs/csdlc-v3/man/man1`; it does not create
`share/man`. No additional manual installation or copy is needed.

After a reviewed native installation succeeds, use its actual `executable` output:

```sh
# Set this to the exact executable returned by the successful native installation.
CSDLC_EXECUTABLE=/absolute/package/installed/bin/csdlc
bash adl/tools/install_csdlc_man_pages.sh --stable-bin-dir "$(dirname "$CSDLC_EXECUTABLE")"
# Use the exact MANPATH export printed above, then:
man -w csdlc-workflow
man csdlc-workflow
```

The retained `install_owner_binaries.sh --bin csdlc` compatibility path performs
only this lookup, including on a no-op request. It never refreshes manuals from
ADL source, copies a supplied source executable, or replaces the selected owner.
Public `adl` and `adl-process` installation remains separate. The explicitly
restricted `csdlc-transition` source route is unchanged by this manual cutover.

For first installation, an already authorized native owner accepts
`csdlc package-install --request REVIEWED_REQUEST.json`. The strict request has
`schema: csdlc.v3.portable_package_install.v1`, `repository_root`, `artifact_path`,
and `observation`. Observation requires `pull_request`, `producer_issue`,
`workflow_run`, `artifact`, `candidate_revision`, `merge_revision`,
`default_branch`, `workflow_attempt`, and `review_attestation_sha256`.
Use the original authenticated artifact ZIP and real independent-review
attestation; do not invent these values. This is a repository-bound first-import
route: it requires the actual primary consumer checkout, preserves existing
activation receipts and shared installs, authenticates current remote provenance,
and requires successful installed-child admission before reporting verification.
It is not a generic overwrite or shared-selector replacement command.

Missing/stale admission remains a native recovery/install boundary. A manifest,
manual directory, successful lookup, or local source build alone does not grant
operational authority. New package revisions deliver new paired manuals atomically;
do not edit an installed package to refresh documentation.

## Owner authoring and historical examples

Manual authoring and renderer tests belong to
[Cognitive-SDLC](https://github.com/agent-logic/cognitive-sdlc/tree/fa5e22e818f9eb5478faac298877f188492cd879/docs/csdlc-v3/man).
Use that owner's source and validation instructions; do not regenerate private
manual sources or compile the retired C-SDLC crate in ADL.

The 18 original request examples are byte-identical in the verified owner
resource set. They are fixtures, not approved operational targets. Resource
presence and manual rendering do not execute a request or authorize a live
GitHub operation. New owner commands and documented policy changes do not
establish behavioral equivalence with the historical ADL manual.

The retained ADL lookup contract is checked with:

```sh
bash docs/csdlc-v3/man/test_install.sh
TMPDIR="$PWD/target" bash adl/tools/test_owner_binary_install.sh
```

These local fixture checks do not install or activate an owner. Actual platform
results belong in their evidence packet; a macOS result is not Linux proof.
