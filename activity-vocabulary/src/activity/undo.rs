use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Undo {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Undo, UndoSubtypes, [Activity, Object], { Undo });
