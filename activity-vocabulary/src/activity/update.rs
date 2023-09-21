use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update)
///
/// uri: `https://www.w3.org/ns/activitystreams#Update`
///
///
/// Indicates that the [Activity::actor] has updated the [Activity::object]. Note,
/// however, that this vocabulary does not define a mechanism for describing the actual set of modifications made to [Activity::object].
///
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally updated her note",
///   "type": "Update",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Update {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Update, UpdateSubtypes, [Activity, Object], { Update });
