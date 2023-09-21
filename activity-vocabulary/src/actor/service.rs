use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service)
///
/// uri: `https://www.w3.org/ns/activitystreams#Service`
///
/// Represents a service of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Service",
///   "name": "Acme Web Service"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Service {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Service, ServiceSubtypes, [Object], { Service });
