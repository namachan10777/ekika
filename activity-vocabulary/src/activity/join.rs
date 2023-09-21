use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Join {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Join, JoinSubtypes, [Activity, Object], { Join });
