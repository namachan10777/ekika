use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete)
///
/// uri: `https://www.w3.org/ns/activitystreams#Delete`
///
/// Indicates that the [Activity::object] has deleted the [Activity::object].
/// If specified, the [Activity::origin] indicates the context from which the [Activity::object] was deleted.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally deleted a note",
///   "type": "Delete",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1",
///   "origin": {
///     "type": "Collection",
///     "name": "Sally's Notes"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Delete {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Delete, DeleteSubtypes, [Activity, Object], { Delete });
