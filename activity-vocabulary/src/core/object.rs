use serde::{Deserialize, Serialize};

use crate::{
    xsd::{self, LangContainer},
    Collection, Image, Link, LinkableProperty, Person, Property,
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
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub attachment: LinkableProperty<ObjectSubtypes>,
    #[serde(
        skip_serializing_if = "LinkableProperty::is_empty",
        rename = "attributedTo"
    )]
    /// Identifies one or more entities to which this object is attributed.
    /// The attributed entities might not be Actors. For instance, an object might be attributed to the completion of another activity.
    pub attributed_to: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub audience: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub content: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "contentMap")]
    pub content_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub context: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub name: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "nameMap")]
    pub name_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "endTime")]
    pub end_time: Option<xsd::DateTime>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub generator: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub icon: LinkableProperty<Image>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub image: LinkableProperty<Image>,
    #[serde(
        skip_serializing_if = "LinkableProperty::is_empty",
        rename = "inReplyTo"
    )]
    pub in_reply_to: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub location: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub preview: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub published: Option<xsd::DateTime>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replies: Option<Collection>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "startTime")]
    pub start_time: Option<xsd::DateTime>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub summary: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub summary_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub tag: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated: Option<xsd::DateTime>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    pub url: Property<Link>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub to: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub bto: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub cc: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub bcc: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "media_type")]
    pub media_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<xsd::DateTime>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(tag = "type")]
pub enum ObjectSubtypes {
    Object(Object),
    Person(Person),
}

impl From<ObjectSubtypes> for Object {
    fn from(value: ObjectSubtypes) -> Self {
        match value {
            ObjectSubtypes::Object(x) => x.into(),
            ObjectSubtypes::Person(x) => x.into(),
        }
    }
}
