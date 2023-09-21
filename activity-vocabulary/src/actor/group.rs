use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Group {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Group, GroupSubtypes, [Object], { Group });
