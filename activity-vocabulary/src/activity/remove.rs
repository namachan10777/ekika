use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Remove {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Remove, RemoveSubtypes, [Activity, Object], { Remove });
