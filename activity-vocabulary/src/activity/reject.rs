use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, TentativeReject};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject)
///
/// uri: `https://www.w3.org/ns/activitystreams#Reject`
///
/// Indicates that the [Activity::actor] is rejecting the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally rejected an invitation to a party",
///   "type": "Reject",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Invite",
///     "actor": "http://john.example.org",
///     "object": {
///       "type": "Event",
///       "name": "Going-Away Party for Jim"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reject {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Reject, RejectSubtypes, [Activity, Object], {Reject, TentativeReject});
