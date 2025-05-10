# typed-num

Typed number in Rust with `serde` and `bincode` support.

```rust
use typed_num::Num;
use serde_derive::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Config {
    version: Num<3>,
    hash: String,
}

const OLD_CONFIG: &str = r#"
version = 2
hash = "OoXQqX+ZRNE7VLmkbhGlj2R1B3n3gAJAaGh9kS0mAv8="
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
assert_eq!(toml::to_string_pretty(&new_config).unwrap().trim(), NEW_CONFIG.trim());
```
