use serde::{Deserialize, Serialize};

use crate::{
    xsd::{self, LangContainer},
    Collection, Image, Link, LinkableProperty, Person, Property,
};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug, Default)]
pub struct Object {
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub attachment: LinkableProperty<ObjectSubtypes>,
    #[serde(
        skip_serializing_if = "LinkableProperty::is_empty",
        rename = "attributedTo"
    )]
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
