use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Note {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Note, NoteSubtypes, [Object], { Note });
