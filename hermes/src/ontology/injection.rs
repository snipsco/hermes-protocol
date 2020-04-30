use std::collections::HashMap;

use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use hermes_utils::Example;

use super::HermesMessage;

type Value = String;
type Entity = String;
type Pronunciation = String;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InjectionKind {
    /// Add to current assistant
    Add,
    /// Add from the values downloaded
    AddFromVanilla,
}

impl Example for InjectionKind {
    fn example(_: hermes_utils::ExampleConfig) -> Self {
        Self::Add
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Example)]
pub struct EntityValue {
    pub value: String,
    pub weight: u32,
}

impl<'de> Deserialize<'de> for EntityValue {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum DeEntityValue {
            Value(String),
            WeightedValue((String, u32)),
        }

        let (value, weight) = match DeEntityValue::deserialize(deserializer)? {
            DeEntityValue::Value(value) => (value, 1),
            DeEntityValue::WeightedValue(weighted_value) => weighted_value,
        };

        Ok(Self { value, weight })
    }
}

impl Serialize for EntityValue {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        #[serde(untagged)]
        enum SerEntityValue<'a> {
            WeightedValue((&'a str, u32)),
        }

        SerEntityValue::WeightedValue((&*self.value, self.weight)).serialize(serializer)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionRequestMessage {
    /// List of operations to execute in the order of the list on a model
    pub operations: Vec<(InjectionKind, HashMap<Entity, Vec<EntityValue>>)>,
    /// List of pre-computed prononciations to add in a model
    #[serde(default)]
    pub lexicon: HashMap<Value, Vec<Pronunciation>>,
    /// Language for cross-language G2P
    pub cross_language: Option<String>,
    /// The id of the `InjectionRequest` that was processed
    pub id: Option<String>,
}

impl<'de> HermesMessage<'de> for InjectionRequestMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionStatusMessage {
    /// Date of the latest injection
    #[example_value(DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(1_545_696_000, 0), Utc))]
    pub last_injection_date: Option<DateTime<Utc>>,
}

impl<'de> HermesMessage<'de> for InjectionStatusMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionCompleteMessage {
    /// The id of the `InjectionRequestMessage`
    pub request_id: Option<String>,
}

impl<'de> HermesMessage<'de> for InjectionCompleteMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionFailedMessage {
    /// The id of the `InjectionFailedMessage`
    pub request_id: Option<String>,
    /// The context of the failure
    pub context: String,
}

impl<'de> HermesMessage<'de> for InjectionFailedMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionResetRequestMessage {
    /// The id of the `InjectionResetRequestMessage`
    pub request_id: Option<String>,
}

impl<'de> HermesMessage<'de> for InjectionResetRequestMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionResetCompleteMessage {
    /// The id of the `InjectionResetCompleteMessage`
    pub request_id: Option<String>,
}

impl<'de> HermesMessage<'de> for InjectionResetCompleteMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Example)]
#[serde(rename_all = "camelCase")]
pub struct InjectionResetFailedMessage {
    /// The id of the `InjectionResetFailedMessage`
    pub request_id: Option<String>,
    /// The context of the failure
    pub context: String,
}

impl<'de> HermesMessage<'de> for InjectionResetFailedMessage {}

#[cfg(test)]
mod test {
    use serde_json;

    use super::*;

    #[test]
    fn custom_deserialization_entityvalue_works() {
        let json = r#" "a" "#;
        let entity_value: EntityValue = serde_json::from_str(&json).unwrap();
        assert_eq!(
            entity_value,
            EntityValue {
                value: "a".to_string(),
                weight: 1,
            }
        );

        let json = r#"["a", 42]"#;
        let entity_value: EntityValue = serde_json::from_str(&json).unwrap();
        assert_eq!(
            entity_value,
            EntityValue {
                value: "a".to_string(),
                weight: 42,
            }
        );
    }

    #[test]
    fn custom_serialization_entityvalue_works() {
        let entity_value = EntityValue {
            value: "hello".to_string(),
            weight: 42,
        };
        let string = serde_json::to_string(&entity_value).unwrap();
        assert_eq!(string, r#"["hello",42]"#);
    }

    #[test]
    fn without_weights_works() {
        let json = r#"{
            "operations": [["add", {"e_0": ["a", ["b", 42]]}]]
        }"#;

        let my_struct: InjectionRequestMessage = serde_json::from_str(&json).unwrap();
        let (operation, values_per_entity) = &my_struct.operations[0];

        assert_eq!(operation, &InjectionKind::Add);
        assert_eq!(
            values_per_entity["e_0"][0],
            EntityValue {
                value: "a".to_string(),
                weight: 1,
            }
        );
        assert_eq!(
            values_per_entity["e_0"][1],
            EntityValue {
                value: "b".to_string(),
                weight: 42,
            }
        );
    }

    #[test]
    fn with_weights_works() {
        let json = r#"{
            "operations": [["add", {"e_0": [["a", 22], ["b", 31]]}]]
        }"#;

        let my_struct: InjectionRequestMessage = serde_json::from_str(&json).unwrap();
        let (operation, values_per_entity) = &my_struct.operations[0];

        assert_eq!(operation, &InjectionKind::Add);
        assert_eq!(
            values_per_entity["e_0"][0],
            EntityValue {
                value: "a".to_string(),
                weight: 22,
            }
        );
        assert_eq!(
            values_per_entity["e_0"][1],
            EntityValue {
                value: "b".to_string(),
                weight: 31,
            }
        );
    }
}
