# Candidate Linux host artifacts (#1077)

`codefriend-host-candidate.yml` is an ordinary pull-request lane with read-only
repository permission, explicit full PR-head checkout, pinned actions and Rust
1.92.0, Ubuntu 24.04 x86_64, and a 45-minute timeout. It does not dispatch workflows,
use AWS or provider credentials, publish releases, deploy, or invoke poweroff.
The binaries are built together using the declared **dev profile with debug
information disabled**. They are not mislabeled optimized release builds.

The job installs isolated copies of codefriend-server and codefriend-agent, then
runs 15 installed smoke cases: empty-registry startup/current embedded gateway
candidate, read-only quiescence, drain/resume ownership, retired attempt and old
instance rejection, restart, unauthenticated malformed-body denial, and actual
agent CLI acceptance/rejection of report fixtures. A bound loopback provider trap
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
server digest before trusting downloaded bytes. **This is build evidence, not a
signed attestation.** Installer `--no-build` metadata alone is not provenance.
The manifest retains `ready_for_deployment:false`; systemd, actual product
journeys and deployed shutdown acceptance remain separate requirements.

Local validation:

```
PYTHONDONTWRITEBYTECODE=1 python3 -m unittest discover -s tools/codefriend_host/ci -p 'test_*.py' -v
```

Run smoke only against explicitly trusted installed binaries and a new private
output path. Its default is a real bounded binary execution, not a dry-run. A
macOS smoke validates script/CLI behavior but does not claim Linux artifact proof.
The fixed fixture and smoke script are versioned build inputs, not customer data.
