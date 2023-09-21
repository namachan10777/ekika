use serde::{Deserialize, Serialize};

use super::*;
use crate::def_subtypes;
use crate::RemotableObjectOrLinkProp;

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-accept)
///
/// uri: `https://www.w3.org/ns/activitystreams#Accept`
///
/// Indicates that the [Activity::actor] accepts the [Activity::object].
/// The [Activity::target] property can be used in certain circumstances
/// to indicate the context into which the [Activity::object] has been accepted.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally accepted an invitation to a party",
///   "type": "Accept",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Invite",
///     "actor": "http://john.example.org",
///     "object": {
///       "type": "Event",
///       "name": "Going-Away Party for Jim"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Accept {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Accept, AcceptSubtypes, [Activity, Object], {Accept, TentativeAccept});

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-add)
///
/// uri: `https://www.w3.org/ns/activitystreams#Add`
///
/// Indicates that the [Activity::actor] has added the [Activity::object] to the [Activity::target].
/// If the [Activity::target] property is not explicitly specified,
/// the target would need to be determined implicitly by context.
/// The [Activity::origin] can be used to identify the context from which the [Activity::object] originated.
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Add {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Add, AddSubtypes, [Activity, Object], { Add });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-announce)
///
/// uri: `https://www.w3.org/ns/activitystreams#Announce`
///
///
/// Indicates that the [Activity::actor] is calling the [Activity::target]'s attention the [Activity::object].
/// The [Activity::origin] typically has no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally announced that she had arrived at work",
///   "type": "Announce",
///   "actor": {
///     "type": "Person",
///     "id": "http://sally.example.org",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Arrive",
///     "actor": "http://sally.example.org",
///     "location": {
///       "type": "Place",
///       "name": "Work"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Announce {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Announce, AnnounceSubtypes, [Activity, Object], { Announce });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-arrive)
///
/// uri: `https://www.w3.org/ns/activitystreams#Arrive`
///
/// An [IntransitiveActivity] that indicates that the [Activity::actor] has arrived at the location.
/// The [Activity::object] can be used to identify the context from which the [Activity::actor] originated.
/// The [Activity::target] typically has no defined meaning.
///
/// ```json
/// {
//   "@context": "https://www.w3.org/ns/activitystreams",
//   "summary": "Sally arrived at work",
//   "type": "Arrive",
//   "actor": {
//     "type": "Person",
//     "name": "Sally"
//   },
//   "location": {
//     "type": "Place",
//     "name": "Work"
//   },
//   "origin": {
//     "type": "Place",
//     "name": "Home"
//   }
// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Arrive {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
}

def_subtypes!(
    Arrive,
    ArriveSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Arrive }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-block)
///
/// uri: `https://www.w3.org/ns/activitystreams#Block`
///
/// Indicates that the actor is blocking the [Activity::object].
/// Blocking is a stronger form of [Ignore].
/// The typical
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

/// [W3c recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-create)
///
/// uri: `https://www.w3.org/ns/activitystreams#Create`
///
/// Indicates that the [Activity::actor] has created the [Activity::object].
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally created a note",
///   "type": "Create",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Note",
///     "name": "A Simple Note",
///     "content": "This is a simple note"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Create {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Create, CreateSubtypes, [Activity, Object], { Create });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-delete)
///
/// uri: `https://www.w3.org/ns/activitystreams#Delete`
///
/// Indicates that the [Activity::object] has deleted the [Activity::object].
/// If specified, the [Activity::origin] indicates the context from which the [Activity::object] was deleted.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally deleted a note",
///   "type": "Delete",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1",
///   "origin": {
///     "type": "Collection",
///     "name": "Sally's Notes"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Delete {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Delete, DeleteSubtypes, [Activity, Object], { Delete });

