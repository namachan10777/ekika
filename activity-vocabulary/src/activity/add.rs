use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add)
///
/// uri: `https://www.w3.org/ns/activitystreams#Add`
///
/// Indicates that the [Activity::actor] has added the [Activity::object] to the [Activity::target].
/// If the [Activity::target] property is not explicitly specified,
/// the target would need to be determined implicitly by context.
/// The [Activity::origin] can be used to identify the context from which the [Activity::object] originated.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Add {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Add, AddSubtypes, [Activity, Object], { Add });
