# Offline release assembly (#1077)

`assemble.py` creates a review packet from exact clean Git candidates and two
explicit prebuilt Linux x86-64 executables. It neither builds nor executes those
binaries, installs packages, deploys, creates config/secrets, calls AWS, nor
fetches remote Git objects. Both pinned commits must already exist locally.
The host implementation, units, proxy configuration and release tools must be
committed before assembly; untracked host files are not publication inputs.

```sh
python3 tools/codefriend_host/release/assemble.py \
  --adl /approved/checkouts/adl \
  --adl-revision "$ADL_REVISION" \
  --website /approved/checkouts/website \
  --website-revision "$WEBSITE_REVISION" \
  --gateway /approved/linux-build/codefriend-server \
  --verifier /approved/linux-build/adl \
  --output /approved/releases/beta-candidate
```

Set each revision to its reviewed 40-character commit and replace paths with
explicit approved local inputs. Source candidates must have no tracked changes
or untracked files. Output must not exist. A failed run leaves its partial output
for inspection; it never deletes an existing release or silently retries.

Output contains `bin/codefriend-server`, `bin/adl` (the report-verifier CLI), a
standalone website Git checkout pinned detached at the requested commit, and
`host/` controller, systemd and proxy inputs. Terraform is excluded from runtime
assembly. Website clone has no remote URL and copies no ignored/untracked local
state. No `node_modules` is copied or inferred. The website's required dependency
installation remains an explicit installer step: run `npm ci --ignore-scripts`
against the pinned lockfile in an approved build environment with Node 24, record
its resolved dependency evidence, then install the resulting reviewed tree.
Do not run npm or mutate source during service startup.

`manifest.json` records both exact source revisions and SHA-256 of every packet
file, including Git metadata. It intentionally says `ready_for_activation:false`
and `binary_authentication:pending_installer_verification`. ELF format and
architecture checks plus local checksums **do not authenticate a binary's build
candidate**. A malicious or incorrect executable can have a valid ELF header.
The existing owner installer writes source-hash/platform metadata, but its
`--no-build` path can copy a separately supplied executable without proving that
executable came from that source. Do not elevate that receipt into attestation.

Before deployment, an installer must verify binaries against authenticated exact
candidate build evidence, record expected binary hashes and builder/toolchain,
verify gateway startup's embedded candidate against the pinned ADL revision,
and exercise the report verifier's installed CLI against the same build record.
Do not execute untrusted packet binaries merely to obtain a self-reported version.
Retain that evidence separately; package preparation never marks it complete.

On the authorized host, install the entire reviewed release root-owned, with
no service/group/other write access to source, Git metadata or dependencies;
keep private runtime state/config outside it. Recompute manifest hashes before
and after transfer. Run website provenance preflight as the service UID; then
validate proxy, units, real dependency availability and installed Linux behavior
as described in sibling instructions. Assembly alone is not deployment acceptance.

Local validation: `python3 -m py_compile tools/codefriend_host/release/assemble.py`
and `python3 -m unittest discover -s tools/codefriend_host/release -p 'test_*.py' -v` prove argument/input/architecture guards,
not real Linux build provenance. Actual candidate assembly awaits committed host
inputs and authenticated Linux artifacts.

To recheck every manifest-listed byte and reject extra files before installation:

```sh
python3 tools/codefriend_host/release/verify_packet.py /approved/releases/beta-candidate
```

This validates integrity against the supplied manifest, not the authenticity of
that manifest. Retain its hash in the authenticated release evidence separately.
Dependency installation changes packet contents and therefore requires a new
installer-owned final manifest and review; do not rewrite this preparation
manifest to hide a mismatch.

## Authorized installed-release modes

The preparation packet root is private mode0700. After integrity/authenticity
checks, the installer must explicitly normalize the **secret-free release only**
so the `codefriend` service account can traverse and read it. With `release`
set to the approved canonical installed release path, before adding dependencies:

```sh
sudo chown -R root:root "$release"
sudo find "$release" -type d -exec chmod 0755 '{}' +
sudo find "$release" -type f -exec chmod 0644 '{}' +
sudo chmod 0755 "$release/bin/codefriend-server" "$release/bin/adl"
sudo chmod 0755 "$release/host/idle_stop.py"
sudo -u codefriend /usr/bin/node --input-type=module -e 'const module = await import(process.argv[1]); console.log(module.websiteCandidate())' "$release/website/app/host-control.mjs"
```

Git metadata is included in the directory/file readability normalization. Run
these commands only against the inspected secret-free packet; private runtime
state/config remain outside it under0700directories/0600files. Install reviewed
Node dependencies afterward, root-owned, preserving dependency executable bits
while removing group/other write permission; do not flatten their executable
modes with the source-file normalization. Repeat the service-UID preflight after
dependency installation and before starting any service. The returned revision
must equal the reviewed website revision. Integrity validation checks bytes and
nonregular objects, not final installation modes; owner/mode and installed
service checks remain mandatory installer evidence.
