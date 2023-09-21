use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Update {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Update, UpdateSubtypes, [Activity, Object], { Update });
