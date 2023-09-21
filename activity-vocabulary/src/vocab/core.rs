use serde::{Deserialize, Serialize};
use url::Url;

use super::*;
use crate::xsd::{self, LangContainer};
use crate::{
    def_subtypes, FunctionalProperty, Or, Property, Remotable, RemotableObjectOrLinkProp,
    RemotableOrLinkProp,
};

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

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collection)
///
/// uri: `https://www.w3.org/ns/activitystreams#Collection`
///
/// A [Collection] is a subtype of [Object] that represents ordered or unordered sets of [Object] or [crate::Link] instances.
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#collection) specification for a complete description of the [Collection] type.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally's notes",
///   "type": "Collection",
///   "totalItems": 2,
///   "items": [
///     {
///       "type": "Note",
///       "name": "A Simple Note"
///     },
///     {
///       "type": "Note",
///       "name": "Another Simple Note"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct Collection {
    #[serde(flatten)]
    pub _super: Object,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-totalitems)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#totalItems`
    ///
    /// A non-negative integer specifying the total number of objects contained
    /// by the logical view of the collection.
    /// This number might not reflect the actual number of items serialized within the [Collection] object instance.
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "totalItems"
    )]
    pub total_items: FunctionalProperty<usize>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-current)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#current`
    ///
    /// In a paged [Collection], indicates the page that contains the most recently updated member items.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub current: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-first)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#first`
    ///
    /// In a paged [Collection], indicates the furthest preceeding page of items in the collection.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub first: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-first)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#first`
    ///
    /// In a paged [Collection], indicates the furthest preceeding page of items in the collection.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub last: FunctionalProperty<Or<Box<CollectionPage>, LinkSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-items)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#items`
    ///
    /// Identifies the items contained in a collection. The items might be ordered or unordered.
    #[serde(skip_serializing_if = "RemotableOrLinkProp::is_empty")]
    pub items: RemotableOrLinkProp<Or<ObjectSubtypes, Vec<RemotableObjectOrLinkProp>>>,
}

def_subtypes!(
    Collection,
    CollectionSubtypes,
    [Object],
    {
        Collection,
        CollectionPage,
        OrderedCollection,
        OrderedCollectionPage
    }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-collectionpage)
///
/// uri: `https://www.w3.org/ns/activitystreams#CollectionPage`
///
/// Used to represent distinct subsets of items from a [Collection].
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-collectionpage) for a complete description of the [CollectionPage] object.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Page 1 of Sally's notes",
///   "type": "CollectionPage",
///   "id": "http://example.org/foo?page=1",
///   "partOf": "http://example.org/foo",
///   "items": [
///     {
///       "type": "Note",
///       "name": "A Simple Note"
///     },
///     {
///       "type": "Note",
///       "name": "Another Simple Note"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct CollectionPage {
    #[serde(flatten)]
    pub _super: Collection,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-partof)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#partOf`
    ///
    /// Identifies the [Collection] to which a [CollectionPage] objects items belong.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none", rename = "partOf")]
    part_of: FunctionalProperty<Box<Or<Remotable<Collection>, LinkSubtypes>>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-next)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#next`
    ///
    /// In a paged [Collection], indicates the next page of items.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    next: FunctionalProperty<Box<Or<Remotable<CollectionPageSubtypes>, LinkSubtypes>>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-prev)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#prev`
    ///
    /// In a paged [Collection], indicates the prev page of items.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    prev: FunctionalProperty<Box<Or<Remotable<CollectionPageSubtypes>, LinkSubtypes>>>,
}

def_subtypes!(
    CollectionPage,
    CollectionPageSubtypes,
    [Collection, Object],
    {
        CollectionPage,
        OrderedCollectionPage
    }
);

/// ## Caution
/// W3C recommendation says IntransitiveActivity has no properties that come from Object.
/// But this definition accepts any properties that Object has.
///
/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-intransitiveactivity)
///
/// uri: `https://www.w3.org/ns/activitystreams#IntransitiveActivity`
///
/// Instances of [IntransitiveActivity] are a subtype of [Activity] representing intransitive actions.
/// The [Activity::object] property is therefore inappropriate for these activities.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Travel",
///   "summary": "Sally went to work",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "target": {
///     "type": "Place",
///     "name": "Work"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(
    IntransitiveActivity,
    IntransitiveActivitySubtypes,
    [Activity, Object],
    { IntransitiveActivity, Arrive, Question, Travel }
);

