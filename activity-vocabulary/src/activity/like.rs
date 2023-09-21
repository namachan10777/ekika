use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Like {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Like, LikeSubtypes, [Activity, Object], { Like });