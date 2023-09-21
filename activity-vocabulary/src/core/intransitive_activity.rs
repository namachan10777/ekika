use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Arrive, Object, Question, Travel};

/// ## Caution
/// W3C recommendation says IntransitiveActivity has no properties that come from Object.
/// But this definition accepts any properties that Object has.
///
/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-intransitiveactivity)
///
/// uri: `https://www.w3.org/ns/activitystreams#IntransitiveActivity`
///
/// Instances of [IntransitiveActivity] are a subtype of [Activity] representing intransitive actions.
/// The [Activity::object] property is therefore inappropriate for these activities.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Travel",
///   "summary": "Sally went to work",
///   "actor": {
///     "type": "Person",
///     "name": "Sally"
///   },
///   "target": {
///     "type": "Place",
///     "name": "Work"
///   }
/// }
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct IntransitiveActivity {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(
    IntransitiveActivity,
    IntransitiveActivitySubtypes,
    [Activity, Object],
    { IntransitiveActivity, Arrive, Question, Travel }
);
