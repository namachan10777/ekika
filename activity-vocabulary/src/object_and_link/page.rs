use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Page, PageSubtypes, [Document, Object], { Page });
