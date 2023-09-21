use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Audio, Image, Object, Page, Video};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document)
///
/// uri: `https://www.w3.org/TR/activitystreams-vocabulary/#dfn-document`
///
/// Represents a document of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Document",
///   "name": "4Q Sales Forecast",
///   "url": "http://example.org/4q-sales-forecast.pdf"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Document {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(
    Document,
    DocumentSubtypes,
    [Object],
    {
        Document,
        Audio,
        Image,
        Video,
        Page
    }
);
