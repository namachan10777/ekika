use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce)
///
/// uri: `https://www.w3.org/ns/activitystreams#Announce`
///
///
/// Indicates that the [Activity::actor] is calling the [Activity::target]'s attention the [Activity::object].
/// The [Activity::origin] typically has no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally announced that she had arrived at work",
///   "type": "Announce",
///   "actor": {
///     "type": "Person",
///     "id": "http://sally.example.org",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Arrive",
///     "actor": "http://sally.example.org",
///     "location": {
///       "type": "Place",
///       "name": "Work"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Announce {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Announce, AnnounceSubtypes, [Activity, Object], { Announce });
