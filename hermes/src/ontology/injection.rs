use super::{HermesMessage, RequestId};
use chrono::prelude::*;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;

type Value = String;
type Entity = String;
type Prononciation = String;

#[derive(Debug, Clone, PartialEq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum InjectionKind {
    /// Add to current assistant
    Add,
    /// Add from the values downloaded
    AddFromVanilla,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectionRequestMessage {
    /// List of operations to execute in the order of the list on a model
    pub operations: Vec<(InjectionKind, HashMap<Entity, Vec<EntityValue>>)>,
    /// List of pre-computed prononciations to add in a model
    #[serde(default)]
    pub lexicon: HashMap<Value, Vec<Prononciation>>,
    /// Language for cross-language G2P
    pub cross_language: Option<String>,
    /// The id of the `InjectionRequest` that was processed
    pub id: Option<RequestId>,
}

impl<'de> HermesMessage<'de> for InjectionRequestMessage {}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InjectionStatusMessage {
    /// Date of the latest injection
    pub last_injection_date: Option<DateTime<Utc>>,
}

impl<'de> HermesMessage<'de> for InjectionStatusMessage {}

#[cfg(test)]
mod test {
    use super::*;
    use serde_json;

    #[test]
    fn custom_deserialization_entityvalue_works() {
        let json = r#" "a" "#;
        let entity_value: EntityValue = serde_json::from_str(&json).unwrap();
        assert_eq!(
            entity_value,
            EntityValue {
                value: "a".to_string(),
                weight: 1
            }
        );

        let json = r#"["a", 42]"#;
        let entity_value: EntityValue = serde_json::from_str(&json).unwrap();
        assert_eq!(
            entity_value,
            EntityValue {
                value: "a".to_string(),
                weight: 42
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
                weight: 1
            }
        );
        assert_eq!(
            values_per_entity["e_0"][1],
            EntityValue {
                value: "b".to_string(),
                weight: 42
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
                weight: 22
            }
        );
        assert_eq!(
            values_per_entity["e_0"][1],
            EntityValue {
                value: "b".to_string(),
                weight: 31
            }
        );
    }
}
