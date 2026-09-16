use std::collections::BTreeMap;

const MODULES: [(&str, &str); 14] = [
    ("binding", include_str!("../src/commands/local/binding.rs")),
    ("cards", include_str!("../src/commands/local/cards.rs")),
    ("context", include_str!("../src/commands/local/context.rs")),
    (
        "failpoints",
        include_str!("../src/commands/local/failpoints.rs"),
    ),
    (
        "filesystem",
        include_str!("../src/commands/local/filesystem.rs"),
    ),
    ("intent", include_str!("../src/commands/local/intent.rs")),
    ("issue", include_str!("../src/commands/local/issue.rs")),
    (
        "lifecycle",
        include_str!("../src/commands/local/lifecycle.rs"),
    ),
    (
        "planning",
        include_str!("../src/commands/local/planning.rs"),
    ),
    ("results", include_str!("../src/commands/local/results.rs")),
    ("routing", include_str!("../src/commands/local/routing.rs")),
    ("storage", include_str!("../src/commands/local/storage.rs")),
    (
        "transactions",
        include_str!("../src/commands/local/transactions.rs"),
    ),
    (
        "worktree",
        include_str!("../src/commands/local/worktree.rs"),
    ),
];

#[test]
fn local_owner_remains_a_thin_acyclic_module_graph() {
    let facade = include_str!("../src/commands/local/mod.rs");
    assert!(
        facade.lines().count() <= 500,
        "local/mod.rs must remain a thin public-contract facade"
    );

    let ranks = BTreeMap::from([
        ("failpoints", 0_u8),
        ("filesystem", 0),
        ("planning", 0_u8),
        ("results", 0),
        ("storage", 1),
        ("worktree", 1),
        ("lifecycle", 2),
        ("transactions", 3),
        ("cards", 4),
        ("context", 4),
        ("binding", 5),
        ("issue", 5),
        ("intent", 6),
        ("routing", 7),
    ]);

    for (module, source) in MODULES {
        assert!(
            source.starts_with("//!"),
            "{module}.rs must name its responsibility with module documentation"
        );
        assert!(
            !source.contains("use super::*;"),
            "{module}.rs must declare its dependencies explicitly"
        );
        assert!(
            !source.contains("crate::commands::"),
            "{module}.rs must use the canonical super::<module> sibling path"
        );
        assert!(
            facade.contains(&format!("mod {module};"))
                || facade.contains(&format!("pub mod {module};")),
            "local/mod.rs must wire {module}.rs into the production owner"
        );

        let compact_source = source.split_whitespace().collect::<String>();
        for statement in source.split(';') {
            let Some((_, import)) = statement.rsplit_once("use ") else {
                continue;
            };
            let compact_import = import.split_whitespace().collect::<String>();
            if compact_import.starts_with("super::{") {
                for dependency in ranks.keys() {
                    assert!(
                        !compact_import.contains(dependency),
                        "{module}.rs must spell sibling imports as use super::<module>::..."
                    );
                }
            }
        }
        for dependency in ranks.keys() {
            if compact_source.contains(&format!("super::{dependency}")) {
                assert!(
                    ranks[dependency] < ranks[module],
                    "{module}.rs must not depend laterally or upward on {dependency}.rs"
                );
            }
        }
    }
}

#[test]
fn local_responsibilities_have_one_production_owner() {
    let owners = [
        ("planning", "fn validate_contract("),
        ("filesystem", "fn atomic_write("),
        ("failpoints", "fn local_transaction_failpoint("),
        ("results", "fn operational_result("),
        ("lifecycle", "fn inspect_lifecycle_issue_root("),
        ("storage", "fn persist_index("),
        ("worktree", "fn git_worktree_registration("),
        ("transactions", "fn recover_pending_local_transaction("),
        ("context", "fn validate_context("),
        ("cards", "fn edit_operational_cards("),
        ("issue", "fn initialize_operational_issue("),
        ("binding", "fn bind_operational_issue("),
        ("intent", "fn prepare_semantic("),
        ("routing", "fn execute_operational_local_route("),
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
