use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Ignore, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block {
    #[serde(flatten)]
    pub _super: Ignore,
}

def_subtypes!(Block, BlockSubtypes, [Ignore, Activity, Object], { Block });
