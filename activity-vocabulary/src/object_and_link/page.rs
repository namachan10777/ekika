use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Document, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-page)
///
/// uri: `https://www.w3.org/ns/activitystreams#Page`
///
/// Represents a Web Page.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Page",
///   "name": "Omaha Weather Report",
///   "url": "http://example.org/weather-in-omaha.html"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Page {
    #[serde(flatten)]
    pub _super: Document,
}

def_subtypes!(Page, PageSubtypes, [Document, Object], { Page });