/// [W3C recommendation]()
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
pub struct Dislike {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Dislike, DislikeSubtypes, [Activity, Object], { Dislike });

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

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-follow)
///
/// uri: `https://www.w3.org/ns/activitystreams#Follow`
///
/// Indicates that the [Activity::actor] is "following" the [Activity::object].
/// Following is defined in the sense typically used within Social systems
/// in which the [Activity::actor] is interested in any activity performed by or on the object.
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
/// ```json
/// {
//   "@context": "https://www.w3.org/ns/activitystreams",
//   "summary": "Sally followed John",
//   "type": "Follow",
//   "actor": {
//     "type": "Person",
//     "name": "Sally"
//   },
//   "object": {
//     "type": "Person",
//     "name": "John"
//   }
// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Follow {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Follow, FollowSubtypes, [Activity, Object], { Follow });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-ignore)
///
/// uri: `https://www.w3.org/ns/activitystreams#Ignore`
///
/// Indicates that the [Activity::actor] is ignoring the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally ignored a note",
///   "type": "Ignore",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Ignore {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Ignore, IgnoreSubtypes, [Activity, Object], { Ignore, Block });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-invite)
///
/// uri: `https://www.w3.org/ns/activitystreams#Invite`
///
/// A specialization of [Offer] in which the [Activity::actor] is extending an invitation for the [Activity::object] to the [Activity::target].
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally invited John and Lisa to a party",
///   "type": "Invite",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Event",
///     "name": "A Party"
///   },
///   "target": [
///     {
///       "type": "Person",
///       "name": "John"
///     },
///     {
///       "type": "Person",
///       "name": "Lisa"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Invite {
    #[serde(flatten)]
    pub _super: Offer,
}

def_subtypes!(Invite, InviteSubtypes, [Offer, Activity, Object], {
    Invite
});

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-join)
///
/// uri: `https://www.w3.org/ns/activitystreams#Join`
///
/// Indicates that the [Activity::actor] has joined the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally joined a group",
///   "type": "Join",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Group",
///     "name": "A Simple Group"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Join {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Join, JoinSubtypes, [Activity, Object], { Join });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-leave)
///
/// uri: `https://www.w3.org/ns/activitystreams#Leave`
///
/// Indicates that the [Activity::actor] has left the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no meaning.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally left work",
///   "type": "Leave",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Place",
///     "name": "Work"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Leave {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Leave, LeaveSubtypes, [Activity, Object], { Leave });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-like)
///
/// uri: `https://www.w3.org/ns/activitystreams#Like`
///
/// Indicates that the [Activity::actor] likes,
/// recommends or endorses the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally liked a note",
///   "type": "Like",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1"
/// }
/// ```

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Like {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Like, LikeSubtypes, [Activity, Object], { Like });

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

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-move)
///
/// uri: `https://www.w3.org/ns/activitystreams#Move`
///
/// Indicates that the [Activity::actor] has moved [Activity::object] from [Activity::origin] to [Activity::target].
/// If the [Activity::origin] or [Activity::target] are not specified, either can be determined by context.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally moved a post from List A to List B",
///   "type": "Move",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/posts/1",
///   "target": {
///     "type": "Collection",
///     "name": "List B"
///   },
///   "origin": {
///     "type": "Collection",
///     "name": "List A"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Move {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Move, MoveSubtypes, [Activity, Object], { Move });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-offer)
///
/// uri: `https://www.w3.org/ns/activitystreams#Offer`
///
/// Indicates that the [Activity::actor] is offering the [Activity::object].
/// If specified, the [Activity::target] indicates the entity to which the [Activity::object] is being offered.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally offered 50% off to Lewis",
///   "type": "Offer",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "http://www.types.example/ProductOffer",
///     "name": "50% Off!"
///   },
///   "target": {
///     "type": "Person",
///     "name": "Lewis"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Offer {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Offer, OfferSubtypes, [Activity, Object], { Offer, Invite });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question)
///
/// uri: `https://www.w3.org/ns/activitystreams#Question`
///
/// Represents a question being asked.
/// Question objects are an extension of [IntransitiveActivity].
/// That is, the Question object is an [Activity],
/// but the direct object is the question itself and therefore it would not contain an [Activity::object] property.
///
/// Either of the [Question::any_of] and [Question::one_of] properties may be used to express possible answers,
/// but a Question object **must not** have both properties.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Question",
///   "name": "What is the answer?",
///   "oneOf": [
///     {
///       "type": "Note",
///       "name": "Option A"
///     },
///     {
///       "type": "Note",
///       "name": "Option B"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Question {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-anyof)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#anyOf`
    ///
    /// Identifies an inclusive option for a [Question].
    ///
    /// To indicate that a [Question] can have only one answer,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub any_of: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-oneof)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#oneOf`
    ///
    /// Identifies an exclusive option for a [Question].
    ///
    /// To indicate that a [Question] can have multiple answers,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub one_of: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-closed)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#closed`
    ///
    /// Describes an indirect object of the activity from which the activity is directed.
    /// The precise meaning of the origin is the object of the English preposition "from".
    /// For instance, in the activity "John moved an item to List B from List A", the origin of the activity is "List A".
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub closed: RemotableObjectOrLinkProp,
}

