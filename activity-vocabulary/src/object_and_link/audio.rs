use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Audio {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Audio, AudioSubtypes, [Document, Object], { Audio });
