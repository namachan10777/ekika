use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    #[serde(flatten)]
    _super: Object,
}

def_subtypes!(Application, ApplicationSubtypes, [Object], { Application });
