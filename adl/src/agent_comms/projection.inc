#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum AcipProtobufFieldKindV1 {
    Scalar,
    Message,
    Enum,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipProtobufFieldProjectionV1 {
    pub field_name: String,
    pub json_pointer: String,
    pub field_number: u32,
    pub kind: AcipProtobufFieldKindV1,
    pub type_ref: String,
    pub repeated: bool,
    pub required_in_json_schema: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipProtobufMessageProjectionV1 {
    pub rust_type: String,
    pub json_schema_version: String,
    pub protobuf_message: String,
    pub fields: Vec<AcipProtobufFieldProjectionV1>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipProtobufEnumProjectionV1 {
    pub rust_type: String,
    pub protobuf_enum: String,
    pub values: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, JsonSchema)]
#[serde(deny_unknown_fields)]
pub struct AcipProtobufProjectionProfileV1 {
    pub schema_version: String,
    pub profile_id: String,
    pub protobuf_syntax: String,
    pub protobuf_package: String,
    pub json_consumption_posture: String,
    pub websocket_consumption_posture: String,
    pub messages: Vec<AcipProtobufMessageProjectionV1>,
    pub enums: Vec<AcipProtobufEnumProjectionV1>,
    pub integration_refs: Vec<String>,
    pub non_claims: Vec<String>,
}

pub fn acip_protobuf_projection_profile_v1_schema_json() -> Result<String> {
    serde_json::to_string_pretty(&schema_for!(AcipProtobufProjectionProfileV1))
        .context("serialize ACIP protobuf projection profile v1 schema")
}

pub fn acip_protobuf_projection_profile_v1() -> AcipProtobufProjectionProfileV1 {
    AcipProtobufProjectionProfileV1 {
        schema_version: ACIP_PROTOBUF_PROJECTION_PROFILE_SCHEMA_VERSION.to_string(),
        profile_id: "acip-a2a-protobuf-projection-v1".to_string(),
        protobuf_syntax: "proto3".to_string(),
        protobuf_package: "adl.acip.v1".to_string(),
        json_consumption_posture:
            "json_projection_primary_until_binary_wire_activation_is_separately_proven"
                .to_string(),
        websocket_consumption_posture:
            "websocket_text_json_frames_for_v0_91_7_loopback_binary_frames_deferred_to_transport_issue"
                .to_string(),
        messages: vec![
            message_projection(
                "AcipMessageEnvelopeV1",
                ACIP_MESSAGE_SCHEMA_VERSION,
                "AcipMessageEnvelope",
                vec![
                    field("schema_version", 1, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("message_id", 2, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("conversation_id", 3, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("timestamp_utc", 4, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("monotonic_order", 5, AcipProtobufFieldKindV1::Scalar, "uint64", false, true),
                    field("sender", 6, AcipProtobufFieldKindV1::Message, "AcipAddress", false, true),
                    field("recipient", 7, AcipProtobufFieldKindV1::Message, "AcipAddress", false, true),
                    field("intent", 8, AcipProtobufFieldKindV1::Enum, "AcipIntent", false, true),
                    field("visibility", 9, AcipProtobufFieldKindV1::Enum, "AcipVisibility", false, true),
                    field("trace_requirement", 10, AcipProtobufFieldKindV1::Enum, "AcipTraceRequirement", false, true),
                    field("content", 11, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("payload_refs", 12, AcipProtobufFieldKindV1::Message, "AcipPayloadRef", true, true),
                    field("artifact_refs", 13, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                    field("attachments", 14, AcipProtobufFieldKindV1::Message, "AcipAttachmentRef", true, true),
                    field("authority_scope", 15, AcipProtobufFieldKindV1::Message, "AcipAuthorityScope", false, false),
                    field("correlation_id", 16, AcipProtobufFieldKindV1::Scalar, "string", false, false),
                    field("prior_message_id", 17, AcipProtobufFieldKindV1::Scalar, "string", false, false),
                ],
            ),
            message_projection(
                "AcipInvocationContractV1",
                ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                "AcipInvocationContract",
                vec![
                    field("schema_version", 1, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("invocation_id", 2, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("conversation_id", 3, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("causal_message_id", 4, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("caller", 5, AcipProtobufFieldKindV1::Message, "AcipAddress", false, true),
                    field("caller_class", 6, AcipProtobufFieldKindV1::Enum, "AcipCallerClass", false, false),
                    field("target", 7, AcipProtobufFieldKindV1::Message, "AcipAddress", false, true),
                    field("route_class", 8, AcipProtobufFieldKindV1::Enum, "AcipRouteClass", false, false),
                    field("intent", 9, AcipProtobufFieldKindV1::Enum, "AcipIntent", false, true),
                    field("purpose", 10, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("input_refs", 11, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                    field("constraints", 12, AcipProtobufFieldKindV1::Message, "AcipInvocationConstraints", false, true),
                    field("expected_outputs", 13, AcipProtobufFieldKindV1::Message, "AcipExpectedOutput", true, true),
                    field("stop_policy", 14, AcipProtobufFieldKindV1::Message, "AcipInvocationStopPolicy", false, true),
                    field("authority_scope", 15, AcipProtobufFieldKindV1::Message, "AcipAuthorityScope", false, true),
                    field("decision_event_ref", 16, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("response_channel", 17, AcipProtobufFieldKindV1::Message, "AcipResponseChannel", false, true),
                    field("trace_requirement", 18, AcipProtobufFieldKindV1::Enum, "AcipTraceRequirement", false, true),
                ],
            ),
            message_projection(
                "AcipInvocationEventV1",
                ACIP_INVOCATION_EVENT_SCHEMA_VERSION,
                "AcipInvocationEvent",
                vec![
                    field("schema_version", 1, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("invocation_id", 2, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("conversation_id", 3, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("causal_message_id", 4, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("caller", 5, AcipProtobufFieldKindV1::Message, "AcipAddress", false, true),
                    field("target", 6, AcipProtobufFieldKindV1::Message, "AcipAddress", false, true),
                    field("contract_ref", 7, AcipProtobufFieldKindV1::Scalar, "string", false, false),
                    field("contract_sha256", 8, AcipProtobufFieldKindV1::Scalar, "string", false, false),
                    field("decision_event_ref", 9, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("input_refs", 10, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                    field("output_refs", 11, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                    field("status", 12, AcipProtobufFieldKindV1::Enum, "AcipInvocationStatus", false, true),
                    field("stop_reason", 13, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("refusal_code", 14, AcipProtobufFieldKindV1::Scalar, "string", false, false),
                    field("failure_code", 15, AcipProtobufFieldKindV1::Scalar, "string", false, false),
                    field("evidence_refs", 16, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                    field("trace_requirement", 17, AcipProtobufFieldKindV1::Enum, "AcipTraceRequirement", false, true),
                ],
            ),
            message_projection(
                "AcipA2aAdapterBoundaryV1",
                ACIP_A2A_ADAPTER_SCHEMA_VERSION,
                "AcipA2aAdapterBoundary",
                vec![
                    field("schema_version", 1, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("adapter_id", 2, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("purpose", 3, AcipProtobufFieldKindV1::Scalar, "string", false, true),
                    field("identity_claim", 4, AcipProtobufFieldKindV1::Message, "AcipA2aAgentCardClaim", false, true),
                    field("capability_mappings", 5, AcipProtobufFieldKindV1::Message, "AcipA2aCapabilityMapping", true, true),
                    field("trust_classification", 6, AcipProtobufFieldKindV1::Message, "AcipA2aTrustClassification", false, true),
                    field("invoke_boundary", 7, AcipProtobufFieldKindV1::Message, "AcipA2aInvokeBoundary", false, true),
                    field("transport_boundary", 8, AcipProtobufFieldKindV1::Message, "AcipA2aTransportBoundary", false, true),
                    field("trace_evidence_refs", 9, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                    field("non_claims", 10, AcipProtobufFieldKindV1::Scalar, "string", true, true),
                ],
            ),
        ],
        enums: vec![
            enum_projection("AcipIntentV1", "AcipIntent", &[
                "conversation",
                "consultation",
                "invocation_setup",
                "review_request",
                "coding_request",
                "delegation",
                "negotiation",
            ]),
            enum_projection("AcipInvocationStatusV1", "AcipInvocationStatus", &[
                "requested",
                "completed",
                "refused",
                "failed",
                "partial",
            ]),
            enum_projection("AcipVisibilityV1", "AcipVisibility", &[
                "private", "shared", "public",
            ]),
            enum_projection("AcipTraceRequirementV1", "AcipTraceRequirement", &[
                "none", "summary", "full",
            ]),
            enum_projection("AcipCallerClassV1", "AcipCallerClass", &[
                "workflow_agent",
                "external_agent",
                "citizen_proxy",
                "group_relay",
            ]),
            enum_projection("AcipRouteClassV1", "AcipRouteClass", &[
                "local_only",
                "cross_boundary_deferred",
            ]),
            enum_projection("AcipA2aTrustClassV1", "AcipA2aTrustClass", &[
                "naked", "guest", "citizen",
            ]),
        ],
        integration_refs: vec![
            "#4658 ACIP schema/protobuf projection".to_string(),
            "#4900 WebSocket carrier substrate selection".to_string(),
            "#4659 WebSocket transport path consumes this profile without claiming binary wire activation".to_string(),
            "docs/milestones/v0.91.7/features/ACIP_A2A_PROTOBUF_RESIDUALS_v0.91.7.md".to_string(),
        ],
        non_claims: vec![
            "This profile does not introduce prost-generated Rust types or protobuf wire encoding.".to_string(),
            "This profile does not activate binary WebSocket frames.".to_string(),
            "This profile does not grant external-agent trust, authentication, TLS, reconnect, or cross-polis transport authority.".to_string(),
        ],
    }
}

pub fn validate_acip_protobuf_projection_profile_v1(
    profile: &AcipProtobufProjectionProfileV1,
) -> Result<()> {
    if profile.schema_version != ACIP_PROTOBUF_PROJECTION_PROFILE_SCHEMA_VERSION {
        return Err(anyhow!(
            "ACIP protobuf projection profile requires schema_version '{}'",
            ACIP_PROTOBUF_PROJECTION_PROFILE_SCHEMA_VERSION
        ));
    }
    validate_id(&profile.profile_id, "profile_id")?;
    if profile.protobuf_syntax != "proto3" {
        return Err(anyhow!("ACIP protobuf projection profile must use proto3"));
    }
    if profile.protobuf_package != "adl.acip.v1" {
        return Err(anyhow!(
            "ACIP protobuf projection profile must bind protobuf_package to 'adl.acip.v1'"
        ));
    }
    require_projection_text(
        &profile.json_consumption_posture,
        "json_projection_primary",
        "json_consumption_posture",
    )?;
    require_projection_text(
        &profile.websocket_consumption_posture,
        "text_json_frames",
        "websocket_consumption_posture",
    )?;
    require_projection_ref(&profile.integration_refs, "#4658")?;
    require_projection_ref(&profile.integration_refs, "#4900")?;
    require_projection_ref(&profile.integration_refs, "#4659")?;
    require_projection_non_claim(&profile.non_claims, "wire encoding")?;
    require_projection_non_claim(&profile.non_claims, "binary WebSocket")?;

    if profile.messages.len() < 4 {
        return Err(anyhow!(
            "ACIP protobuf projection profile must cover message, invocation, event, and A2A surfaces"
        ));
    }
    let mut messages = BTreeSet::new();
    let enum_type_refs = profile
        .enums
        .iter()
        .map(|projection| projection.protobuf_enum.clone())
        .collect::<BTreeSet<_>>();
    for message in &profile.messages {
        validate_message_projection(message, &enum_type_refs)?;
        if !messages.insert(message.rust_type.clone()) {
            return Err(anyhow!(
                "ACIP protobuf projection profile contains duplicate message projection '{}'",
                message.rust_type
            ));
        }
    }
    for required in [
        "AcipMessageEnvelopeV1",
        "AcipInvocationContractV1",
        "AcipInvocationEventV1",
        "AcipA2aAdapterBoundaryV1",
    ] {
        if !messages.contains(required) {
            return Err(anyhow!(
                "ACIP protobuf projection profile missing required projection '{}'",
                required
            ));
        }
    }

    let mut enums = BTreeSet::new();
    for projection in &profile.enums {
        validate_id(&projection.rust_type, "enums[].rust_type")?;
        validate_id(&projection.protobuf_enum, "enums[].protobuf_enum")?;
        if projection.values.is_empty() {
            return Err(anyhow!("enums[].values must not be empty"));
        }
        let mut values = BTreeSet::new();
        for value in &projection.values {
            validate_id(value, "enums[].values[]")?;
            if !values.insert(value.clone()) {
                return Err(anyhow!("enums[].values[] must not contain duplicates"));
            }
        }
        if !enums.insert(projection.rust_type.clone()) {
            return Err(anyhow!(
                "ACIP protobuf projection profile contains duplicate enum projection '{}'",
                projection.rust_type
            ));
        }
    }
    Ok(())
}

pub fn validate_acip_protobuf_projection_against_json_schemas_v1(
    profile: &AcipProtobufProjectionProfileV1,
) -> Result<()> {
    validate_acip_protobuf_projection_profile_v1(profile)?;
    for message in &profile.messages {
        match message.rust_type.as_str() {
            "AcipMessageEnvelopeV1" => validate_projection_against_schema(
                message,
                ACIP_MESSAGE_SCHEMA_VERSION,
                &serde_json::to_value(schema_for!(AcipMessageEnvelopeV1))?,
            )?,
            "AcipInvocationContractV1" => validate_projection_against_schema(
                message,
                ACIP_INVOCATION_CONTRACT_SCHEMA_VERSION,
                &serde_json::to_value(schema_for!(AcipInvocationContractV1))?,
            )?,
            "AcipInvocationEventV1" => validate_projection_against_schema(
                message,
                ACIP_INVOCATION_EVENT_SCHEMA_VERSION,
                &serde_json::to_value(schema_for!(AcipInvocationEventV1))?,
            )?,
            "AcipA2aAdapterBoundaryV1" => validate_projection_against_schema(
                message,
                ACIP_A2A_ADAPTER_SCHEMA_VERSION,
                &serde_json::to_value(schema_for!(AcipA2aAdapterBoundaryV1))?,
            )?,
            other => {
                return Err(anyhow!(
                    "ACIP protobuf projection profile cannot validate unknown rust_type '{}'",
                    other
                ))
            }
        }
    }
    Ok(())
}

fn message_projection(
    rust_type: &str,
    json_schema_version: &str,
    protobuf_message: &str,
    fields: Vec<AcipProtobufFieldProjectionV1>,
) -> AcipProtobufMessageProjectionV1 {
    AcipProtobufMessageProjectionV1 {
        rust_type: rust_type.to_string(),
        json_schema_version: json_schema_version.to_string(),
        protobuf_message: protobuf_message.to_string(),
        fields,
    }
}

fn field(
    field_name: &str,
    field_number: u32,
    kind: AcipProtobufFieldKindV1,
    type_ref: &str,
    repeated: bool,
    required_in_json_schema: bool,
) -> AcipProtobufFieldProjectionV1 {
    AcipProtobufFieldProjectionV1 {
        field_name: field_name.to_string(),
        json_pointer: format!("/{field_name}"),
        field_number,
        kind,
        type_ref: type_ref.to_string(),
        repeated,
        required_in_json_schema,
    }
}

fn enum_projection(
    rust_type: &str,
    protobuf_enum: &str,
    values: &[&str],
) -> AcipProtobufEnumProjectionV1 {
    AcipProtobufEnumProjectionV1 {
        rust_type: rust_type.to_string(),
        protobuf_enum: protobuf_enum.to_string(),
        values: values.iter().map(|value| value.to_string()).collect(),
    }
}

fn validate_message_projection(
    message: &AcipProtobufMessageProjectionV1,
    enum_type_refs: &BTreeSet<String>,
) -> Result<()> {
    validate_id(&message.rust_type, "messages[].rust_type")?;
    validate_id(&message.protobuf_message, "messages[].protobuf_message")?;
    validate_non_empty(
        &message.json_schema_version,
        "messages[].json_schema_version",
    )?;
    if message.fields.is_empty() {
        return Err(anyhow!("messages[].fields must not be empty"));
    }
    let mut field_names = BTreeSet::new();
    let mut field_numbers = BTreeSet::new();
    for field in &message.fields {
        validate_id(&field.field_name, "messages[].fields[].field_name")?;
        validate_non_empty(&field.json_pointer, "messages[].fields[].json_pointer")?;
        if field.json_pointer != format!("/{}", field.field_name) {
            return Err(anyhow!(
                "protobuf field '{}' must bind json_pointer to the same top-level JSON field",
                field.field_name
            ));
        }
        if field.field_number == 0 {
            return Err(anyhow!("protobuf field numbers must be positive"));
        }
        validate_id(&field.type_ref, "messages[].fields[].type_ref")?;
        if field.kind == AcipProtobufFieldKindV1::Enum && !enum_type_refs.contains(&field.type_ref)
        {
            return Err(anyhow!(
                "protobuf enum field '{}' references undeclared enum '{}'",
                field.field_name,
                field.type_ref
            ));
        }
        if !field_names.insert(field.field_name.clone()) {
            return Err(anyhow!(
                "message projection '{}' contains duplicate field '{}'",
                message.rust_type,
                field.field_name
            ));
        }
        if !field_numbers.insert(field.field_number) {
            return Err(anyhow!(
                "message projection '{}' contains duplicate field number {}",
                message.rust_type,
                field.field_number
            ));
        }
    }
    Ok(())
}

fn validate_projection_against_schema(
    message: &AcipProtobufMessageProjectionV1,
    expected_json_schema_version: &str,
    schema: &JsonValue,
) -> Result<()> {
    if message.json_schema_version != expected_json_schema_version {
        return Err(anyhow!(
            "projection '{}' must bind json_schema_version to '{}'",
            message.rust_type,
            expected_json_schema_version
        ));
    }
    let properties = schema
        .get("properties")
        .or_else(|| schema.get("schema").and_then(|schema| schema.get("properties")))
        .and_then(JsonValue::as_object)
        .ok_or_else(|| anyhow!("JSON schema for '{}' lacks properties", message.rust_type))?;
    let required_root = schema
        .get("required")
        .or_else(|| schema.get("schema").and_then(|schema| schema.get("required")));
    let required = required_root
        .and_then(JsonValue::as_array)
        .cloned()
        .unwrap_or_default()
        .into_iter()
        .filter_map(|value| value.as_str().map(str::to_string))
        .collect::<BTreeSet<_>>();
    for field in &message.fields {
        if !properties.contains_key(&field.field_name) {
            return Err(anyhow!(
                "protobuf field '{}' is not present in JSON schema for '{}'",
                field.field_name,
                message.rust_type
            ));
        }
        let schema_required = required.contains(&field.field_name);
        if field.required_in_json_schema != schema_required {
            return Err(anyhow!(
                "protobuf field '{}' required flag does not match JSON schema for '{}'",
                field.field_name,
                message.rust_type
            ));
        }
        let property = properties
            .get(&field.field_name)
            .ok_or_else(|| anyhow!("missing schema property '{}'", field.field_name))?;
        let expected_shape = projected_schema_shape(property).with_context(|| {
            format!(
                "derive JSON schema projection shape for '{}.{}'",
                message.rust_type, field.field_name
            )
        })?;
        if field.repeated != expected_shape.repeated {
            return Err(anyhow!(
                "protobuf field '{}' repeated flag does not match JSON schema for '{}'",
                field.field_name,
                message.rust_type
            ));
        }
        if field.kind != expected_shape.kind {
            return Err(anyhow!(
                "protobuf field '{}' kind does not match JSON schema for '{}'",
                field.field_name,
                message.rust_type
            ));
        }
        if field.type_ref != expected_shape.type_ref {
            return Err(anyhow!(
                "protobuf field '{}' type_ref '{}' does not match JSON schema-derived type_ref '{}'",
                field.field_name,
                field.type_ref,
                expected_shape.type_ref
            ));
        }
    }
    for required_field in required {
        if message
            .fields
            .iter()
            .all(|field| field.field_name != required_field)
        {
            return Err(anyhow!(
                "required JSON field '{}' is missing from protobuf projection for '{}'",
                required_field,
                message.rust_type
            ));
        }
    }
    for property_name in properties.keys() {
        if message
            .fields
            .iter()
            .all(|field| field.field_name != *property_name)
        {
            return Err(anyhow!(
                "JSON field '{}' is missing from protobuf projection for '{}'",
                property_name,
                message.rust_type
            ));
        }
    }
    Ok(())
}

struct ProjectedSchemaShape {
    kind: AcipProtobufFieldKindV1,
    type_ref: String,
    repeated: bool,
}

fn projected_schema_shape(property: &JsonValue) -> Result<ProjectedSchemaShape> {
    if let Some(nullable) = property
        .get("anyOf")
        .or_else(|| property.get("oneOf"))
        .and_then(JsonValue::as_array)
    {
        let non_null = nullable
            .iter()
            .find(|candidate| {
                candidate.get("type").and_then(JsonValue::as_str) != Some("null")
            })
            .ok_or_else(|| anyhow!("nullable schema property lacks non-null branch"))?;
        return projected_schema_shape(non_null);
    }

    if let Some(all_of) = property.get("allOf").and_then(JsonValue::as_array) {
        let first = all_of
            .first()
            .ok_or_else(|| anyhow!("allOf schema property lacks entries"))?;
        return projected_schema_shape(first);
    }

    if property
        .get("type")
        .and_then(JsonValue::as_str)
        .is_some_and(|value| value == "array")
    {
        let items = property
            .get("items")
            .ok_or_else(|| anyhow!("array schema property lacks items"))?;
        let mut shape = projected_schema_shape(items)?;
        shape.repeated = true;
        return Ok(shape);
    }

    if let Some(ref_value) = property.get("$ref").and_then(JsonValue::as_str) {
        let type_ref = protobuf_type_ref_from_schema_ref(ref_value)?;
        let kind = if type_ref.starts_with("Acip")
            && matches!(
                type_ref.as_str(),
                "AcipIntent"
                    | "AcipVisibility"
                    | "AcipTraceRequirement"
                    | "AcipInvocationStatus"
                    | "AcipCallerClass"
                    | "AcipRouteClass"
                    | "AcipA2aTrustClass"
            ) {
            AcipProtobufFieldKindV1::Enum
        } else {
            AcipProtobufFieldKindV1::Message
        };
        return Ok(ProjectedSchemaShape {
            kind,
            type_ref,
            repeated: false,
        });
    }

    let Some(json_type) = schema_property_type(property) else {
        return Err(anyhow!("schema property lacks type or $ref"));
    };
    let type_ref = match json_type {
        "string" => "string".to_string(),
        "boolean" => "bool".to_string(),
        "integer" => match property.get("format").and_then(JsonValue::as_str) {
            Some("uint32") => "uint32".to_string(),
            Some("uint64") => "uint64".to_string(),
            _ => "int64".to_string(),
        },
        other => {
            return Err(anyhow!(
                "unsupported scalar JSON schema type '{}' for protobuf projection",
                other
            ))
        }
    };
    Ok(ProjectedSchemaShape {
        kind: AcipProtobufFieldKindV1::Scalar,
        type_ref,
        repeated: false,
    })
}

fn protobuf_type_ref_from_schema_ref(ref_value: &str) -> Result<String> {
    let type_name = ref_value
        .strip_prefix("#/definitions/")
        .ok_or_else(|| anyhow!("unsupported JSON schema ref '{}'", ref_value))?;
    Ok(type_name
        .strip_suffix("V1")
        .unwrap_or(type_name)
        .to_string())
}

fn schema_property_type(property: &JsonValue) -> Option<&str> {
    if let Some(json_type) = property.get("type").and_then(JsonValue::as_str) {
        return Some(json_type);
    }
    property
        .get("type")
        .and_then(JsonValue::as_array)?
        .iter()
        .filter_map(JsonValue::as_str)
        .find(|json_type| *json_type != "null")
}

fn require_projection_ref(values: &[String], needle: &str) -> Result<()> {
    if values.iter().all(|value| !value.contains(needle)) {
        return Err(anyhow!(
            "ACIP protobuf projection profile must reference '{}'",
            needle
        ));
    }
    Ok(())
}

fn require_projection_non_claim(values: &[String], needle: &str) -> Result<()> {
    if values.iter().all(|value| !value.contains(needle)) {
        return Err(anyhow!(
            "ACIP protobuf projection profile non_claims must mention '{}'",
            needle
        ));
    }
    Ok(())
}

fn require_projection_text(value: &str, needle: &str, field: &str) -> Result<()> {
    if !value.contains(needle) {
        return Err(anyhow!("{field} must mention '{needle}'"));
    }
    Ok(())
}
