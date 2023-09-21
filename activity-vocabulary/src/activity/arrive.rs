use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, IntransitiveActivity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Arrive {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
}

def_subtypes!(
    Arrive,
    ArriveSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Arrive }
);
