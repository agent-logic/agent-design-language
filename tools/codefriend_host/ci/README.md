> Source ownership after the repository split: controller, registry, probe, unit files, and host manifest tests are delivered by [CodeFriend at e93a457](https://github.com/agent-logic/codefriend/tree/e93a457ad6325017627a66a2abe31a3376fa28fd/tools/codefriend_host), authenticated product artifact `11804671827`. ADL no longer supplies these local source files. The installation discussion below describes the installed layout and its separate deployment obligations. The archived original [`ci/smoke.py`](https://github.com/agent-logic/agent-design-language/blob/9707731d50f4209c07e3f7193d8be254ab6b9167/tools/codefriend_host/ci/smoke.py) and its typed interrupted-report fixture use the historical ADL CLI; they are historical obligations, not a qualified successor smoke command. Source qualification covers 36 original host methods plus one added owner method; it does not establish live systemd, website, provider, or host acceptance.

# Candidate Linux host artifacts (#1077)

The retired ADL `codefriend-host-candidate.yml` was a reusable lane selected by the former
`ci.yaml` pull-request classifier, with read-only
repository permission, explicit full PR-head checkout (or exact dispatched commit), pinned actions and Rust
1.92.0, Ubuntu 24.04 x86_64, and a 45-minute timeout. Its former standalone `workflow_dispatch` entry is historical; it is no longer an ADL entrypoint. The former main `adl-ci` aggregate
required success when selected and skip when unselected, rejecting failures and
cancellation. It does not dispatch workflows,
use AWS or provider credentials, publish releases, deploy, or invoke poweroff.
The binaries are built together using the declared **dev profile with debug
information disabled**. They are not mislabeled optimized release builds.

The job installs isolated copies of codefriend-server, codefriend-agent, and adl, then
runs 16 installed smoke cases: empty-registry startup/current embedded gateway
candidate, read-only quiescence, drain/resume ownership, retired attempt and old
instance rejection, restart, unauthenticated malformed-body denial, and actual
agent CLI acceptance/rejection of report fixtures, plus the installed ADL CodeFriend
Journey help entrypoint. All three binaries come from the same exact source commit.
The CLI help case establishes availability only, not a completed product journey. A bound loopback provider trap
must receive zero connections, and the durable operations directory stays empty.
The subprocess environment contains only explicit PATH/HOME values; provider,
GitHub and AWS credentials are not inherited. Both gateway instances must exit
cleanly through SIGINT; a timeout causes failure after bounded process cleanup.
Only a verified owned socket inode is removed after its process has exited.

The valid report fixture is a typed interrupted report with no results/source
payload and expiry 2100-01-01. Its precomputed digest uses native RunReport field
serialization and BLAKE3; changing `status` without updating its digest must fail.
This checks the actual verifier command, not a successful review/provider claim.
No third-party hashing package is installed just to construct a smoke fixture.

`manifest.py` checks that built, installed and tested executable SHA256s agree,
requires Linux amd64 ELF headers, exact clean source HEAD and Rust 1.92.0, and
records source tree, lock/input hashes, toolchain, declared build command/profile,
workflow SHA/ref, run ID/attempt and smoke proof. Artifact copying is create-only.
A tar archive preserves executable modes through artifact transport. The upload
step records its artifact ID and digest in the job summary. Fetchers
must independently authenticate GitHub run/job/artifact metadata and compare the
digest of each required binary before trusting downloaded bytes. **This is build evidence, not a
signed attestation.** Installer `--no-build` metadata alone is not provenance.
The manifest retains `ready_for_deployment:false`; systemd, actual product
journeys and deployed shutdown acceptance remain separate requirements.

Current owner source validation (select an authenticated CodeFriend checkout explicitly):

```
case "$CODEFRIEND_HOST_SOURCE_ROOT" in /*) ;; *) exit 2;; esac
cd "$CODEFRIEND_HOST_SOURCE_ROOT/tools/codefriend_host/ci"
python3 -B -m unittest -v test_manifest.BinaryProof
# Separate added owner contract, outside the six original methods:
python3 -B -m unittest -v test_manifest.InstalledContentContract
```

Historical smoke contract (not a current successor instruction): the original smoke required explicitly trusted installed binaries and a new private
output path. Its default is a real bounded binary execution, not a dry-run. A
macOS smoke validates script/CLI behavior but does not claim Linux artifact proof.
The original fixed fixture and smoke script remain in the pinned Git revision and retirement custody archive as historical build inputs, not customer data or current local commands.
