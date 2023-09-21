use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person)
///
/// uri: `https://www.w3.org/ns/activitystreams#Person`
///
/// Represents an individual person.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Person",
///   "name": "Sally Smith"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Person {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Person, PersonSubtypes, [Object], { Person });
