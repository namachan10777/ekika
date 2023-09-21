use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Move {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Move, MoveSubtypes, [Activity, Object], { Move });
