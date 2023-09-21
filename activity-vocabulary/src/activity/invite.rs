use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, Offer};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite)
///
/// uri: `https://www.w3.org/ns/activitystreams#Invite`
///
/// A specialization of [Offer] in which the [Activity::actor] is extending an invitation for the [Activity::object] to the [Activity::target].
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally invited John and Lisa to a party",
///   "type": "Invite",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Event",
///     "name": "A Party"
///   },
///   "target": [
///     {
///       "type": "Person",
///       "name": "John"
///     },
///     {
///       "type": "Person",
///       "name": "Lisa"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Invite {
    #[serde(flatten)]
    pub _super: Offer,
}

def_subtypes!(Invite, InviteSubtypes, [Offer, Activity, Object], {
    Invite
});
