use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-view)
///
/// uri: `https://www.w3.org/ns/activitystreams#View`
///
/// Indicates that the [Activity::actor] has viewed the [Activity::object].
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally read an article",
///   "type": "View",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Article",
///     "name": "What You Should Know About Activity Streams"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct View {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(View, ViewSubtypes, [Activity, Object], { View });
