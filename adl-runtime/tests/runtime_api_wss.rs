use serde_json::Value;

const OPENAPI: &str = include_str!("../../docs/api/runtime-v3/v1/openapi.json");
const OBSERVATORY_OPENAPI: &str =
    include_str!("../../docs/api/runtime-v3/v1/observatory.openapi.json");
const HTML_OBSERVATORY_APP: &str = include_str!("../../demos/html-observatory/app.js");
const CSMCTL: &str = include_str!("../../CSMctl");
const RESTART_VALIDATOR: &str =
    include_str!("../../adl/tools/validate_v092_observatory_restart_reconnect.sh");

fn openapi() -> Value {
    serde_json::from_str(OPENAPI).expect("runtime OpenAPI must parse")
}

fn observatory_openapi() -> Value {
    serde_json::from_str(OBSERVATORY_OPENAPI).expect("observatory OpenAPI must parse")
}

fn path<'a>(document: &'a Value, route: &str) -> &'a Value {
    document["paths"]
        .get(route)
        .unwrap_or_else(|| panic!("{route} must be documented"))
}

fn response_codes(document: &Value, route: &str, method: &str) -> Vec<String> {
    path(document, route)[method]["responses"]
        .as_object()
        .unwrap_or_else(|| panic!("{method} {route} must have response codes"))
        .keys()
        .cloned()
        .collect()
}

#[test]
fn exposed_runtime_routes_are_all_covered_by_the_restart_validator() {
    let runtime = openapi();
    let observatory = observatory_openapi();
    let runtime_get_routes = [
        "/v1/health",
        "/v1/ready",
        "/v1/metrics",
        "/v1/openapi.json",
        "/v1/docs/",
        "/v1/observatory/openapi.json",
        "/v1/observatory/docs/",
    ];
    let observatory_get_routes = ["/v1/agents", "/v1/agents/{agent_id}", "/v1/observatory"];
    for route in runtime_get_routes {
        assert!(
            path(&runtime, route).get("get").is_some(),
            "{route} must remain a documented GET route"
        );
        assert!(
            RESTART_VALIDATOR.contains(route),
            "{route} must be exercised by the issue-owned restart validator"
        );
    }
    for route in observatory_get_routes {
        assert!(
            path(&observatory, route).get("get").is_some(),
            "{route} must remain a documented Observatory GET route"
        );
        let validator_route = route.replace("{agent_id}", "");
        assert!(
            RESTART_VALIDATOR.contains(&validator_route),
            "{route} must be exercised by the issue-owned restart validator"
        );
    }

    for route in ["/v1/control", "/v1/layer8/recipient-acknowledgement"] {
        assert!(
            path(&runtime, route).get("options").is_some(),
            "{route} must expose browser preflight"
        );
        assert!(
            path(&runtime, route).get("post").is_some(),
            "{route} must retain its signed POST contract"
        );
        assert!(
            RESTART_VALIDATOR.contains(route),
            "{route} must be probed without performing a signed mutation"
        );
    }

    assert!(
        response_codes(&runtime, "/v1/acip/ws", "get").contains(&"101".to_owned()),
        "ACIP WSS must document successful protocol upgrade"
    );
    assert!(
        response_codes(&observatory, "/v1/observatory/ws", "get").contains(&"101".to_owned()),
        "Observatory WSS must document successful protocol upgrade"
    );
    assert!(RESTART_VALIDATOR.contains("/v1/acip/ws"));
    assert!(RESTART_VALIDATOR.contains("/v1/observatory/ws"));
}

#[test]
fn html_observatory_live_mode_requires_the_full_runtime_read_surface() {
    for required in [
        "health_endpoint: \"/v1/health\"",
        "fetchRuntimeV3Health(base)",
        "fetchRuntimeV3Readiness(base)",
        "fetch(`${base}${config.observatory_endpoint}`",
        "response.status !== 200",
    ] {
        assert!(
            HTML_OBSERVATORY_APP.contains(required),
            "HTML Observatory must require real Runtime v3 read evidence: {required}"
        );
    }
}

#[test]
fn csmctl_observatory_serves_index_at_root_and_persists_runtime_target() {
    for required in [
        "if self.path in (\"\", \"/\"):",
        "self.path = \"/index.html\"",
        "OBSERVATORY_RUNTIME_BASE=%q",
        "OBSERVATORY_URL=%q",
        "OBSERVATORY_LAUNCH_WORKING_DIR",
        "load_observatory_state || true",
        "runtimeApiBase=$OBSERVATORY_RUNTIME_BASE",
    ] {
        assert!(
            CSMCTL.contains(required),
            "CSMctl must preserve simple root launch and Runtime target evidence: {required}"
        );
    }
    for required in [
        "observatory_restart_cmd_stale_url",
        "observatory_restart_cmd_stale_runtime_api_base",
    ] {
        assert!(
            RESTART_VALIDATOR.contains(required),
            "restart validator must prove CSMctl start reports the current Observatory URL after restart: {required}"
        );
    }
}
