use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Ignore, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block)
///
/// uri: `https://www.w3.org/ns/activitystreams#Block`
///
/// Indicates that the actor is blocking the [Activity::object].
/// Blocking is a stronger form of [Ignore].
/// The typical use is to support social systems that allow one user to block activities or content of other users.
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
//   "@context": "https://www.w3.org/ns/activitystreams",
//   "summary": "Sally blocked Joe",
//   "type": "Block",
//   "actor": "http://sally.example.org",
//   "object": "http://joe.example.org"
// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Block {
    #[serde(flatten)]
    pub _super: Ignore,
}

def_subtypes!(Block, BlockSubtypes, [Ignore, Activity, Object], { Block });
