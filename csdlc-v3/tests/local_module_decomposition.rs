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

fn dependency_style_error(source: &str, modules: &[&str]) -> Option<String> {
    let compact_source = source.split_whitespace().collect::<String>();
    let normalized_absolute_paths = compact_source.replace(['{', '}'], "");
    if normalized_absolute_paths.contains("crate::commands") {
        return Some("absolute local-module paths are forbidden".into());
    }
    for statement in source.split(';') {
        let Some((_, import)) = statement.rsplit_once("use ") else {
            continue;
        };
        let compact_import = import.split_whitespace().collect::<String>();
        let normalized_import = compact_import.replace(['{', '}'], "");
        if normalized_import.starts_with("crateas")
            || normalized_import.starts_with("crate::selfas")
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
        .filter(|dependency| compact_source.contains(&format!("super::{dependency}")))
        .collect()
}

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
    let module_names = ranks.keys().copied().collect::<Vec<_>>();

    for (module, source) in MODULES {
        assert!(
            source.starts_with("//!"),
            "{module}.rs must name its responsibility with module documentation"
        );
        assert!(
            !source.contains("use super::*;"),
            "{module}.rs must declare its dependencies explicitly"
        );
        assert_eq!(
            dependency_style_error(source, &module_names),
            None,
            "{module}.rs must use the canonical super::<module> sibling path"
        );
        assert!(
            facade.contains(&format!("mod {module};"))
                || facade.contains(&format!("pub mod {module};")),
            "local/mod.rs must wire {module}.rs into the production owner"
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
        "use super::{routing::execute};",
        "use crate::commands::local::routing::execute;",
        "use crate::{commands::local::routing::execute};",
        "use crate::commands::{local::routing::execute};",
        "use crate::{commands::{local::routing::execute}};",
        "use crate as root; use root::commands::local::routing::execute;",
        "use crate::{self as root}; use root::commands::local::routing::execute;",
        "use super as parent; use parent::routing::execute;",
        "use super::{self as parent}; use parent::routing::execute;",
        "use super::routing as routed;",
    ] {
        assert!(
            dependency_style_error(rejected, &modules).is_some(),
            "alternate import unexpectedly accepted: {rejected}"
        );
    }
    assert_eq!(
        direct_sibling_dependencies("use super::routing::execute as routed;", &modules),
        vec!["routing"]
    );
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
