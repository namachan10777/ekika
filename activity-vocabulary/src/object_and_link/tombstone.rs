use serde::{Deserialize, Serialize};

use crate::{def_subtypes, xsd, FunctionalProperty, Object, RemotableProperty};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tombstone)
///
/// uri: `https://www.w3.org/ns/activitystreams#Tombstone`
///
/// A Tombstone represents a content object that has been deleted.
/// It can be used in [crate::Collection]s to signify that there used to be an object at this position, but it has been deleted.
///
/// ```json
/// {
///   "type": "OrderedCollection",
///   "totalItems": 3,
///   "name": "Vacation photos 2016",
///   "orderedItems": [
///     {
///       "type": "Image",
///       "id": "http://image.example/1"
///     },
///     {
///       "type": "Tombstone",
///       "formerType": "Image",
///       "id": "http://image.example/2",
///       "deleted": "2016-03-17T00:00:00Z"
///     },
///     {
///       "type": "Image",
///       "id": "http://image.example/3"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Tombstone {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-formertype)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#formerType`
    ///
    /// On a [Tombstone] object, the [Tombstone::former_type] property identifies the type of the object that was deleted.
    #[serde(
        skip_serializing_if = "RemotableProperty::is_empty",
        rename = "formerType"
    )]
    pub former_type: RemotableProperty<Object>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-deleted)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#deleted`
    ///
    /// On a [Tombstone] object, the [Tombstone::deleted] property is a timestamp for when the object was deleted.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub deleted: FunctionalProperty<xsd::DateTime>,
}

def_subtypes!(Tombstone, TombstoneSubtypes, [Object], { Tombstone });