/// A [Link] is an indirect, qualified reference to a resource identified by a URL.
/// The fundamental model for links is established by [RFC5988](https://datatracker.ietf.org/doc/html/rfc5988).\
/// Many of the properties defined by the Activity Vocabulary allow values
/// that are either instances of [crate::Object] or [Link]. When a [Link] is used,
/// it establishes a qualified relation connecting the subject (the containing object) to the resource
/// identified by the [Link::href].
/// Properties of the [Link] are properties of the reference as opposed to properties of the resource.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Link",
///   "href": "http://example.org/abc",
///   "hreflang": "en",
///   "mediaType": "text/html",
///   "name": "An example link"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Link {
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-href)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#href`
    ///
    /// The target resource pointed to by a Link.
    pub href: Url,
    #[serde(skip_serializing_if = "LangContainer::is_empty")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-href)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#href`
    ///
    /// The target resource pointed to by a Link.
    pub hreflang: LangContainer<Url>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-rel)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#rel`
    ///
    /// A link relation associated with a [Link]. The value must conform to
    /// both the [HTML5](https://html.spec.whatwg.org/multipage/) and [RFC5988](https://datatracker.ietf.org/doc/html/rfc5988) "link relation" definitions.
    /// In the [HTML5](https://html.spec.whatwg.org/multipage/), any string not containing the "space" U+0020, "tab" (U+0009),
    /// "LF" (U+000A), "FF" (U+000C), "CR" (U+000D) or "," (U+002C) characters can be used as a valid link relation.
    pub rel: Property<String>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mediatype)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#mediaType`
    ///
    /// Identifies the MIME media type of the referenced resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: FunctionalProperty<String>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-context)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#name`
    ///
    /// A simple, human-readable, plain-text name for the object.
    /// HTML markup must not be included.
    /// The name may be expressed using multiple language-tagged values.
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub name: Property<String>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-context)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#name`
    ///
    /// A simple, human-readable, plain-text name for the object.
    /// HTML markup must not be included.
    /// The name may be expressed using multiple language-tagged values.
    #[serde(skip_serializing_if = "Property::is_empty", rename = "nameMap")]
    pub name_map: Property<LangContainer<String>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-height)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#height`
    ///
    /// On a [Link], specifies a hint as to the rendering height in device-independent pixels of the linked resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: FunctionalProperty<u64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-width)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#width`
    ///
    /// On a [Link], specifies a hint as to the rendering width in device-independent pixels of the linked resource.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: FunctionalProperty<u64>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-preview)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#preview`
    ///
    /// Identifies an entity that provides a preview of this object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub preview: RemotableObjectOrLinkProp,
}

impl From<url::Url> for Link {
    fn from(value: url::Url) -> Self {
        Self {
            href: value,
            rel: Default::default(),
            media_type: Default::default(),
            name: Default::default(),
            name_map: Default::default(),
            hreflang: Default::default(),
            height: Default::default(),
            width: Default::default(),
            preview: Default::default(),
        }
    }
}

def_subtypes!(Link, LinkSubtypes, [], { Link, Mention });

