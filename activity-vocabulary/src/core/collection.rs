use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, CollectionPage, FunctionalProperty, LinkSubtypes, Object, ObjectSubtypes, Or,
    OrderedCollection, OrderedCollectionPage, RemotableObjectOrLinkProp, RemotableOrLinkProp,
};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection)
///
/// uri: `https://www.w3.org/ns/activitystreams#Collection`
///
/// A [Collection] is a subtype of [Object] that represents ordered or unordered sets of [Object] or [crate::Link] instances.
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#collection) specification for a complete description of the [Collection] type.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally's notes",
///   "type": "Collection",
///   "totalItems": 2,
///   "items": [
///     {
///       "type": "Note",
///       "name": "A Simple Note"
///     },
///     {
///       "type": "Note",
///       "name": "Another Simple Note"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct Collection {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-totalitems)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#totalItems`
    ///
    /// A non-negative integer specifying the total number of objects contained
    /// by the logical view of the collection.
    /// This number might not reflect the actual number of items serialized within the [Collection] object instance.
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "totalItems"
    )]
    pub total_items: FunctionalProperty<usize>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-current)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#current`
    ///
    /// In a paged [Collection], indicates the page that contains the most recently updated member items.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub current: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-first)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#first`
    ///
    /// In a paged [Collection], indicates the furthest preceeding page of items in the collection.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub first: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-first)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#first`
    ///
    /// In a paged [Collection], indicates the furthest preceeding page of items in the collection.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub last: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-items)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#items`
    ///
    /// Identifies the items contained in a collection. The items might be ordered or unordered.
    #[serde(skip_serializing_if = "RemotableOrLinkProp::is_empty")]
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
