use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-audio)
///
/// uri: `https://www.w3.org/ns/activitystreams#Audio`
///
/// Represents an audio document of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Audio",
///   "name": "Interview With A Famous Technologist",
///   "url": {
///     "type": "Link",
///     "href": "http://example.org/podcast.mp3",
///     "mediaType": "audio/mp3"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Audio {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Audio, AudioSubtypes, [Document, Object], { Audio });
