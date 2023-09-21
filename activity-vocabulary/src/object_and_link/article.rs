use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-article)
///
/// uri: `https://www.w3.org/ns/activitystreams#Article`
///
/// Represents any kind of multi-paragraph written work.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Article",
///   "name": "What a Crazy Day I Had",
///   "content": "<div>... you will never believe ...</div>",
///   "attributedTo": "http://sally.example.org"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Article {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Article, ArticleSubtypes, [Object], { Article });
