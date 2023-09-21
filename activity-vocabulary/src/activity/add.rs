use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Add {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Add, AddSubtypes, [Activity, Object], { Add });
