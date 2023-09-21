use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Leave {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Leave, LeaveSubtypes, [Activity, Object], { Leave });
