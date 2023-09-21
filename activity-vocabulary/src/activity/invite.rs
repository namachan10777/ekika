use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, Offer};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Invite {
    #[serde(flatten)]
    pub _super: Offer,
}

def_subtypes!(Invite, InviteSubtypes, [Offer, Activity, Object], {
    Invite
});
