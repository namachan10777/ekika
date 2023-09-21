use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{def_subtypes, FunctionalProperty, Object};

/// Units for [Place]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Unit {
    /// represents `cm`
    Cm,
    /// represents `ft`
    Feet,
    /// represents `in`
    Inches,
    /// represents `km`
    Km,
    /// represents `m`
    M,
    /// represents `mi`
    Miles,
    /// any units type identified by URL
    Uri(url::Url),
}

impl Default for Unit {
    fn default() -> Self {
        Self::M
    }
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

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-place)
///
/// uri: `https://www.w3.org/ns/activitystreams#Place`
///
/// Represents a logical or physical location. See [5.3 Representing Places](https://www.w3.org/TR/activitystreams-vocabulary/#places) for additional information.
///
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Place",
///   "name": "Fresno Area",
///   "latitude": 36.75,
///   "longitude": 119.7667,
///   "radius": 15,
///   "units": "miles"
/// }
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Place {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accuracy)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#accuracy`
    ///
    /// Indicates the accuracy of position coordinates on a [Place] objects.
    /// Expressed in properties of percentage. e.g. "94.0" means "94.0% accurate".
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub accuracy: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-altitude)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#altitude`
    ///
    /// Indicates the altitude of a place.
    /// The measurement [Place::units] is indicated using the units property.
    ///
    /// See also [Unit].
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub altitude: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-latitude)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#latitude`
    ///
    /// The latitude of a place
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub latitude: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-longitude)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#longitude`
    ///
    /// The longitude of a place
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub longitude: FunctionalProperty<f64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-radius)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#radius`
    ///
    /// The radius from the given latitude and longitude for a [Place].
    /// The units is expressed by the [Place::units] property.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub radius: FunctionalProperty<f64>,
    /// [W3C recommendation]()
    ///
    /// uri: `https://www.w3.org/TR/activitystreams-vocabulary/#dfn-units`
    ///
    /// Specifies the measurement units for the [Place::radius] and [Place::altitude] properties on a [Place] object.
    /// If not specified, the default is assumed to be [Unit::M] for "meters".
    #[serde(default = "Unit::default")]
    pub units: Unit,
}

def_subtypes!(Place, PlaceSubtypes, [Object], { Place });
