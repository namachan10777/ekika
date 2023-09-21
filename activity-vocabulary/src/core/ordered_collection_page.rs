use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes, Collection, CollectionPage, FunctionalProperty, Object, OrderedCollection,
};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollectionpage)
///
/// uri: `https://www.w3.org/ns/activitystreams#OrderedCollectionPage`
///
/// Used to represent ordered subsets of items from an [OrderedCollection].
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-orderedcollectionpage) for a complete description of the [OrderedCollectionPage] object.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Page 1 of Sally's notes",
///   "type": "OrderedCollectionPage",
///   "id": "http://example.org/foo?page=1",
///   "partOf": "http://example.org/foo",
///   "orderedItems": [
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
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub _super: CollectionPage,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-startindex)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#startIndex`
    ///
    /// A non-negative integer value identifying the relative position within the logical view of a strictly ordered collection.
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
        OrderedCollection {
            _super: value._super.into(),
        }
    }
}
