use std::collections::BTreeMap;

const MODULES: [(&str, &str); 11] = [
    ("binding", include_str!("../src/commands/local/binding.rs")),
    ("cards", include_str!("../src/commands/local/cards.rs")),
    ("context", include_str!("../src/commands/local/context.rs")),
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
        ("planning", 0_u8),
        ("storage", 0),
        ("lifecycle", 1),
        ("worktree", 1),
        ("transactions", 2),
        ("cards", 3),
        ("context", 3),
        ("binding", 4),
        ("issue", 4),
        ("intent", 5),
        ("routing", 6),
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
            facade.contains(&format!("mod {module};"))
                || facade.contains(&format!("pub mod {module};")),
            "local/mod.rs must wire {module}.rs into the production owner"
        );

        for dependency in ranks.keys() {
            if source.contains(&format!("super::{dependency}")) {
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
        ("lifecycle", "fn inspect_lifecycle_issue_root("),
        ("storage", "fn atomic_write("),
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
