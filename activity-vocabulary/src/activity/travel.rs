use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, IntransitiveActivity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel)
///
/// uri: `https://www.w3.org/ns/activitystreams#Travel`
///
/// Indicates that the [Activity::actor] is traveling to [Activity::target] from [Activity::origin].
/// [Travel] is an [IntransitiveActivity] whose [Activity::actor] specifies the direct object.
/// If the [Activity::target] or [Activity::origin] are not specified, either can be determined by context.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally went home from work",
///   "type": "Travel",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "target": {
///     "type": "Place",
///     "name": "Home"
///   },
///   "origin": {
///     "type": "Place",
///     "name": "Work"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Travel {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
}

def_subtypes!(
    Travel,
    TravelSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Travel }
);
