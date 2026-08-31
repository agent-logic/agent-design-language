# Polis GCP Vertex AI issue-creation canary defects

This canary creates one GitHub issue for configuring Polis on GCP to use Vertex
AI after #528 is terminal.

Observed defects to retain for cutover:

1. The desired single Rust `csdlc` command is not yet the authoritative GitHub
   mutation route. Live issue creation still has to use the typed v2
   `csdlc-github-issue` owner.
2. The current v3 local binary shape was in flux during this run: the target is
   one `csdlc` binary with subcommands, but the merged construction line still
   previously exposed split `csdlc-v3-local` and `csdlc-v3-foundation` binaries.
3. A one-command operator path for `issue create --depends-on #528 --provider
   vertex-ai` does not yet exist; the issue body and dependency truth are still
   hand-authored into a typed request packet.
4. Running the authoritative typed v2 issue-create command from the dirty
   cutover-repair branch failed closed on stale owner-binary provenance:
   installed source set
   `28f37dd00d1ecc3677b6f3c95d8f9f40fc1c8ebe8af51f070f5a865acff6fbc1`
   differed from current
   `c037c878606ec86a98fc296ba7f0bf913cdc91b805fda657e4c2fd4be8a6d366`.
   The command correctly refused mutation, but this exposes the need for a
   clean one-command issue-start path that can isolate lifecycle writes from
   unrelated dirty implementation work.
5. A fresh clean worktree created from `origin/main` did not have the repo-local
   installed owner binaries at `.adl/bin/csdlc-v2`, so the expected typed
   command path was absent until an install was performed.
6. `csdlc-install install --destination .adl/bin/csdlc-v2` produced an
   install receipt whose relative destination failed later verification with
   `installed receipt identity is invalid`. Reinstalling with the absolute
   destination `/Volumes/FastWork/adl-worktrees/adl-canary-polis-gcp-vertex-ai/.adl/bin/csdlc-v2`
   repaired the receipt identity.
7. The prescribed reinstall required an already existing absolute
   `CARGO_TARGET_DIR`, turning first issue creation into an install/build
   sequence instead of one quick governed command.
8. During the canary, one `gh issue view` read was used for a dependency/status
   check before the operator clarified "don't use gh". The final tool must make
   typed C-SDLC readback as fast and obvious as `gh` so operators do not reach
   for raw GitHub reads.
9. `bash adl/tools/test_ci_path_policy.sh` is too broad for a quick
   cutover canary: it entered authoritative coverage/cache-warmup paths and
   wrote scratch targets under the system temp directory before being stopped.
   The cutover tool needs a focused, repo-contained canary lane for path-policy
   assertions instead of forcing broad coverage behavior during issue-start
   validation.
