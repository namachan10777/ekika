use serde::{Deserialize, Serialize};

use crate::{xsd::LangContainer, *};

///	Describes an object of any kind.
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
