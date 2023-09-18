use serde::{Deserialize, Serialize};
use url::Url;

use crate::{xsd::LangContainer, Linkable, LinkableProperty, ObjectSubtypes, Property};

/// Linkは直接のURIとしてもパース出来る。Recommendationには直接的にはそのようなことは一言も書かれていないが、Exampleを見るに仕様だと思われる
#[derive(PartialEq, Clone, Debug)]
pub struct Link {
    /// 常識的に考えて、流石にこれは存在すると仮定してる
    href: Url,
    hreflang: LangContainer<Url>,
    rel: Property<String>,
    media_type: Option<String>,
    name: Property<String>,
    name_map: Property<LangContainer<String>>,
    height: Option<u64>,
    width: Option<u64>,
    preview: Property<Linkable<ObjectSubtypes>>,
}

impl Link {
    pub fn as_href(&self) -> Option<&url::Url> {
        if self.rel.is_empty()
            && self.media_type.is_none()
            && self.name.is_empty()
            && self.hreflang.is_empty()
            && self.height.is_none()
            && self.width.is_none()
            && self.preview.is_empty()
        {
            Some(&self.href)
        } else {
            None
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LinkSimpleSerde {
    /// 常識的に考えて、これは流石に存在すると考えて良さそう
    href: Url,
    #[serde(skip_serializing_if = "LangContainer::is_empty")]
    hreflang: LangContainer<Url>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    rel: Property<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    media_type: Option<String>,
    #[serde(skip_serializing_if = "Property::is_empty")]
    name: Property<String>,
    #[serde(skip_serializing_if = "Property::is_empty", rename = "nameMap")]
    name_map: Property<LangContainer<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    height: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<u64>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    preview: LinkableProperty<ObjectSubtypes>,
}

impl From<LinkSimpleSerde> for Link {
    fn from(value: LinkSimpleSerde) -> Self {
        Self {
            href: value.href,
            rel: value.rel,
            media_type: value.media_type,
            name: value.name,
            name_map: value.name_map,
            hreflang: value.hreflang,
            height: value.height,
            width: value.width,
            preview: value.preview,
        }
    }
}

impl From<Link> for LinkSimpleSerde {
    fn from(value: Link) -> Self {
        Self {
            href: value.href,
            rel: value.rel,
            media_type: value.media_type,
            name: value.name,
            name_map: value.name_map,
            hreflang: value.hreflang,
            height: value.height,
            width: value.width,
            preview: value.preview,
        }
    }
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

impl<'de> Deserialize<'de> for Link {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let content = serde::__private::de::Content::deserialize(deserializer)?;
        let deserializer = serde::__private::de::ContentRefDeserializer::<D::Error>::new(&content);
        match LinkSimpleSerde::deserialize(deserializer) {
            Ok(just) => Ok(just.into()),
            Err(just_err) => Url::deserialize(deserializer)
                .map_err(|vec_err| serde::de::Error::custom(format!("{vec_err} and {just_err}")))
                .map(Into::into),
        }
    }
}

impl Serialize for Link {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if let Some(url) = self.as_href() {
            url.serialize(serializer)
        } else {
            Into::<LinkSimpleSerde>::into(self.clone()).serialize(serializer)
        }
    }
}
