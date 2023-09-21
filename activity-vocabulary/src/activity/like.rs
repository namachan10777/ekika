use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like)
///
/// uri: `https://www.w3.org/ns/activitystreams#Like`
///
/// Indicates that the [Activity::actor] likes,
/// recommends or endorses the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally liked a note",
///   "type": "Like",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1"
/// }
/// ```

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Like {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Like, LikeSubtypes, [Activity, Object], { Like });
