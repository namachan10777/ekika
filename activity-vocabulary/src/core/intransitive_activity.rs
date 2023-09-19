use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// ## Caution
/// W3C recommendation says IntransitiveActivity has no properties that come from Object.
/// But this definition accepts any properties that Object has.
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(
    IntransitiveActivity,
    IntransitiveActivitySubtypes,
    [Object, Activity],
    { IntransitiveActivity }
);
