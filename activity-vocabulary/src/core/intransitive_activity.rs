use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{Activity, Object};

/// ## Caution
/// W3C recommendation says IntransitiveActivity has no properties that come from Object.
/// But this definition accepts any properties that Object has.
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

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum IntransitiveActivitySubtypes {
    IntransitiveActivity(IntransitiveActivity),
}

impl From<IntransitiveActivitySubtypes> for IntransitiveActivity {
    fn from(value: IntransitiveActivitySubtypes) -> Self {
        match value {
            IntransitiveActivitySubtypes::IntransitiveActivity(x) => x.into(),
        }
    }
}

impl From<IntransitiveActivity> for Activity {
    fn from(value: IntransitiveActivity) -> Self {
        value._super
    }
}

impl From<IntransitiveActivity> for Object {
    fn from(value: IntransitiveActivity) -> Self {
        value._super.into()
    }
}
