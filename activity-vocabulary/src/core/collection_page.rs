use serde::{Deserialize, Serialize};

use crate::{
    Collection, FunctionalProperty, LinkSubtypes, Object, Or, OrderedCollectionPage, Remotable,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub _super: Collection,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none", rename = "partOf")]
    part_of: FunctionalProperty<Box<Or<Remotable<Collection>, LinkSubtypes>>>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    next: FunctionalProperty<Box<Or<Remotable<CollectionPageSubtypes>, LinkSubtypes>>>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    prev: FunctionalProperty<Box<Or<Remotable<CollectionPageSubtypes>, LinkSubtypes>>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum CollectionPageSubtypes {
    CollectionPage(CollectionPage),
    OrderedCollectionPage(OrderedCollectionPage),
}

impl From<CollectionPage> for Collection {
    fn from(value: CollectionPage) -> Self {
        value._super
    }
}

impl From<CollectionPage> for Object {
    fn from(value: CollectionPage) -> Self {
        value._super.into()
    }
}

impl From<CollectionPageSubtypes> for CollectionPage {
    fn from(value: CollectionPageSubtypes) -> Self {
        match value {
            CollectionPageSubtypes::CollectionPage(x) => x.into(),
            CollectionPageSubtypes::OrderedCollectionPage(x) => x.into(),
        }
    }
}

impl From<CollectionPageSubtypes> for Collection {
    fn from(value: CollectionPageSubtypes) -> Self {
        Into::<CollectionPage>::into(value).into()
    }
}

impl From<CollectionPageSubtypes> for Object {
    fn from(value: CollectionPageSubtypes) -> Self {
        Into::<CollectionPage>::into(value).into()
    }
}
