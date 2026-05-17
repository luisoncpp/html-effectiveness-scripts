use serde::Deserialize;

use super::components::prompt_box::PromptBoxData;

#[derive(Debug, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum UiComponent {
    #[serde(rename = "prompt-box")]
    PromptBox(PromptBoxData),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserializes_prompt_box_from_valid_yaml() {
        let yaml = r#"
type: prompt-box
label: Test Label
content: Test Content
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_ok());
        assert_eq!(
            result.unwrap(),
            UiComponent::PromptBox(PromptBoxData {
                label: "Test Label".to_string(),
                content: "Test Content".to_string(),
            })
        );
    }

    #[test]
    fn returns_error_for_unknown_type() {
        let yaml = r#"
type: unknown-component
label: Test
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }

    #[test]
    fn returns_error_for_missing_required_fields() {
        let yaml = r#"
type: prompt-box
label: Test Label
"#;
        let result: Result<UiComponent, _> = serde_yaml::from_str(yaml);
        assert!(result.is_err());
    }
}
