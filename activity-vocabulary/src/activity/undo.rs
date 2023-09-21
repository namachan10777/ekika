use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object};

/// [W3C recommendation]
///
/// uri: `https://www.w3.org/ns/activitystreams#Undo`
///
/// Indicates that the [Activity::object] is undoing the [Activity::object].
/// In most cases, the [Activity::object] will be an [Activity] describing some previously performed action
/// (for instance, a person may have previously "liked" an article but, for whatever reason,
/// might choose to undo that like at some later point in time).
///
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally retracted her offer to John",
///   "type": "Undo",
///   "actor": "http://sally.example.org",
///   "object": {
///     "type": "Offer",
///     "actor": "http://sally.example.org",
///     "object": "http://example.org/posts/1",
///     "target": "http://john.example.org"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Undo {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Undo, UndoSubtypes, [Activity, Object], { Undo });
