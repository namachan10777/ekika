use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-listen)
///
/// uri: `https://www.w3.org/ns/activitystreams#Listen`
///
/// Indicates that the [Activity::actor] has listened to the [Activity::object].
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally listened to a piece of music",
///   "type": "Listen",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/music.mp3"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Listen {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Listen, ListenSubtypes, [Activity, Object], { Listen });
