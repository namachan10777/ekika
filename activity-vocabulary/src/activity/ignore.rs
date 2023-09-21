use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Block, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore)
///
/// uri: `https://www.w3.org/ns/activitystreams#Ignore`
///
/// Indicates that the [Activity::actor] is ignoring the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally ignored a note",
///   "type": "Ignore",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ignore {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Ignore, IgnoreSubtypes, [Activity, Object], { Ignore, Block });
