use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(flatten)]
    _super: Document,
}

def_subtypes!(Image, ImageSubtypes, [Document, Object], { Image });
