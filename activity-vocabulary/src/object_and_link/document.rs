use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Audio, Image, Object, Page, Video};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Document {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(
    Document,
    DocumentSubtypes,
    [Object],
    {
        Document,
        Audio,
        Image,
        Video,
        Page
    }
);
