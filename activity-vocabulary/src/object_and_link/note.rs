use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-note)
///
/// uri: `https://www.w3.org/ns/activitystreams#Note`
///
/// Represents a short written work typically less than a single paragraph in length.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Note",
///   "name": "A Word of Warning",
///   "content": "Looks like it is going to rain today. Bring an umbrella!"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Note {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Note, NoteSubtypes, [Object], { Note });
