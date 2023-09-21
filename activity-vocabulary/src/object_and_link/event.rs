use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-event)
///
/// uri: `https://www.w3.org/ns/activitystreams#Event`
///
/// Represents any kind of event.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Event",
///   "name": "Going-Away Party for Jim",
///   "startTime": "2014-12-31T23:00:00-08:00",
///   "endTime": "2015-01-01T06:00:00-08:00"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Event {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Event, EventSubtypes, [Object], { Event });
