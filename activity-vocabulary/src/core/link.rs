use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    def_subtypes, xsd::LangContainer, FunctionalProperty, Mention, Property,
    RemotableObjectOrLinkProp,
};

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

def_subtypes!(Link, LinkSubtypes, [], { Link, Mention });
