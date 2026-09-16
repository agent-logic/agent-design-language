use std::collections::BTreeMap;

const MODULES: [(&str, &str); 12] = [
    (
        "authority",
        include_str!("../src/commands/remote/authority.rs"),
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
    ("model", include_str!("../src/commands/remote/model.rs")),
    (
        "mutation",
        include_str!("../src/commands/remote/mutation.rs"),
    ),
    (
        "publication",
        include_str!("../src/commands/remote/publication.rs"),
    ),
    ("routing", include_str!("../src/commands/remote/routing.rs")),
    ("storage", include_str!("../src/commands/remote/storage.rs")),
    ("support", include_str!("../src/commands/remote/support.rs")),
    (
        "transport",
        include_str!("../src/commands/remote/transport.rs"),
    ),
];

fn direct_sibling_dependencies<'a>(source: &str, modules: &'a [&str]) -> Vec<&'a str> {
    let compact_source = source.split_whitespace().collect::<String>();
    modules
        .iter()
        .copied()
        .filter(|dependency| compact_source.contains(&format!("super::{dependency}")))
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
        ("publication", 3),
        ("transport", 3),
        ("merge", 4),
        ("mutation", 5),
        ("routing", 6),
        ("intent", 7),
    ]);
    let module_names = ranks.keys().copied().collect::<Vec<_>>();

    for (module, source) in MODULES {
        assert!(
            source.starts_with("//!"),
            "{module}.rs must name its responsibility with module documentation"
        );
        assert!(
            !source.contains("use super::*;"),
            "{module}.rs must declare its module dependencies explicitly"
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
fn remote_responsibilities_have_one_production_owner() {
    let owners = [
        ("model", "pub struct RemoteRouteRequest"),
        ("support", "fn github_mutation_operation_digest("),
        ("storage", "fn persist_json_create_new("),
        ("authority", "fn verify_canonical_v3_authority("),
        ("delivery", "pub(crate) fn deliver("),
        ("merge_linkage", "pub fn merge_linkage_query("),
        ("transport", "fn github_mutation_invocation("),
        ("publication", "pub fn prepare_remote_publication_route("),
        ("merge", "fn execute_inner("),
        ("mutation", "pub fn stage_github_mutation("),
        ("routing", "pub fn dispatch_operational_remote("),
        ("intent", "pub fn pending_operations("),
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
