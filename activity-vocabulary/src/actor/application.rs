use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application)
///
/// uri: `https://www.w3.org/ns/activitystreams#Application`
///
/// Describes a software application.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Application",
///   "name": "Exampletron 3000"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Application, ApplicationSubtypes, [Object], { Application });
