use crate::Num;
use serde::{
    de::{self, Deserializer, Visitor},
    ser::Serializer,
    Deserialize, Serialize,
};
use std::fmt;

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
        deserializer.deserialize_i64(NumVisitor::<N>)
    }
}

struct NumVisitor<const N: i64>;

impl<'de, const N: i64> Visitor<'de> for NumVisitor<N> {
    type Value = Num<N>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(&format!("{N}"))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        if N == value {
            Ok(Default::default())
        } else {
            Err(E::custom(format!("not {N}")))
        }
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match i64::try_from(value) {
            Ok(v) if v == N => Ok(Default::default()),
            _ => Err(E::custom(format!("not {N}"))),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Num;
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

        assert!(toml::from_str::<Config>(OLD_CONFIG).is_err());
        assert_eq!(toml::from_str::<Config>(NEW_CONFIG).unwrap(), new_config);
        assert_eq!(
            toml::to_string_pretty(&new_config).unwrap().trim(),
            NEW_CONFIG.trim()
        );
    }

    #[test]
    fn test_typed_num_negative() {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct ConfigNegative {
            version: Num<-1>,
            description: String,
        }

        const CONFIG_WITH_NEGATIVE_VAL: &str = r#"
version = -1
description = "negative one"
"#;
        const CONFIG_WITH_WRONG_NEGATIVE_VAL: &str = r#"
version = -2
description = "wrong negative one"
"#;

        let config_negative = ConfigNegative {
            version: Num,
            description: "negative one".to_string(),
        };

        assert!(toml::from_str::<ConfigNegative>(CONFIG_WITH_WRONG_NEGATIVE_VAL).is_err());
        assert_eq!(
            toml::from_str::<ConfigNegative>(CONFIG_WITH_NEGATIVE_VAL).unwrap(),
            config_negative
        );
        assert_eq!(
            toml::to_string_pretty(&config_negative).unwrap().trim(),
            CONFIG_WITH_NEGATIVE_VAL.trim()
        );
    }

    #[test]
    fn test_serde_typed_num_i64_min() {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct SerdeConfigMin {
            version: Num<{ i64::MIN }>,
            data: String,
        }

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct SerdeConfigMinMinusOne {
            version: Num<{ i64::MIN + 1 }>,
            data: String,
        }

        let config_min = SerdeConfigMin {
            version: Num,
            data: "test_data_min".to_string(),
        };

        let config_min_toml = toml::to_string_pretty(&config_min).unwrap();
        let decoded_min: SerdeConfigMin = toml::from_str(&config_min_toml).unwrap();
        assert_eq!(decoded_min, config_min);

        let config_min_minus_one = SerdeConfigMinMinusOne {
            version: Num,
            data: "test_data_min_minus_one".to_string(),
        };

        let config_min_minus_one_toml = toml::to_string_pretty(&config_min_minus_one).unwrap();
        let decode_result_min_minus_one: Result<SerdeConfigMin, _> =
            toml::from_str(&config_min_minus_one_toml);

        assert!(decode_result_min_minus_one.is_err());
    }

    #[test]
    fn test_serde_typed_num_i64_max() {
        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct SerdeConfigMax {
            version: Num<{ i64::MAX }>,
            data: String,
        }

        #[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
        struct SerdeConfigMaxMinusOne {
            version: Num<{ i64::MAX - 1 }>,
            data: String,
        }

        let config_max = SerdeConfigMax {
            version: Num,
            data: "test_data_max".to_string(),
        };

        let config_max_toml = toml::to_string_pretty(&config_max).unwrap();
        let decoded_max: SerdeConfigMax = toml::from_str(&config_max_toml).unwrap();
        assert_eq!(decoded_max, config_max);

        let config_max_minus_one = SerdeConfigMaxMinusOne {
            version: Num,
            data: "test_data_max_minus_one".to_string(),
        };

        let config_max_minus_one_toml = toml::to_string_pretty(&config_max_minus_one).unwrap();
        let decode_result_max_minus_one: Result<SerdeConfigMax, _> =
            toml::from_str(&config_max_minus_one_toml);

        assert!(decode_result_max_minus_one.is_err());
    }
}
