use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{def_subtypes, FunctionalProperty, Object};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Unit {
    Cm,
    Feet,
    Inches,
    Km,
    M,
    Miles,
    Uri(url::Url),
}

impl FromStr for Unit {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "cm" => Self::Cm,
            "feet" => Self::Feet,
            "inches" => Self::Inches,
            "km" => Self::Km,
            "m" => Self::M,
            "miles" => Self::Miles,
            other => other.parse().map(Self::Uri)?,
        })
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cm => f.write_str("cm"),
            Self::Feet => f.write_str("feet"),
            Self::Inches => f.write_str("inches"),
            Self::Km => f.write_str("km"),
            Self::M => f.write_str("m"),
            Self::Miles => f.write_str("miles"),
            Self::Uri(uri) => uri.fmt(f),
        }
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = <&str as Deserialize>::deserialize(deserializer)?;
        s.parse().map_err(serde::de::Error::custom)
    }
}

impl Serialize for Unit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Place {
    #[serde(flatten)]
    _super: Object,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    accuracy: FunctionalProperty<f64>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    altitude: FunctionalProperty<f64>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    latitude: FunctionalProperty<f64>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    longitude: FunctionalProperty<f64>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    radius: FunctionalProperty<f64>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    units: FunctionalProperty<Unit>,
}

def_subtypes!(Place, PlaceSubtypes, [Object], { Place });
