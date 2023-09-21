use serde::{Deserialize, Serialize};

use crate::{def_subtypes, FunctionalProperty, Object, Remotable};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    #[serde(flatten)]
    _super: Object,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    describes: FunctionalProperty<Remotable<Object>>,
}

def_subtypes!(Profile, ProfileSubtypes, [Object], { Profile });
