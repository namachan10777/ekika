use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, Collection, CollectionPage, FunctionalProperty, Object, OrderedCollection,
};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub _super: CollectionPage,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none", rename = "partOf")]
    pub start_index: FunctionalProperty<Box<OrderedCollectionPage>>,
}

// Cannot impl `From<OrdererdCollectionPage> for OrderedCollection` because OrderedCollection is an alias of Collection.

def_subtypes!(
    OrderedCollectionPage,
    OrderedCollectionPageSubtypes,
    [CollectionPage, Collection, Object],
    { OrderedCollectionPage }
);

impl From<OrderedCollectionPage> for OrderedCollection {
    fn from(value: OrderedCollectionPage) -> Self {
        OrderedCollection { _super: value._super.into() }
    }
}
