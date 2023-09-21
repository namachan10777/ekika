use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Invite, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer)
///
/// uri: `https://www.w3.org/ns/activitystreams#Offer`
///
/// Indicates that the [Activity::actor] is offering the [Activity::object].
/// If specified, the [Activity::target] indicates the entity to which the [Activity::object] is being offered.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally offered 50% off to Lewis",
///   "type": "Offer",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "http://www.types.example/ProductOffer",
///     "name": "50% Off!"
///   },
///   "target": {
///     "type": "Person",
///     "name": "Lewis"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Offer {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Offer, OfferSubtypes, [Activity, Object], { Offer, Invite });
