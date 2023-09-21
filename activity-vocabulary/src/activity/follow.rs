use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow)
///
/// uri: `https://www.w3.org/ns/activitystreams#Follow`
///
/// Indicates that the [Activity::actor] is "following" the [Activity::object].
/// Following is defined in the sense typically used within Social systems
/// in which the [Activity::actor] is interested in any activity performed by or on the object.
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
/// ```json
/// {
//   "@context": "https://www.w3.org/ns/activitystreams",
//   "summary": "Sally followed John",
//   "type": "Follow",
//   "actor": {
//     "type": "Person",
//     "name": "Sally"
//   },
//   "object": {
//     "type": "Person",
//     "name": "John"
//   }
// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Follow {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Follow, FollowSubtypes, [Activity, Object], { Follow });
