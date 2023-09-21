use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Collection, Object, OrderedCollectionPage};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollection)
///
/// uri: `https://www.w3.org/ns/activitystreams#OrderedCollection`
///
/// A subtype of [Collection] in which members of the logical collection are assumed to always be strictly ordered.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally's notes",
///   "type": "OrderedCollection",
///   "totalItems": 2,
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
