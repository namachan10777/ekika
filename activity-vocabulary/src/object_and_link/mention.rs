use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Link};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mention)
///
/// uri: `https://www.w3.org/ns/activitystreams#Mention`
///
/// A specialized [Link] that represents an @mention.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Mention of Joe by Carrie in her note",
///   "type": "Mention",
///   "href": "http://example.org/joe",
///   "name": "Joe"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Mention {
    #[serde(flatten)]
    pub _super: Link,
}

def_subtypes!(Mention, MentionSubtypes, [Link], { Mention });