/// Describes an object of any kind.
/// The Object type serves as the base type for
/// most of the other kinds of objects defined in the Activity Vocabulary,
/// including other Core types such as [Activity], [IntransitiveActivity], [Collection] and [OrderedCollection].
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Object",
///   "id": "http://www.test.example/object/1",
///   "name": "A Simple, non-specific object"
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Object {
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-attachment)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#attributedTo`
    ///
    /// Identifies a resource attached or related to an object that potentially requires special handling.
    /// The intent is to provide a model that is at least semantically similar to attachments in email.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub attachment: RemotableObjectOrLinkProp,
    #[serde(
        skip_serializing_if = "RemotableObjectOrLinkProp::is_empty",
        rename = "attributedTo"
    )]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-attachment)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#attributedTo`
    ///
    /// Identifies one or more entities to which this object is attributed.
    /// The attributed entities might not be Actors. For instance, an object might be attributed to the completion of another activity.
    pub attributed_to: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audience)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#audience`
    ///
    /// Identifies one or more entities that represent the total population of entities
    /// for which the object can considered to be relevant.
    pub audience: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "Property::is_empty")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-content)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#content`
    ///
    /// The content or textual representation of the Object encoded as a JSON string.
    /// By default, the value of content is HTML.
    /// The [Object::media_type] property
    /// can be used in the object to indicate a different content type.
    /// The content may be expressed using multiple language-tagged values.
    pub content: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "contentMap")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-content)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#content`
    ///
    /// The content or textual representation of the Object encoded as a JSON string.
    /// By default, the value of content is HTML.
    /// The [Object::media_type] property
    /// can be used in the object to indicate a different content type.
    /// The content may be expressed using multiple language-tagged values.
    pub content_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-context)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#context`
    ///
    /// Identifies the context within which the object exists or an activity was performed.
    /// The notion of "context" used is intentionally vague.
    /// The intended function is to serve as a means of grouping objects
    /// and activities that share a common originating context or purpose.
    /// An example could be all activities relating to a common project or event.
    pub context: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "Property::is_empty")]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-context)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#name`
    ///
    /// A simple, human-readable, plain-text name for the object.
    /// HTML markup must not be included.
    /// The name may be expressed using multiple language-tagged values.
    pub name: Property<String>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-context)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#name`
    ///
    /// A simple, human-readable, plain-text name for the object.
    /// HTML markup must not be included.
    /// The name may be expressed using multiple language-tagged values.
    #[serde(skip_serializing_if = "Property::is_empty", rename = "nameMap")]
    pub name_map: Property<LangContainer<String>>,
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "endTime"
    )]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-endtime)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#endTime`
    ///
    /// The date and time describing the actual or expected ending time of the object.
    /// When used with an Activity object, for instance,
    /// the endTime property specifies the moment the activity concluded or is expected to conclude.
    pub end_time: FunctionalProperty<xsd::DateTime>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-generator)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#generator`
    ///
    /// Identifies the entity (e.g. an application) that generated the object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub generator: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-icon)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#icon`
    ///
    /// Indicates an entity that describes an icon for this object.
    /// The image should have an aspect ratio of one (horizontal)
    /// to one (vertical) and should be suitable for presentation at a small size.
    #[serde(skip_serializing_if = "RemotableOrLinkProp::is_empty")]
    pub icon: RemotableOrLinkProp<ImageSubtypes>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-image)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#image`
    ///
    /// Indicates an entity that describes an image for this object. Unlike the icon property,
    /// there are no aspect ratio or display size limitations assumed.
    #[serde(skip_serializing_if = "RemotableOrLinkProp::is_empty")]
    pub image: RemotableOrLinkProp<ImageSubtypes>,
    /// [W3C recommendation](hhttps://www.w3.org/TR/activitystreams-vocabulary/#dfn-inreplyto)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#inReplyTo`
    ///
    /// Indicates one or more entities for which this object is considered a response.
    #[serde(
        skip_serializing_if = "RemotableObjectOrLinkProp::is_empty",
        rename = "inReplyTo"
    )]
    pub in_reply_to: RemotableObjectOrLinkProp,
    /// [W3C recommendation](hhttps://www.w3.org/TR/activitystreams-vocabulary/#dfn-location)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#location`
    ///
    /// Indicates one or more physical or logical locations associated with the object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub location: RemotableObjectOrLinkProp,
    /// [W3C recommendation](hhttps://www.w3.org/TR/activitystreams-vocabulary/#dfn-preview)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#preview`
    ///
    /// Identifies an entity that provides a preview of this object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub preview: RemotableObjectOrLinkProp,
    /// [W3C recommendation](hhttps://www.w3.org/TR/activitystreams-vocabulary/#dfn-published)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#published`
    ///
    /// The date and time at which the object was published
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub published: FunctionalProperty<xsd::DateTime>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-replies)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#replies`
    ///
    /// Identifies a Collection containing objects considered to be responses to this object.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub replies: FunctionalProperty<Box<CollectionSubtypes>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-replies)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#startTime`
    ///
    /// The date and time describing the actual or expected starting time of the object.
    /// When used with an [Activity] object,
    /// for instance, the [Object::start_time] property specifies the moment the activity began or is scheduled to begin.
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "startTime"
    )]
    pub start_time: FunctionalProperty<xsd::DateTime>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-summary)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#summary`
    ///
    /// A natural language summarization of the object encoded as HTML.
    /// Multiple language tagged summaries **may** be provided.
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub summary: Property<String>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-summary)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#summary`
    ///
    /// A natural language summarization of the object encoded as HTML.
    /// Multiple language tagged summaries **may** be provided.
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub summary_map: Property<LangContainer<String>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-tag)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#tag`
    ///
    /// One or more "tags" that have been associated with an objects.
    /// A tag can be any kind of Object.
    /// The key difference between [Object::attachment] and [Object::tag]
    /// is that the former implies association by inclusion, while the latter implies associated by reference.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub tag: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-updated)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#updated`
    ///
    /// The date and time at which the object was updated
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: FunctionalProperty<xsd::DateTime>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-url)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#url`
    ///
    /// Identifies one or more links to representations of the object
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub url: Property<Or<LinkSubtypes, url::Url>>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-to)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#to`
    ///
    /// Identifies an entity considered to be part of the public primary audience of an Object
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub to: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-bto)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#bto`
    ///
    /// Identifies an Object that is part of the private primary audience of this Object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub bto: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-cc)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#cc`
    ///
    /// Identifies an Object that is part of the public secondary audience of this Object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub cc: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-cc)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#bcc`
    ///
    /// Identifies one or more Objects that are part of the private secondary audience of this Object.
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub bcc: RemotableObjectOrLinkProp,
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "media_type"
    )]
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-mediatype)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#mediaType`
    ///
    /// Identifies the MIME media type of the value of the [Object::content] property.
    /// If not specified, the [Object::content] property is assumed to contain text/html content.
    pub media_type: FunctionalProperty<String>,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-duration)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#duration`
    ///
    /// When the object describes a time-bound resource,
    /// such as an audio or video, a meeting, etc,
    /// the [Object::duration] property indicates the object's approximate duration.
    /// The value **must** be expressed as an xsd:duration as defined by [xmlschema11-2](https://www.w3.org/TR/xmlschema11-2/),
    /// section 3.3.6 (e.g. a period of 5 seconds is represented as "PT5S").
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub duration: FunctionalProperty<xsd::DateTime>,
}

