use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group)
///
/// uri: `https://www.w3.org/ns/activitystreams#Group`
///
/// Represents a formal or informal collective of Actors.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Group",
///   "name": "Big Beards of Austin"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Group {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Group, GroupSubtypes, [Object], { Group });
