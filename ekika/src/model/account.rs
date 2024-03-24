use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
pub enum AccountKind {
    Person,
}

#[derive(Clone, Deserialize, Serialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Account {
    pub kind: AccountKind,
    pub preferred_user_name: String,
    pub name: String,
    pub summary: String,
    pub icon: Vec<url::Url>,
}
