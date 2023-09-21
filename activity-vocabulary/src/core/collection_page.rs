use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, Collection, FunctionalProperty, LinkSubtypes, Object, Or, OrderedCollectionPage,
    Remotable,
};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collectionpage)
///
/// uri: `https://www.w3.org/ns/activitystreams#CollectionPage`
///
/// Used to represent distinct subsets of items from a [Collection].
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-collectionpage) for a complete description of the [CollectionPage] object.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub _super: Collection,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-partof)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#partOf`
    ///
    /// Identifies the [Collection] to which a [CollectionPage] objects items belong.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none", rename = "partOf")]
    part_of: FunctionalProperty<Box<Or<Remotable<Collection>, LinkSubtypes>>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-next)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#next`
    ///
    /// In a paged [Collection], indicates the next page of items.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    next: FunctionalProperty<Box<Or<Remotable<CollectionPageSubtypes>, LinkSubtypes>>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-prev)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#prev`
    ///
    /// In a paged [Collection], indicates the prev page of items.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    prev: FunctionalProperty<Box<Or<Remotable<CollectionPageSubtypes>, LinkSubtypes>>>,
}

def_subtypes!(
    CollectionPage,
    CollectionPageSubtypes,
    [Collection, Object],
    {
        CollectionPage,
        OrderedCollectionPage
    }
);
