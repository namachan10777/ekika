use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Flag {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Flag, FlagSubtypes, [Activity, Object], { Flag });
