use serde::{Deserialize, Serialize};

use crate::{Collection, CollectionPage, FunctionalProperty, Object};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub _super: CollectionPage,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none", rename = "partOf")]
    start_index: FunctionalProperty<Box<OrderedCollectionPage>>,
}

// Cannot impl `From<OrdererdCollectionPage> for OrderedCollection` because OrderedCollection is an alias of Collection.

impl From<OrderedCollectionPage> for CollectionPage {
    fn from(value: OrderedCollectionPage) -> Self {
        value._super
    }
}

impl From<OrderedCollectionPage> for Collection {
    fn from(value: OrderedCollectionPage) -> Self {
        value._super.into()
    }
}

impl From<OrderedCollectionPage> for Object {
    fn from(value: OrderedCollectionPage) -> Self {
        value._super.into()
    }
}
