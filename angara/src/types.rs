use std::str::FromStr;

use serde::Deserialize;
use valuable::Valuable;

#[derive(Valuable)]
pub struct WebfingerId {}

impl FromStr for WebfingerId {
    type Err = anyhow::Error;

    fn from_str(_: &str) -> Result<Self, Self::Err> {
        unimplemented!()
    }
}

impl<'de> Deserialize<'de> for WebfingerId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str as Deserialize>::deserialize(deserializer)?;
        WebfingerId::from_str(s).map_err(|e| serde::de::Error::custom(e.to_string()))
    }
}
