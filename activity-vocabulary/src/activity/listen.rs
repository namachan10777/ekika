use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Listen {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Listen, ListenSubtypes, [Activity, Object], { Listen });
