use serde::{Deserialize, Serialize};

use crate::*;

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-activity)
///
/// uri: `https://www.w3.org/ns/activitystreams#Activity`
///
/// An [Activity] is a subtype of [Object] that describes some form of action that may happen,
/// is currently happening, or has already happened.
/// The [Activity] type itself serves as an abstract base type for all types of activities.
/// It is important to note that the [Activity] type itself does not carry any specific semantics about the kind of action being taken.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Activity",
///   "summary": "Sally did something to a note",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Note",
///     "name": "A Note"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct Activity {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-actor)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#actor`
    ///
    /// Describes one or more entities that either performed or are expected to perform the activity.
    /// Any single activity can have multiple [Activity::actor]s. The [Activity::actor] may be specified using an indirect [crate::Link].
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub actor: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-object)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#object`
    ///
    /// When used within an [Activity], describes the direct object of the activity. For instance,
    /// in the activity "John added a movie to his wishlist", the object of the activity is the movie added.
    /// When used within a [Relationship] describes the entity to which the [Relationship::subject] is related.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub object: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-target)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#target`
    ///
    /// Describes the indirect object, or target, of the activity.
    /// The precise meaning of the target is largely dependent
    /// on the type of action being described but will often be the object of the English preposition "to".
    /// For instance, in the activity "John added a movie to his wishlist",
    /// the target of the activity is John's wishlist. An activity can have more than one target.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub target: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-result)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#result`
    ///
    /// Describes the result of the activity.
    /// For instance, if a particular action results in the creation of a new resource,
    /// the result property can be used to describe that new resource.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub result: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-origin)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#origin`
    ///
    /// Describes an indirect object of the activity from which the activity is directed.
    /// The precise meaning of the origin is the object of the English preposition "from".
    /// For instance, in the activity "John moved an item to List B from List A",
    /// the origin of the activity is "List A".
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub origin: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-instrument)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#instrument`
    ///
    /// Identifies one or more objects used (or to be used) in the completion of an [Activity].
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub instrument: RemotableObjectOrLinkProp,
}

def_subtypes!(
    Activity,
    ActivitySubtypes,
    [Object],
    {
        Activity,
        IntransitiveActivity,
        Accept,
        TentativeAccept,
        Add,
        Announce,
        Arrive,
        Block,
        Create,
        Delete,
        Dislike,
        Flag,
        Follow,
        Ignore,
        Invite,
        Join,
        Leave,
        Like,
        Listen,
        Move,
        Offer,
        Question,
        Read,
        Reject,
        TentativeReject,
        Remove,
        Undo,
        Travel,
        Update
    }
);
