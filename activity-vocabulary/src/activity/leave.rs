use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave)
///
/// uri: `https://www.w3.org/ns/activitystreams#Leave`
///
/// Indicates that the [Activity::actor] has left the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no meaning.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally left work",
///   "type": "Leave",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Place",
///     "name": "Work"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Leave {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Leave, LeaveSubtypes, [Activity, Object], { Leave });
