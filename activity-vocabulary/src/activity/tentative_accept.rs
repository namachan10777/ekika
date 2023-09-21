use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Accept, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept)
///
/// uri: `https://www.w3.org/ns/activitystreams#TentativeAccept`
///
/// A specialization of [Accept] indicating that the acceptance is tentative.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally tentatively accepted an invitation to a party",
///   "type": "TentativeAccept",
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
pub struct TentativeAccept {
    #[serde(flatten)]
    pub _super: Accept,
}

def_subtypes!(
    TentativeAccept,
    IntransitiveAcceptSubtypes,
    [Accept, Activity, Object],
    { TentativeAccept }
);
