# Issue 761 denominator proof

Run `python3 adl/tools/skills/repo-packet-builder/tests/test_denominators.py -v`.
PVF lane: packet-denominator-contract. Proof role: deterministic offline Python
routing and integrity contracts. Resource profile: small CPU/local temporary Git;
no network, providers or Rust build. Release gate: no.

The exact issue #520 fixture is reconstructed from base
`f0a011a5c59d46c763d669f69a10308b3f870ba4` to candidate
`c24f8fa65ce445b03ce6cd69007307291d78b60c`, sorted as one path per LF-terminated
line: 5,481 paths, SHA-256
`59c4c5de57d5aac07549a97bf508c00cbc2b5234e63f984a2eece6a8e07acb33`.
Tests stub line counts for this historical routing fixture, not file categories.
A separate real temporary Git CLI test proves diff scoping and validator exit codes.
The manifest-heavy fixture proves dependency records cannot starve mandatory lanes.
Negative tests cover empty assignments, wrong categories, bad hashes, incorrect
exclusions, omitted evidence, and a resealed incomplete source denominator.

These tests prove deterministic routing and integrity, not semantic review coverage.
