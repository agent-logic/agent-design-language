# Public dependency advisory disposition — #1329

Scope: public ADL at baseline `d984020473275f6b490b9dd4839f73429c02bea0`.
This is a bounded follow-up to the original six-lockfile version audit, not an
exploitability assessment or release qualification.

## Compatible updates

| CLI locked package | Before | Candidate | Advisory source |
|---|---|---|---|
| anyhow | 1.0.100 | 1.0.103 | [RUSTSEC-2026-0190](https://rustsec.org/advisories/RUSTSEC-2026-0190.html) |
| bytes | 1.11.0 | 1.11.1 | [RUSTSEC-2026-0007](https://rustsec.org/advisories/RUSTSEC-2026-0007.html) |
| rustls | 0.23.43 | 0.23.46 | RUSTSEC-2026-0285, [0346](https://rustsec.org/advisories/RUSTSEC-2026-0346.html), 0348, 0349 |
| rustls-webpki | 0.103.9 | 0.103.16 | RUSTSEC-2026-0049, 0098, 0099, 0104, [0382](https://rustsec.org/advisories/RUSTSEC-2026-0382.html) |
| webpki-roots | 1.0.6 | 1.0.10 | [RUSTSEC-2026-0381](https://rustsec.org/advisories/RUSTSEC-2026-0381.html) |
| quinn-proto | 0.11.17 | 0.11.18 | RUSTSEC-2026-0338, 0340, [0343](https://rustsec.org/advisories/RUSTSEC-2026-0343.html) |

The 17 original RustSec records were independently retrieved from the authoritative
RustSec advisory database; none was withdrawn. The selected stable versions cover
the original affected ranges and satisfy existing parent version requirements.
Cargo generated the six updated version/checksum records. Unrelated resolver
changes were discarded: all other CLI package records, including tempfile's
getrandom and Windows dependency selections, retain their baseline bytes.

The first five packages are in the default CLI dependency graph. Its optional
HTTPS telemetry uses reqwest only when an endpoint is configured. Version matching
does not prove that an advisory's triggering conditions occur in ADL. Quinn is
absent from the default host graph; its retained optional lock entry is updated
without enabling HTTP/3. No claim covers every platform or feature configuration.

## Unsupported and maintenance decisions

Characterization's h2 0.3.27 comes through jsonschema's default HTTP resolver,
reqwest 0.11 and hyper 0.14. The advisory's patched h2 0.4.16 is outside those
parents' accepted range. A direct forced patch is not a compatible fix. The
HTTP resolver is now disabled with `default-features = false` and
`features = ["resolve-file"]`. This removes reqwest/hyper/h2 from the
characterization graph without upgrading the parent stack. The actual
`load_corpus` entrypoint rejects HTTP and HTTPS references with the explicit
disabled-feature diagnostic. Canonical fragment references and local file
references retain validation behavior; a file URI containing spaces and `#`
is tested with valid and invalid input. File references are not confined to
the corpus root; this is not a filesystem sandbox.

[unsafe-libyaml / RUSTSEC-2024-0449](https://rustsec.org/advisories/RUSTSEC-2024-0449.html)
is an informational unmaintained notice, with no patched version. It remains
active through serde_yaml in the CLI, legacy contracts and characterization.
Replacing the parser needs a deliberate YAML compatibility decision and proof;
retention acceptance is pending the owner. No security patch or removal is claimed.

## Validation status

The public CLI lane passed, followed by successful final locked graph resolution
with the original unrelated CLI dependency selections retained. All characterization
tests passed, including eight manifest tests. The final CLI graph contains the
updated TLS/HTTP dependencies and excludes Quinn; the characterization graph
excludes reqwest, hyper and h2. Independent review and exact-head native proof
are recorded separately in the issue delivery evidence.

The [repeat OSV result](evidence/1329/advisory-after.json) queries the same six
lockfiles as the original audit: CLI, legacy contracts, schema, UTS, v2 workspace
and characterization. The original baseline had 430 distinct registry versions
and eight matched versions. The candidate has 395 distinct versions after
removing unused HTTP dependencies; all 395 were queried on 2026-10-21 UTC.
Only unsafe-libyaml 0.2.11 matches, across three locks. Alias duplicates are
not counted as separate exploits. Lock SHA-256 identities bind this observation
to exact inputs. This is a dated version audit, not a claim of vulnerability
absence or exhaustive security review.
