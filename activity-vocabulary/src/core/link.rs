use serde::{Deserialize, Serialize};
use url::Url;

use crate::{xsd::LangContainer, FunctionalProperty, Property, RemotableObjectOrLinkProp};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Link {
    href: Url,
    #[serde(skip_serializing_if = "LangContainer::is_empty")]
    hreflang: LangContainer<Url>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    rel: Property<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: FunctionalProperty<String>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    name: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "nameMap")]
    name_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: FunctionalProperty<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: FunctionalProperty<u64>,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    preview: RemotableObjectOrLinkProp,
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

impl From<Link> for Url {
    fn from(value: Link) -> Self {
        value.href
    }
}

impl AsRef<Url> for Link {
    fn as_ref(&self) -> &Url {
        &self.href
    }
}

#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum LinkSubtypes {
    Link(Link),
}

impl From<LinkSubtypes> for Link {
    fn from(value: LinkSubtypes) -> Self {
        match value {
            LinkSubtypes::Link(x) => x.into(),
        }
    }
}
