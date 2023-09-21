use serde::{Deserialize, Serialize};

use crate::{def_subtypes, xsd, FunctionalProperty, Object, RemotableProperty};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tombstone {
    #[serde(flatten)]
    _super: Object,
    #[serde(
        skip_serializing_if = "RemotableProperty::is_empty",
        rename = "formerType"
    )]
    former_type: RemotableProperty<Object>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    deleted: FunctionalProperty<xsd::DateTime>,
}

def_subtypes!(Tombstone, TombstoneSubtypes, [Object], { Tombstone });
