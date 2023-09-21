use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, Reject};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject)
///
/// uri: `https://www.w3.org/ns/activitystreams#TentativeReject`
///
/// A specialization of [Reject] in which the rejection is considered tentative.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally tentatively rejected an invitation to a party",
///   "type": "TentativeReject",
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
pub struct TentativeReject {
    #[serde(flatten)]
    pub _super: Reject,
}

def_subtypes!(
    TentativeReject,
    TentativeRejectSubtypes,
    [Reject, Activity, Object],
    { TentativeReject }
);
