use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Delete {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Delete, DeleteSubtypes, [Activity, Object], { Delete });
