use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove)
///
/// uri: `https://www.w3.org/ns/activitystreams#Remove`
///
/// Indicates that the [Activity::actor] is removing the [Activity::object].
/// If specified, the [Activity::origin] indicates the context from which the [Activity::object] is being removed.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally removed a note from her notes folder",
///   "type": "Remove",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1",
///   "target": {
///     "type": "Collection",
///     "name": "Notes Folder"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Remove {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Remove, RemoveSubtypes, [Activity, Object], { Remove });
