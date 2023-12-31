pub enum AccountKind {
    Person,
}

pub struct Account {
    pub kind: AccountKind,
    pub preffered_user_name: String,
    pub name: String,
    pub summary: String,
    pub icon: Vec<url::Url>,
}
