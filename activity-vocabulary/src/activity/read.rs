use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Read {
    pub _super: Activity,
}

def_subtypes!(Read, ReadSubtypes, [Activity, Object], { Read });
