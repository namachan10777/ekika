use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct Context {
    pub urls: HashSet<url::Url>,
    pub def: ContextDef,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ContextDef {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
}

impl Context {
    pub fn standard(lang: String) -> Self {
        Context {
            urls: ["https://www.w3.org/ns/activitystreams".parse().unwrap()]
                .into_iter()
                .collect(),
            def: ContextDef {
                language: Some(lang),
            },
        }
    }
}
