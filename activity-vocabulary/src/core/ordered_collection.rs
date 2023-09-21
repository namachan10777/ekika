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
    [Object, Collection],
    { OrderedCollection, OrderedCollectionPage }
);

impl std::ops::Deref for OrderedCollection {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self._super
    }
}
