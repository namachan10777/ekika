use serde::{Deserialize, Serialize};
use url::Url;

use crate::{
    def_subtypes, xsd::LangContainer, FunctionalProperty, Mention, Property,
    RemotableObjectOrLinkProp,
};

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