def_subtypes!(
    Object,
    ObjectSubtypes,
    [],
    {
        Object,
        Activity,
        IntransitiveActivity,
        Collection,
        CollectionPage,
        OrderedCollection,
        OrderedCollectionPage,
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
        Update,
        Application,
        Group,
        Organization,
        Person,
        Service,
        Article,
        Audio,
        Document,
        Event,
        Image,
        Note,
        Page,
        Place,
        Profile,
        Relationship,
        Tombstone,
        Video
    }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollection)
///
/// uri: `https://www.w3.org/ns/activitystreams#OrderedCollection`
///
/// A subtype of [Collection] in which members of the logical collection are assumed to always be strictly ordered.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Sally's notes",
///   "type": "OrderedCollection",
///   "totalItems": 2,
///   "orderedItems": [
///     {
///       "type": "Note",
///       "name": "A Simple Note"
///     },
///     {
///       "type": "Note",
///       "name": "Another Simple Note"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct OrderedCollection {
    #[serde(flatten)]
    pub _super: Collection,
}

def_subtypes!(
    OrderedCollection,
    OrderedCollectionSubtypes,
    [Collection, Object],
    { OrderedCollection, OrderedCollectionPage }
);

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-orderedcollectionpage)
///
/// uri: `https://www.w3.org/ns/activitystreams#OrderedCollectionPage`
///
/// Used to represent ordered subsets of items from an [OrderedCollection].
/// Refer to the [Activity Streams 2.0 Core](https://www.w3.org/TR/activitystreams-core/#dfn-orderedcollectionpage) for a complete description of the [OrderedCollectionPage] object.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "summary": "Page 1 of Sally's notes",
///   "type": "OrderedCollectionPage",
///   "id": "http://example.org/foo?page=1",
///   "partOf": "http://example.org/foo",
///   "orderedItems": [
///     {
///       "type": "Note",
///       "name": "A Simple Note"
///     },
///     {
///       "type": "Note",
///       "name": "Another Simple Note"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct OrderedCollectionPage {
    #[serde(flatten)]
    pub _super: CollectionPage,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-startindex)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#startIndex`
    ///
    /// A non-negative integer value identifying the relative position within the logical view of a strictly ordered collection.
    #[serde(skip_serializing_if = "FunctionalProperty::is_none", rename = "partOf")]
    pub start_index: FunctionalProperty<Box<OrderedCollectionPage>>,
}

// Cannot impl `From<OrdererdCollectionPage> for OrderedCollection` beca

def_subtypes!(
    OrderedCollectionPage,
    OrderedCollectionPageSubtypes,
    [CollectionPage, Collection, Object],
    { OrderedCollectionPage }
);

impl From<OrderedCollectionPage> for OrderedCollection {
    fn from(value: OrderedCollectionPage) -> Self {
        OrderedCollection {
            _super: value._super.into(),
        }
    }
}