def_subtypes!(
    Question,
    QuestionSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Question }
);

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

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-reject)
///
/// uri: `https://www.w3.org/ns/activitystreams#Reject`
///
/// Indicates that the [Activity::actor] is rejecting the [Activity::object].
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally rejected an invitation to a party",
///   "type": "Reject",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Invite",
///     "actor": "http://john.example.org",
///     "object": {
///       "type": "Event",
///       "name": "Going-Away Party for Jim"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reject {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Reject, RejectSubtypes, [Activity, Object], {Reject, TentativeReject});

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-remove)
///
/// uri: `https://www.w3.org/ns/activitystreams#Remove`
///
/// Indicates that the [Activity::actor] is removing the [Activity::object].
/// If specified, the [Activity::origin] indicates the context from which the [Activity::object] is being removed.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally removed a note from her notes folder",
///   "type": "Remove",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1",
///   "target": {
///     "type": "Collection",
///     "name": "Notes Folder"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Remove {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Remove, RemoveSubtypes, [Activity, Object], { Remove });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativeaccept)
///
/// uri: `https://www.w3.org/ns/activitystreams#TentativeAccept`
///
/// A specialization of [Accept] indicating that the acceptance is tentative.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally tentatively accepted an invitation to a party",
///   "type": "TentativeAccept",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Invite",
///     "actor": "http://john.example.org",
///     "object": {
///       "type": "Event",
///       "name": "Going-Away Party for Jim"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TentativeAccept {
    #[serde(flatten)]
    pub _super: Accept,
}

def_subtypes!(
    TentativeAccept,
    IntransitiveAcceptSubtypes,
    [Accept, Activity, Object],
    { TentativeAccept }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tentativereject)
///
/// uri: `https://www.w3.org/ns/activitystreams#TentativeReject`
///
/// A specialization of [Reject] in which the rejection is considered tentative.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally tentatively rejected an invitation to a party",
///   "type": "TentativeReject",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": {
///     "type": "Invite",
///     "actor": "http://john.example.org",
///     "object": {
///       "type": "Event",
///       "name": "Going-Away Party for Jim"
///     }
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TentativeReject {
    #[serde(flatten)]
    pub _super: Reject,
}

def_subtypes!(
    TentativeReject,
    TentativeRejectSubtypes,
    [Reject, Activity, Object],
    { TentativeReject }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-travel)
///
/// uri: `https://www.w3.org/ns/activitystreams#Travel`
///
/// Indicates that the [Activity::actor] is traveling to [Activity::target] from [Activity::origin].
/// [Travel] is an [IntransitiveActivity] whose [Activity::actor] specifies the direct object.
/// If the [Activity::target] or [Activity::origin] are not specified, either can be determined by context.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally went home from work",
///   "type": "Travel",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "target": {
///     "type": "Place",
///     "name": "Home"
///   },
///   "origin": {
///     "type": "Place",
///     "name": "Work"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Travel {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
}

def_subtypes!(
    Travel,
    TravelSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Travel }
);

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

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-update)
///
/// uri: `https://www.w3.org/ns/activitystreams#Update`
///
///
/// Indicates that the [Activity::actor] has updated the [Activity::object]. Note,
/// however, that this vocabulary does not define a mechanism for describing the actual set of modifications made to [Activity::object].
///
/// The [Activity::target] and [Activity::origin] typically have no defined meaning.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally updated her note",
///   "type": "Update",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "object": "http://example.org/notes/1"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Update {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Update, UpdateSubtypes, [Activity, Object], { Update });

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
