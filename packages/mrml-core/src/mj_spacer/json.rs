use super::{MJSpacer, NAME};
use crate::json_attrs_serializer;
use serde::de::{Error, MapAccess, Visitor};
use serde::ser::SerializeMap;
use serde::{Deserialize, Deserializer};
use std::fmt;

const FIELDS: [&str; 2] = ["type", "attributes"];

json_attrs_serializer!(MJSpacer, NAME);

#[derive(Default)]
struct MJSpacerVisitor;

impl<'de> Visitor<'de> for MJSpacerVisitor {
    type Value = MJSpacer;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an map with properties type and attributes")
    }

    fn visit_map<M>(self, mut access: M) -> Result<Self::Value, M::Error>
    where
        M: MapAccess<'de>,
    {
        let mut result = MJSpacer::default();
        while let Some(key) = access.next_key::<String>()? {
            if key == "type" {
                if access.next_value::<String>()? != NAME {
                    return Err(M::Error::custom(format!("expected type to equal {}", NAME)));
                }
            } else if key == "attributes" {
                result.attributes = access.next_value()?;
            } else {
                return Err(M::Error::unknown_field(&key, &FIELDS));
            }
        }
        Ok(result)
    }
}

impl<'de> Deserialize<'de> for MJSpacer {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_map(MJSpacerVisitor::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::mj_spacer::MJSpacer;

    #[test]
    fn serialize() {
        let mut elt = MJSpacer::default();
        elt.attributes.insert("margin-bottom".into(), "20px".into());
        assert_eq!(
            serde_json::to_string(&elt).unwrap(),
            r#"{"type":"mj-spacer","attributes":{"margin-bottom":"20px"}}"#
        );
    }

    #[test]
    fn deserialize() {
        let elt = MJSpacer::default();
        let json = serde_json::to_string(&elt).unwrap();
        let _res: MJSpacer = serde_json::from_str(&json).unwrap();
    }
}
