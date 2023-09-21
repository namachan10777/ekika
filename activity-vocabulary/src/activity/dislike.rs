use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Dislike {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Dislike, DislikeSubtypes, [Activity, Object], { Dislike });
