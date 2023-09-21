use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, TentativeAccept};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept)
///
/// uri: `https://www.w3.org/ns/activitystreams#Accept`
///
/// Indicates that the [Activity::actor] accepts the [Activity::object].
/// The [Activity::target] property can be used in certain circumstances
/// to indicate the context into which the [Activity::object] has been accepted.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally accepted an invitation to a party",
///   "type": "Accept",
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
pub struct Accept {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Accept, AcceptSubtypes, [Activity, Object], {Accept, TentativeAccept});
