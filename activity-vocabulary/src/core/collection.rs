use serde::{Deserialize, Serialize};

use crate::{
    CollectionPage, FunctionalProperty, LinkSubtypes, Object, ObjectSubtypes, Or,
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

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum CollectionSubtypes {
    Collection(Collection),
    CollectionPage(CollectionPage),
    OrderedCollection(OrderedCollection),
    OrderedCollectionPage(OrderedCollectionPage),
}

impl From<Collection> for Object {
    fn from(value: Collection) -> Self {
        value._super
    }
}

impl From<CollectionSubtypes> for Collection {
    fn from(value: CollectionSubtypes) -> Self {
        match value {
            CollectionSubtypes::Collection(x) => x.into(),
            CollectionSubtypes::CollectionPage(x) => x.into(),
            CollectionSubtypes::OrderedCollection(x) => x.into(),
            CollectionSubtypes::OrderedCollectionPage(x) => x.into(),
        }
    }
}

impl From<CollectionSubtypes> for Object {
    fn from(value: CollectionSubtypes) -> Self {
        Into::<Collection>::into(value).into()
    }
}
