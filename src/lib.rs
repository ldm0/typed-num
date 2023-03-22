use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
pub struct Num<const N: i64>;

impl<const N: i64> Serialize for Num<N> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_i64(N)
    }
}

impl<'de, const N: i64> Deserialize<'de> for Num<N> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_u64(NumVisitor::<N>)
    }
}

struct NumVisitor<const N: i64>;

impl<'de, const N: i64> Visitor<'de> for NumVisitor<N> {
    type Value = Num<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("{}", N))
    }

    fn visit_i8<E>(self, value: i8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == i64::from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_i16<E>(self, value: i16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == i64::from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_i32<E>(self, value: i32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == i64::from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == value {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_u8<E>(self, value: u8) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == i64::from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_u16<E>(self, value: u16) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == i64::from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_u32<E>(self, value: u32) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == i64::from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if Ok(N) == i64::try_from(value) {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {}", N)))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_derive::{Deserialize, Serialize};

    #[test]
    fn test_typed_num() {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct Config {
            version: Num<3>,
            hash: String,
        }

        const OLD_CONFIG: &str = r#"
version = 2
hash = "9g8OSL4Ty5nQ62yYBOOmujNAfFrkVZVUMS1iQJ85QlU="
"#;

        const NEW_CONFIG: &str = r#"
version = 3
hash = "OoXQqX+ZRNE7VLmkbhGlj2R1B3n3gAJAaGh9kS0mAv8="
"#;

        let new_config = Config {
            version: Num,
            hash: "OoXQqX+ZRNE7VLmkbhGlj2R1B3n3gAJAaGh9kS0mAv8=".to_string(),
        };

        toml::from_str::<Config>(OLD_CONFIG).unwrap_err();
        assert_eq!(toml::from_str::<Config>(NEW_CONFIG).unwrap(), new_config);
        assert_eq!(
            toml::to_string_pretty(&new_config).unwrap().trim(),
            NEW_CONFIG.trim()
        );
    }
}
