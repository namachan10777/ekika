use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, FunctionalProperty, Object,
};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
enum Unit {
    Cm,
    // "cm" | " feet" | " inches" | " km" | " m" | " miles" | xsd:anyURI
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
