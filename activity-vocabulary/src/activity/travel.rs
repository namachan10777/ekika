use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, IntransitiveActivity, Object};

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
