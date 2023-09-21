use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join)
///
/// uri: `https://www.w3.org/ns/activitystreams#Join`
///
/// Indicates that the [Activity::actor] has joined the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally joined a group",
///   "type": "Join",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Group",
///     "name": "A Simple Group"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Join {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Join, JoinSubtypes, [Activity, Object], { Join });
