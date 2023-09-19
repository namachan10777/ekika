use serde::{Deserialize, Serialize};

use crate::{
    def_subtypes,
    xsd::{self, LangContainer},
    Activity, Collection, CollectionPage, CollectionSubtypes, FunctionalProperty,
    IntransitiveActivity, Link, OrderedCollection, OrderedCollectionPage, Property,
    RemotableObjectOrLinkProp, RemotableOrLinkProp,
};

///	Describes an object of any kind.
/// The Object type serves as the base type for
/// most of the other kinds of objects defined in the Activity Vocabulary,
/// including other Core types such as [crate::Activity], [crate::IntransitiveActivity], [crate::Collection] and [crate::OrderedCollection].
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
    /// Identifies one or more entities to which this object is attributed.
    /// The attributed entities might not be Actors. For instance, an object might be attributed to the completion of another activity.
    pub attributed_to: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub audience: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub content: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "contentMap")]
    pub content_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub context: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub name: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "nameMap")]
    pub name_map: Property<LangContainer<String>>,
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "endTime"
    )]
    pub end_time: FunctionalProperty<xsd::DateTime>,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub generator: RemotableObjectOrLinkProp,
    /// unimplemented
    #[serde(skip_serializing_if = "RemotableOrLinkProp::is_empty")]
    pub icon: RemotableOrLinkProp<()>,
    /// unimplemented
    #[serde(skip_serializing_if = "RemotableOrLinkProp::is_empty")]
    pub image: RemotableOrLinkProp<()>,
    #[serde(
        skip_serializing_if = "RemotableObjectOrLinkProp::is_empty",
        rename = "inReplyTo"
    )]
    pub in_reply_to: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub location: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub preview: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub published: FunctionalProperty<xsd::DateTime>,
    #[serde(skip_serializing_if = "FunctionalProperty::is_none")]
    pub replies: FunctionalProperty<Box<CollectionSubtypes>>,
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "startTime"
    )]
    pub start_time: FunctionalProperty<xsd::DateTime>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub summary: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub summary_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub tag: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: FunctionalProperty<xsd::DateTime>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub url: Property<Link>,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub to: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub bto: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub cc: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub bcc: RemotableObjectOrLinkProp,
    #[serde(
        skip_serializing_if = "FunctionalProperty::is_none",
        rename = "media_type"
    )]
    pub media_type: FunctionalProperty<String>,
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
        OrderedCollectionPage
    }
);
