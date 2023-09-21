use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, Collection, FunctionalProperty, LinkSubtypes, Object, Or, OrderedCollectionPage,
    Remotable,
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

def_subtypes!(
    CollectionPage,
    CollectionPageSubtypes,
    [Object, Collection],
    {
        CollectionPage,
        OrderedCollectionPage
    }
);

impl std::ops::Deref for CollectionPage {
    type Target = Collection;

    fn deref(&self) -> &Self::Target {
        &self._super
    }
}
