# Versions and compatibility

Version numbers name different contracts. A milestone, package, wire schema and
installed product can have different versions without contradicting one another.

| Surface | Source baseline | What it means |
| --- | --- | --- |
| Public CLI | [`adl` 0.92.1](../adl/Cargo.toml) | The public command package; listed owner commands require separate installations. |
| Rust build | [Rust 1.92.0](../rust-toolchain.toml) | Repository toolchain and components; inspected packages declare Rust 1.92 minimum. |
| ADL documents | [Legacy document types](../adl-legacy-contracts/src/adl/types.rs), [example](../adl/examples/v0-3-fork-join-seq-run.adl.yaml) | The `adl_version` field is a document-format version, not the CLI release. The demonstrated document uses `0.3`. |
| Language design | [ADL 1.0 draft](../adl-spec/README.md) | Evolving specification; not a claim that every proposed runtime semantic is implemented. |
| Public schema and legacy packages | [`adl-schema`](../adl-schema/Cargo.toml), [`adl-legacy-contracts`](../adl-legacy-contracts/Cargo.toml), both 0.1.0 | Public Rust contracts; valid data does not establish Runtime admission. |
| UTS package | [`adl-uts` 0.1.0](../adl-uts/Cargo.toml) | Implements `uts.v1` and the declared `uts.v1.1` surface; see [package boundaries](specs/uts/PACKAGE.md). Package version is not a wire version. |
| ACC | [v1.0](specs/acc/ACC_V1.0_SPEC.md) and [v1.1 specification](specs/acc/ACC_V1.1_SPEC.md) | Public specifications/schemas. Runtime enforcement belongs to its separate owner; schemas alone grant no authority. |
| Language/compiler workspace | [`adl-v2` workspace](../adl-v2/Cargo.toml) | The selected language and compiler packages use workspace version 0.92.1; retained unselected trees are not validated by that selection. |
| Runtime, CodeFriend, Cognitive-SDLC | [Owner map](REPOSITORIES.md) | Installations have their own qualified version, artifact identity and admission requirements. Public CLI validation does not qualify them. |
| Milestones | [Roadmap](ROADMAP.md) | Planned and historical work; merged children do not by themselves establish a release. |

Use the exact revision and owner instructions associated with your installation.
The [canonical releases](https://github.com/agent-logic/agent-design-language/releases)
are the distribution record. Source tags do not promise attached binaries or
external package-registry publication. No support lifetime or backport guarantee
is implied here; [SECURITY.md](../SECURITY.md) owns the security support policy.
Report ordinary compatibility defects through the public issue templates with
CLI/package version, document/schema version and reproducible input. Do not include
credentials, private data or a sensitive vulnerability in a public report.
