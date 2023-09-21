use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {
    #[serde(flatten)]
    _super: Document,
}

def_subtypes!(Video, VideoSubtypes, [Document, Object], { Video });
