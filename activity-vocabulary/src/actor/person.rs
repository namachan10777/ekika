use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Person {
    #[serde(flatten)]
    _super: Object,
}

def_subtypes!(Person, PersonSubtypes, [Object], { Person });
