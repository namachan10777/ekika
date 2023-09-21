use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move)
///
/// uri: `https://www.w3.org/ns/activitystreams#Move`
///
/// Indicates that the [Activity::actor] has moved [Activity::object] from [Activity::origin] to [Activity::target].
/// If the [Activity::origin] or [Activity::target] are not specified, either can be determined by context.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally moved a post from List A to List B",
///   "type": "Move",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/posts/1",
///   "target": {
///     "type": "Collection",
///     "name": "List B"
///   },
///   "origin": {
///     "type": "Collection",
///     "name": "List A"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Move {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Move, MoveSubtypes, [Activity, Object], { Move });
