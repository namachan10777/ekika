use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-read)
///
/// uri: `https://www.w3.org/ns/activitystreams#Read`
///
/// Indicates that the [Activity::actor] has read the [Activity::object].
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally read a blog post",
///   "type": "Read",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/posts/1"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Read {
    pub _super: Activity,
}

def_subtypes!(Read, ReadSubtypes, [Activity, Object], { Read });
