use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object, RemotableObjectOrLinkProp, RemotableProperty};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship)
///
/// uri: `https://www.w3.org/ns/activitystreams#Relationship`
///
/// Describes a relationship between two individuals.
/// The [Relationship::subject] and [Relationship::object] properties are used to identify the connected individuals.
/// See [5.2 Representing Relationships Between Entities](https://www.w3.org/TR/activitystreams-vocabulary/#connections) for additional information.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally is an acquaintance of John",
///   "type": "Relationship",
///   "subject": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "relationship": "http://purl.org/vocab/relationship/acquaintanceOf",
///   "object": {
///     "type": "Person",
///     "name": "John"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Relationship {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-subject)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#subject`
    ///
    /// On a [Relationship] object, the [Relationship::subject] property identifies one of the connected individuals.
    /// For instance, for a [Relationship] object describing "John is related to Sally", [Relationship::subject] would refer to John.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub subject: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-object)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#object`
    ///
    /// When used within an [crate::Activity], describes the direct object of the activity.
    /// For instance, in the activity "John added a movie to his wishlist", the object of the activity is the movie added.
    /// When used within a [Relationship] describes the entity to which the [Relationship::subject] is related.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub object: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-relationship)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#relationship`
    ///
    /// On a [Relationship] object,
    /// the [Relationship::relationship] property identifies the kind of relationship that exists between [Relationship::subject] and [Relationship::object].
    #[serde(skip_serializing_if = "RemotableProperty::is_empty")]
    pub relationship: RemotableProperty<Object>,
}

def_subtypes!(Relationship, RelationShipSubtypes, [Object], {
    Relationship
});
