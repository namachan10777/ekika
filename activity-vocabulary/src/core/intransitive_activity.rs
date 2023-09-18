use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::Activity;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub _super: Activity,
}

impl Deref for IntransitiveActivity {
    type Target = Activity;
    fn deref(&self) -> &Self::Target {
        &self._super
    }
}

impl From<IntransitiveActivity> for Activity {
    fn from(value: IntransitiveActivity) -> Self {
        value._super
    }
}
