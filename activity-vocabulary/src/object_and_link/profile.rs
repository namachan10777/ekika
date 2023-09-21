use serde::{Deserialize, Serialize};

use crate::{def_subtypes, FunctionalProperty, Object, Remotable};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-profile)
///
/// uri: `https://www.w3.org/ns/activitystreams#Profile`
///
/// A Profile is a content object that describes another Object, typically used to describe
/// [Actor Type](https://www.w3.org/TR/activitystreams-vocabulary/#actor-types) objects.
/// The [Profile::describes] property is used to reference the object being described by the profile.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Profile",
///   "summary": "Sally's Profile",
///   "describes": {
///     "type": "Person",
///     "name": "Sally Smith"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Profile {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-describes)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#describes`
    ///
    /// On a [Profile] object, the [Profile::describes] property identifies the object described by the Profile.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub describes: FunctionalProperty<Remotable<Object>>,
}

def_subtypes!(Profile, ProfileSubtypes, [Object], { Profile });
