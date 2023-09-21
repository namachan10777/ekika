use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object, RemotableObjectOrLinkProp, RemotableProperty};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(flatten)]
    pub _super: Object,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub subject: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub object: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableProperty::is_empty")]
    pub relationship: RemotableProperty<Object>,
}

def_subtypes!(Relationship, RelationShipSubtypes, [Object], {
    Relationship
});
