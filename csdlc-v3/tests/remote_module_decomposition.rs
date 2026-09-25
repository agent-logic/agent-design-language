use std::collections::BTreeMap;

const MODULES: [(&str, &str); 16] = [
    (
        "authority",
        include_str!("../src/commands/remote/authority.rs"),
    ),
    (
        "coordination",
        include_str!("../src/commands/remote/coordination.rs"),
    ),
    (
        "delivery",
        include_str!("../src/commands/remote/delivery.rs"),
    ),
    ("intent", include_str!("../src/commands/remote/intent.rs")),
    ("merge", include_str!("../src/commands/remote/merge.rs")),
    (
        "merge_linkage",
        include_str!("../src/commands/remote/merge_linkage.rs"),
    ),
    (
        "merge_retirement",
        include_str!("../src/commands/remote/merge_retirement.rs"),
    ),
    ("model", include_str!("../src/commands/remote/model.rs")),
    (
        "mutation",
        include_str!("../src/commands/remote/mutation.rs"),
    ),
    (
        "publication",
        include_str!("../src/commands/remote/publication.rs"),
    ),
    (
        "publication_observation",
        include_str!("../src/commands/remote/publication_observation.rs"),
    ),
    ("routing", include_str!("../src/commands/remote/routing.rs")),
    ("storage", include_str!("../src/commands/remote/storage.rs")),
    ("support", include_str!("../src/commands/remote/support.rs")),
    ("target", include_str!("../src/commands/remote/target.rs")),
    (
        "transport",
        include_str!("../src/commands/remote/transport.rs"),
    ),
];

fn dependency_style_error(source: &str, modules: &[&str]) -> Option<String> {
    let compact_source = source.split_whitespace().collect::<String>();
    let normalized_absolute_paths = compact_source.replace(['{', '}'], "");
    if normalized_absolute_paths.contains("crate::commands::remote") {
        return Some("absolute remote-module paths are forbidden".into());
    }
    for statement in source.split(';') {
        let Some((_, import)) = statement.rsplit_once("use ") else {
            continue;
        };
        let compact_import = import.split_whitespace().collect::<String>();
        let normalized_import = compact_import.replace(['{', '}'], "");
        if normalized_import.starts_with("crateas")
            || normalized_import.starts_with("crate::selfas")
            || normalized_import.starts_with("crate::commandsas")
            || normalized_import.starts_with("crate::commands::selfas")
            || normalized_import.starts_with("superas")
            || normalized_import.starts_with("super::selfas")
        {
            return Some("aliases of crate and super roots are forbidden".into());
        }
        let imported_identifiers = import
            .split(|character: char| !character.is_ascii_alphanumeric() && character != '_')
            .filter(|identifier| !identifier.is_empty())
            .collect::<Vec<_>>();
        for dependency in modules {
            let references_sibling = imported_identifiers.contains(dependency);
            if references_sibling && compact_import.starts_with("super::{") {
                return Some(format!(
                    "sibling {dependency} must not use a braced super import"
                ));
            }
            if references_sibling
                && normalized_import.starts_with("super")
                && !compact_import.starts_with(&format!("super::{dependency}::"))
            {
                return Some(format!(
                    "sibling {dependency} must use its canonical super path"
                ));
            }
        }
    }
    None
}

fn direct_sibling_dependencies<'a>(source: &str, modules: &'a [&str]) -> Vec<&'a str> {
    let compact_source = source.split_whitespace().collect::<String>();
    modules
        .iter()
        .copied()
        .filter(|dependency| {
            compact_source
                .match_indices(&format!("super::{dependency}"))
                .any(|(start, matched)| {
                    compact_source[start + matched.len()..]
                        .chars()
                        .next()
                        .is_none_or(|next| !next.is_ascii_alphanumeric() && next != '_')
                })
        })
        .collect()
}

