#![cfg(feature = "character-profile")]
#![allow(clippy::unwrap_used)]

use oclive_kernel_base::{character_profile, minimal_role, CharacterProfileDefinition};

#[test]
fn canonical_and_legacy_paths_are_the_same_type_and_json_projection() {
    let old_module = minimal_role::MinimalRoleDefinition {
        persona_prompt: "  A careful guide.\n".into(),
        visual_assets: vec!["adapter:portrait".into()],
    };
    let canonical_module: character_profile::CharacterProfileDefinition = old_module;
    let old_root: oclive_kernel_base::MinimalRoleDefinition = canonical_module;
    let canonical_root: CharacterProfileDefinition = old_root;
    let old_name_in_new_module: character_profile::MinimalRoleDefinition = canonical_root;
    let new_name_in_old_module: minimal_role::CharacterProfileDefinition = old_name_in_new_module;
    let raw = serde_json::to_string(&new_name_in_old_module).unwrap();
    let new_parser = character_profile::parse_character_profile_definition(&raw).unwrap();
    let old_parser = minimal_role::parse_minimal_role_definition(&raw).unwrap();
    assert_eq!(new_parser, old_parser);
    assert_eq!(new_parser, new_name_in_old_module);
    assert_eq!(new_parser.persona_prompt, "  A careful guide.\n");
    assert_eq!(
        serde_json::to_value(new_parser).unwrap(),
        serde_json::json!({
            "persona_prompt": "  A careful guide.\n",
            "visual_assets": ["adapter:portrait"]
        })
    );
}

#[test]
fn one_asset_remains_compatible_and_seven_is_a_recommendation() {
    for assets in [
        vec!["portrait"],
        vec!["one", "two", "three", "four", "five", "six", "seven"],
        vec![],
        vec!["portrait", " \t"],
    ] {
        let profile = CharacterProfileDefinition {
            persona_prompt: "Guide".into(),
            visual_assets: assets.iter().map(|asset| (*asset).into()).collect(),
        };
        let expected_valid =
            !assets.is_empty() && assets.iter().all(|asset| !asset.trim().is_empty());
        let new_result = oclive_kernel_base::validate_character_profile_definition(&profile);
        let old_result = oclive_kernel_base::validate_minimal_role_definition(&profile);
        assert_eq!(new_result.is_ok(), expected_valid);
        assert_eq!(new_result, old_result);
    }
}

#[test]
fn parser_aliases_preserve_errors_without_echoing_supplied_content() {
    for raw in [
        "{broken",
        r#"{"persona_prompt":"private-persona","visual_assets":"private-asset"}"#,
        r#"{"persona_prompt":"private-persona","visual_assets":[]}"#,
        r#"{"persona_prompt":"private-persona","visual_assets":[" \t"]}"#,
    ] {
        let errors = oclive_kernel_base::parse_character_profile_definition(raw).unwrap_err();
        assert_eq!(
            errors,
            oclive_kernel_base::parse_minimal_role_definition(raw).unwrap_err()
        );
        assert!(errors.iter().all(|error| {
            error.starts_with("minimal role:")
                && !error.contains("private-persona")
                && !error.contains("private-asset")
        }));
    }
}
