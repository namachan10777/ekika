use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Follow {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Follow, FollowSubtypes, [Activity, Object], { Follow });
