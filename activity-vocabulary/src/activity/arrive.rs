use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, IntransitiveActivity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive)
///
/// uri: `https://www.w3.org/ns/activitystreams#Arrive`
///
/// An [IntransitiveActivity] that indicates that the [Activity::actor] has arrived at the location.
/// The [Activity::object] can be used to identify the context from which the [Activity::actor] originated.
/// The [Activity::target] typically has no defined meaning.
///
/// ```json
/// {
//   "@context": "https://www.w3.org/ns/activitystreams",
//   "summary": "Sally arrived at work",
//   "type": "Arrive",
//   "actor": {
//     "type": "Person",
//     "name": "Sally"
//   },
//   "location": {
//     "type": "Place",
//     "name": "Work"
//   },
//   "origin": {
//     "type": "Place",
//     "name": "Home"
//   }
// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Arrive {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
}

def_subtypes!(
    Arrive,
    ArriveSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Arrive }
);
