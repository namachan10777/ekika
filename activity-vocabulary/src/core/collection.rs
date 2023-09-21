use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, CollectionPage, FunctionalProperty, LinkSubtypes, Object, ObjectSubtypes, Or,
    OrderedCollection, OrderedCollectionPage, RemotableObjectOrLinkProp, RemotableOrLinkProp,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct Collection {
    #[serde(flatten)]
    pub _super: Object,
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "totalItems"
    )]
    pub total_items: FunctionalProperty<usize>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub current: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub first: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub last: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    pub items: RemotableOrLinkProp<Or<ObjectSubtypes, Vec<RemotableObjectOrLinkProp>>>,
}

def_subtypes!(
    Collection,
    CollectionSubtypes,
    [Object],
    {
        Collection,
        CollectionPage,
        OrderedCollection,
        OrderedCollectionPage
    }
);

