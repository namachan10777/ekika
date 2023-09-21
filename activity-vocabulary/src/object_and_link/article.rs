use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Article {
    #[serde(flatten)]
    _super: Object,
}

def_subtypes!(Article, ArticleSubtypes, [Object], { Article });
