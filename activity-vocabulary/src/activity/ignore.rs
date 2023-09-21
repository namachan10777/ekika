use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Block, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ignore {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Ignore, IgnoreSubtypes, [Activity, Object], { Ignore, Block });
