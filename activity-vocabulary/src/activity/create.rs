use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3c recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create)
///
/// uri: `https://www.w3.org/ns/activitystreams#Create`
///
/// Indicates that the [Activity::actor] has created the [Activity::object].
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally created a note",
///   "type": "Create",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Note",
///     "name": "A Simple Note",
///     "content": "This is a simple note"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Create {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Create, CreateSubtypes, [Activity, Object], { Create });
