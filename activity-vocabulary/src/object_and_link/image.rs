use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

/// [W3C recommendation]
///
/// uri: `https://www.w3.org/ns/activitystreams#Image`
///
/// An image document of any kind
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Image",
///   "name": "Cat Jumping on Wagon",
///   "url": [
///     {
///       "type": "Link",
///       "href": "http://example.org/image.jpeg",
///       "mediaType": "image/jpeg"
///     },
///     {
///       "type": "Link",
///       "href": "http://example.org/image.png",
///       "mediaType": "image/png"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Image {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Image, ImageSubtypes, [Document, Object], { Image });
