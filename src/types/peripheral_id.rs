use std::{fmt::Display, str::FromStr};

use serde::de::Error as SerdeError;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct PeripheralId {
    uuid: String,
    index: String,
}

impl PeripheralId {
    pub fn new(uuid: String, index: String) -> PeripheralId {
        PeripheralId { uuid, index }
    }
    pub fn uuid(&self) -> &str {
        &self.uuid
    }
    pub fn index(&self) -> &str {
        &self.index
    }
}

impl<'de> serde::Deserialize<'de> for PeripheralId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let str = <&str>::deserialize(deserializer)?;
        Self::from_str(str).map_err(SerdeError::custom)
    }
}

impl FromStr for PeripheralId {
    type Err = String;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let split = str.split_once('-');
        match split {
            Some((uuid, index)) if !uuid.is_empty() && !index.is_empty() => Ok(Self {
                uuid: uuid.to_string(),
                index: index.to_string(),
            }),
            _ => Err(format!(
                "PeripheralId is expected to be of form `<uuid>-<index>`, but was `{}`",
                str
            )),
        }
    }
}

impl serde::Serialize for PeripheralId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.collect_str(self)
    }
}

impl Display for PeripheralId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}-{}", self.uuid, self.index))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::PeripheralId;

    #[test]
    fn is_not_peripheral_id() {
        let inputs = ["asd", "1_23", "-hej", "-", "asd-"];
        for i in inputs {
            assert!(PeripheralId::from_str(i).is_err());
        }
    }

    #[test]
    fn is_peripheral_id() {
        let tester = |input, exp_uuid, exp_index| {
            // Check FromStr impl
            let ref p @ PeripheralId {
                ref index,
                ref uuid,
            } = PeripheralId::from_str(input).unwrap();
            assert_eq!(exp_uuid, uuid);
            assert_eq!(exp_index, index);

            // Check serialization roundtrip
            let json = serde_json::to_string(&p).unwrap();
            let back: PeripheralId = serde_json::from_str(&json).unwrap();
            assert_eq!(p, &back);

            // Check serialization is to a string
            let json_value = serde_json::Value::from_str(&json).unwrap();
            assert!(json_value.is_string());
        };

        tester("1-2", "1", "2");
        tester("1-2 3", "1", "2 3");
        tester("1 2-3 4", "1 2", "3 4");
        // We see the uuid as being the up to the first '-'
        tester("1 2-3-4", "1 2", "3-4");
    }
}
