use serde::{Deserialize, Serialize};

use crate::{Collection, Object, OrderedCollectionPage};

pub type OrderedCollection = Collection;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum OrdererdCollectionSubtypes {
    OrderedCollection(OrderedCollection),
    OrderedCollectionPage(OrderedCollectionPage),
}

impl From<OrdererdCollectionSubtypes> for OrderedCollection {
    fn from(value: OrdererdCollectionSubtypes) -> Self {
        match value {
            OrdererdCollectionSubtypes::OrderedCollection(x) => x.into(),
            OrdererdCollectionSubtypes::OrderedCollectionPage(x) => x.into(),
        }
    }
}

impl From<OrdererdCollectionSubtypes> for Object {
    fn from(value: OrdererdCollectionSubtypes) -> Self {
        Into::<OrderedCollection>::into(value).into()
    }
}
