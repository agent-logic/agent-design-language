use adl_runtime::distributed::polis_runtime::PolisCommand;

#[test]
fn legacy_direct_authority_command_shapes_fail_closed() {
    let caller_indexed_prepare = serde_json::json!({
        "operation": "prepare_authority",
        "prepare_log_index": 71,
        "authority": {"caller_selected": true}
    });
    let caller_indexed_finalize = serde_json::json!({
        "operation": "finalize_authority",
        "finalize_log_index": 72,
        "verified_authority_operation": {"caller_minted": true}
    });

    assert!(serde_json::from_value::<PolisCommand>(caller_indexed_prepare).is_err());
    assert!(serde_json::from_value::<PolisCommand>(caller_indexed_finalize).is_err());
}
