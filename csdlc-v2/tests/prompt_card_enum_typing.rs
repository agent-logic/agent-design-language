use std::fmt::{Debug, Display};
use std::str::FromStr;

use csdlc_v2::cards::{
    apply, render, CardKind, CardStatus, CardValues, CloseoutState, EvidenceOutcome,
    FindingDisposition, FindingSeverity, IntegrationState, MergeState, PlanningCollectionField,
    PlanningProfile, PublicationState, ResourceProfile, ReviewResult, SemanticOperation,
    StepStatus, TextField,
};
use csdlc_v2::{public_schema_bundle, DesignReview, LifecyclePhase};
use schemars::JsonSchema;
use serde::de::DeserializeOwned;
use serde::Serialize;
use strum::IntoEnumIterator;

fn assert_finite_enum_contract<T>()
where
    T: IntoEnumIterator
        + Serialize
        + DeserializeOwned
        + JsonSchema
        + Display
        + FromStr
        + Debug
        + PartialEq,
    <T as FromStr>::Err: Debug,
{
    let schema = serde_json::to_string(&schemars::schema_for!(T)).expect("schema serializes");
    let mut count = 0;

    for value in T::iter() {
        count += 1;
        let json = serde_json::to_string(&value).expect("enum serializes");
        let decoded: T = serde_json::from_str(&json).expect("canonical value deserializes");
        assert_eq!(decoded, value);

        let token = json.trim_matches('"');
        assert_eq!(token.parse::<T>().expect("canonical token parses"), value);
        assert_eq!(value.to_string(), token);
        assert!(
            schema.contains(token),
            "schema omits canonical value {token}"
        );
    }

    assert!(count > 0, "finite enum must expose at least one value");
    assert!(
        serde_json::from_str::<T>(r#""__unsupported_value__""#).is_err(),
        "unknown finite value must fail closed"
    );
    assert!(
        "__unsupported_value__".parse::<T>().is_err(),
        "unknown finite token must fail closed"
    );
}

#[test]
fn finite_prompt_card_enums_share_serde_parse_display_and_schema_authority() {
    assert_finite_enum_contract::<CardKind>();
    assert_finite_enum_contract::<CardStatus>();
    assert_finite_enum_contract::<StepStatus>();
    assert_finite_enum_contract::<PlanningProfile>();
    assert_finite_enum_contract::<FindingDisposition>();
    assert_finite_enum_contract::<ResourceProfile>();
    assert_finite_enum_contract::<FindingSeverity>();
    assert_finite_enum_contract::<ReviewResult>();
    assert_finite_enum_contract::<EvidenceOutcome>();
    assert_finite_enum_contract::<IntegrationState>();
    assert_finite_enum_contract::<PublicationState>();
    assert_finite_enum_contract::<MergeState>();
    assert_finite_enum_contract::<CloseoutState>();
    assert_finite_enum_contract::<LifecyclePhase>();
    assert_finite_enum_contract::<TextField>();
    assert_finite_enum_contract::<PlanningCollectionField>();
}

#[test]
fn unsupported_legacy_spellings_do_not_rewrite_durable_values() {
    assert_eq!(
        serde_json::from_str::<CardKind>(r#""sip""#).unwrap(),
        CardKind::Sip
    );
    for unsupported in [r#""SIP""#, r#""Sip""#, r#""structured_intent_prompt""#] {
        assert!(serde_json::from_str::<CardKind>(unsupported).is_err());
    }

    assert_eq!(
        serde_json::from_str::<StepStatus>(r#""in_progress""#).unwrap(),
        StepStatus::InProgress
    );
    for unsupported in [r#""in-progress""#, r#""in progress""#, r#""InProgress""#] {
        assert!(serde_json::from_str::<StepStatus>(unsupported).is_err());
    }
}

#[test]
fn tagged_enum_contracts_round_trip_and_reject_unknown_discriminants() {
    let design = DesignReview::Approved {
        reviewer: "reviewer".into(),
        revision: "revision".into(),
    };
    let design_json = serde_json::to_string(&design).unwrap();
    assert_eq!(
        serde_json::from_str::<DesignReview>(&design_json).unwrap(),
        design
    );
    assert!(serde_json::from_str::<DesignReview>(r#""approved""#).is_err());

    let operation = SemanticOperation::Replan {
        field: TextField::PlanSummary,
        value: "bounded plan".into(),
    };
    let operation_json = serde_json::to_string(&operation).unwrap();
    assert_eq!(
        serde_json::from_str::<SemanticOperation>(&operation_json).unwrap(),
        operation
    );
    assert!(serde_json::from_str::<SemanticOperation>(r#"{"operation":"unknown"}"#).is_err());
}

#[test]
fn all_six_values_cards_and_rendered_markdown_remain_stable() {
    let fixtures = [
        (
            include_str!("../../.csdlc/issues/5824/cards/sip.values.json"),
            include_str!("../../.csdlc/issues/5824/cards/sip.md"),
        ),
        (
            include_str!("../../.csdlc/issues/5824/cards/stp.values.json"),
            include_str!("../../.csdlc/issues/5824/cards/stp.md"),
        ),
        (
            include_str!("../../.csdlc/issues/5824/cards/spp.values.json"),
            include_str!("../../.csdlc/issues/5824/cards/spp.md"),
        ),
        (
            include_str!("../../.csdlc/issues/5824/cards/vpp.values.json"),
            include_str!("../../.csdlc/issues/5824/cards/vpp.md"),
        ),
        (
            include_str!("../../.csdlc/issues/5824/cards/srp.values.json"),
            include_str!("../../.csdlc/issues/5824/cards/srp.md"),
        ),
        (
            include_str!("../../.csdlc/issues/5824/cards/sor.values.json"),
            include_str!("../../.csdlc/issues/5824/cards/sor.md"),
        ),
    ];

    for (fixture, tracked_markdown) in fixtures {
        let values: CardValues = serde_json::from_str(fixture).expect("tracked values card");
        let json = serde_json::to_string(&values).unwrap();
        assert_eq!(serde_json::from_str::<CardValues>(&json).unwrap(), values);

        let first = render(&values).expect("render and Markdown AST parse");
        let second = render(&values).expect("stable second render");
        assert_eq!(first.markdown, tracked_markdown);
        assert_eq!(first.markdown, second.markdown);
        assert_eq!(first.values_digest, second.values_digest);
        assert_eq!(first.rendered_digest, second.rendered_digest);
        assert_eq!(first.ast_digest, second.ast_digest);
    }
}

#[test]
fn active_registry_and_editor_schema_expose_the_canonical_values() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repository root");
    let registry: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(root.join("docs/templates/prompts/current.json")).unwrap(),
    )
    .unwrap();
    let native = &registry["generations"]["csdlc_v2_native"];
    assert_eq!(native["template_set"], "1.0.0");
    let shape_path = native["shape_manifest_path"].as_str().unwrap();
    let shape: serde_json::Value =
        serde_json::from_str(&std::fs::read_to_string(root.join(shape_path)).unwrap()).unwrap();
    assert_eq!(shape["template_set"], native["template_set"]);

    let fixtures = [
        include_str!("../../.csdlc/issues/5824/cards/sip.values.json"),
        include_str!("../../.csdlc/issues/5824/cards/stp.values.json"),
        include_str!("../../.csdlc/issues/5824/cards/spp.values.json"),
        include_str!("../../.csdlc/issues/5824/cards/vpp.values.json"),
        include_str!("../../.csdlc/issues/5824/cards/srp.values.json"),
        include_str!("../../.csdlc/issues/5824/cards/sor.values.json"),
    ];
    for fixture in fixtures {
        let values: CardValues = serde_json::from_str(fixture).unwrap();
        assert_eq!(values.identity.template_version, native["template_set"]);
        let markdown = render(&values).unwrap().markdown;
        let headings: Vec<_> = markdown
            .lines()
            .filter_map(|line| line.strip_prefix("## "))
            .collect();
        let expected: Vec<_> = shape["cards"][values.kind().to_string()]
            .as_array()
            .unwrap()
            .iter()
            .map(|value| value.as_str().unwrap())
            .collect();
        assert_eq!(headings, expected);
    }

    for template in registry["templates"].as_object().unwrap().values() {
        let schema_path = template["structure_schema_path"].as_str().unwrap();
        let schema: serde_json::Value =
            serde_json::from_str(&std::fs::read_to_string(root.join(schema_path)).unwrap())
                .unwrap();
        assert!(
            schema.is_object(),
            "{schema_path} must contain a JSON schema object"
        );
    }

    let edit_schema = public_schema_bundle()["edit_request"].to_string();
    for field in TextField::iter() {
        assert!(edit_schema.contains(&field.to_string()));
    }
    for field in PlanningCollectionField::iter() {
        assert!(edit_schema.contains(&field.to_string()));
    }
}

#[test]
fn invalid_finite_edit_reports_a_typed_diagnostic_without_mutation() {
    let mut values: CardValues = serde_json::from_str(include_str!(
        "../../.csdlc/issues/5824/cards/sip.values.json"
    ))
    .unwrap();
    let before = values.clone();
    let error = apply(
        &mut values,
        &SemanticOperation::UpdateIdentityVersion {
            version: "not-a-version".into(),
        },
    )
    .expect_err("invalid version must fail closed");
    assert_eq!(error.code.to_string(), "invalid_input");
    assert!(error.message.contains("version"));
    assert_eq!(values, before);
}