#[test]
fn remote_owner_remains_a_thin_acyclic_module_graph() {
    let facade = include_str!("../src/commands/remote/mod.rs");
    assert!(
        facade.lines().count() <= 100,
        "remote/mod.rs must remain a thin public-contract facade"
    );

    let ranks = BTreeMap::from([
        ("model", 0_u8),
        ("support", 1),
        ("authority", 2),
        ("delivery", 2),
        ("merge_linkage", 2),
        ("storage", 2),
        ("target", 3),
        ("publication", 3),
        ("transport", 3),
        ("coordination", 4),
        ("publication_observation", 4),
        ("merge_retirement", 4),
        ("merge", 5),
        ("mutation", 6),
        ("routing", 7),
        ("intent", 8),
    ]);
    let module_names = ranks.keys().copied().collect::<Vec<_>>();
    let declared = facade
        .lines()
        .filter_map(|line| {
            let line = line.trim().strip_prefix("pub ").unwrap_or(line.trim());
            line.strip_prefix("mod ")?.strip_suffix(';')
        })
        .filter(|name| *name != "tests")
        .collect::<std::collections::BTreeSet<_>>();
    let inventoried = MODULES
        .iter()
        .map(|(name, _)| *name)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        declared, inventoried,
        "all production modules must be inventoried"
    );
    assert_eq!(
        module_names
            .iter()
            .copied()
            .collect::<std::collections::BTreeSet<_>>(),
        inventoried
    );

    for (module, source) in MODULES {
        assert!(
            source.starts_with("//!"),
            "{module}.rs must name its responsibility with module documentation"
        );
        assert!(
            !source.contains("use super::*;"),
            "{module}.rs must declare its module dependencies explicitly"
        );
        assert_eq!(
            dependency_style_error(source, &module_names),
            None,
            "{module}.rs must use canonical super::<module> sibling imports"
        );
        assert!(
            source.lines().count() <= 1_000,
            "{module}.rs must not become a replacement remote god module"
        );
        assert!(
            facade.contains(&format!("mod {module};"))
                || facade.contains(&format!("pub mod {module};")),
            "remote/mod.rs must wire {module}.rs into the production owner"
        );

        for dependency in direct_sibling_dependencies(source, &module_names) {
            assert!(
                ranks[dependency] < ranks[module],
                "{module}.rs must not depend laterally or upward on {dependency}.rs"
            );
        }
    }
}

#[test]
fn alternate_sibling_import_forms_fail_closed() {
    let modules = ["routing", "storage"];
    for rejected in [
        "use super::{routing::dispatch_operational_remote};",
        "use crate::commands::remote::routing::dispatch_operational_remote;",
        "use crate::{commands::remote::routing::dispatch_operational_remote};",
        "use crate::commands::{remote::routing::dispatch_operational_remote};",
        "use crate::{commands::{remote::routing::dispatch_operational_remote}};",
        "use crate as root; use root::commands::remote::routing::dispatch_operational_remote;",
        "use crate::{self as root}; use root::commands::remote::routing::dispatch_operational_remote;",
        "use crate::commands as cmd; use cmd::remote::routing::dispatch_operational_remote;",
        "use crate::commands::{self as cmd}; use cmd::remote::routing::dispatch_operational_remote;",
        "use crate::{commands::{self as cmd}}; use cmd::remote::routing::dispatch_operational_remote;",
        "use super as parent; use parent::routing::dispatch_operational_remote;",
        "use super::{self as parent}; use parent::routing::dispatch_operational_remote;",
        "use super::routing as routed;",
    ] {
        assert!(
            dependency_style_error(rejected, &modules).is_some(),
            "alternate sibling import unexpectedly accepted: {rejected}"
        );
    }
    assert_eq!(
        direct_sibling_dependencies(
            "use super::routing::dispatch_operational_remote as routed;",
            &modules,
        ),
        vec!["routing"]
    );
}

#[test]
fn remote_responsibilities_have_one_production_owner() {
    let owners = [
        ("model", "pub struct RemoteRouteRequest"),
        ("support", "fn github_mutation_operation_digest("),
        ("storage", "fn persist_json_create_new("),
        ("authority", "fn verify_canonical_v3_authority("),
        ("delivery", "pub(crate) fn deliver("),
        ("merge_linkage", "pub fn merge_linkage_query("),
        ("transport", "fn github_mutation_invocation("),
        ("model", "pub(super) const COORDINATION_CONTRACT_PREFIX"),
        ("publication", "pub fn prepare_remote_publication_route("),
        ("merge", "fn execute_inner("),
        ("mutation", "pub fn stage_github_mutation("),
        ("routing", "pub fn dispatch_operational_remote("),
        ("intent", "pub fn pending_operations("),
        (
            "publication_observation",
            "pub fn observe_ready_publication(",
        ),
    ];

    for (owner, symbol) in owners {
        let matching_modules = MODULES
            .iter()
            .filter_map(|(module, source)| source.contains(symbol).then_some(*module))
            .collect::<Vec<_>>();
        assert_eq!(
            matching_modules,
            vec![owner],
            "{symbol} must have exactly one cohesive production owner"
        );
    }
}

// PVF: deterministic architecture contract regression; local strings only,
// required owner guard, negligible resources, no external effects.
#[test]
fn sibling_dependencies_match_complete_identifiers() {
    let modules = ["merge", "merge_linkage", "merge_retirement"];
    for source in [
        "use super::merge_retirement::*;",
        "super::merge_retirement::present(path)",
    ] {
        assert_eq!(
            direct_sibling_dependencies(source, &modules),
            vec!["merge_retirement"]
        );
    }
    assert_eq!(
        direct_sibling_dependencies("use super::merge_linkage::query;", &modules),
        vec!["merge_linkage"]
    );
    assert!(direct_sibling_dependencies(
        "super::merge_retirement_extra::run(); super::merge2::run();",
        &modules
    )
    .is_empty());
    assert_eq!(
        direct_sibling_dependencies(
            "use super::merge::*; super::merge_retirement::run();",
            &modules
        ),
        vec!["merge", "merge_retirement"]
    );
}
