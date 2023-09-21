use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Collection, Object, OrderedCollectionPage};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub _super: Collection,
}

def_subtypes!(
    OrderedCollection,
    OrderedCollectionSubtypes,
    [Collection, Object],
    { OrderedCollection, OrderedCollectionPage }
);
