use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object, RemotableObjectOrLinkProp, RemotableProperty};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(flatten)]
    _super: Object,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    subject: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    object: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableProperty::is_empty")]
    relationship: RemotableProperty<Object>,
}

def_subtypes!(Relationship, RelationShipSubtypes, [Object], {
    Relationship
});
