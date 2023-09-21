use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-video)
///
/// uri: `https://www.w3.org/ns/activitystreams#Video`
///
/// Represents a video document of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Video",
///   "name": "Puppy Plays With Ball",
///   "url": "http://example.org/video.mkv",
///   "duration": "PT2H"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Video {
    #[serde(flatten)]
    _super: Document,
}

def_subtypes!(Video, VideoSubtypes, [Document, Object], { Video });
