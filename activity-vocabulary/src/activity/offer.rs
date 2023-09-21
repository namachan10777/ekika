use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Invite, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Offer {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Offer, OfferSubtypes, [Activity, Object], { Offer, Invite });
