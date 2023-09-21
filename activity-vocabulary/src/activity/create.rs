use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Create {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Create, CreateSubtypes, [Activity, Object], { Create });
