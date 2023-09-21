use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-flag)
///
/// uri: `https://www.w3.org/ns/activitystreams#Dislike`
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally disliked a post",
///   "type": "Dislike",
///   "actor": "http://sally.example.org",
///   "object": "http://example.org/posts/1"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Flag {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Flag, FlagSubtypes, [Activity, Object], { Flag });
