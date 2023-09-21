use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, IntransitiveActivity, Object, RemotableObjectOrLinkProp};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-question)
///
/// uri: `https://www.w3.org/ns/activitystreams#Question`
///
/// Represents a question being asked.
/// Question objects are an extension of [IntransitiveActivity].
/// That is, the Question object is an [Activity],
/// but the direct object is the question itself and therefore it would not contain an [Activity::object] property.
///
/// Either of the [Question::any_of] and [Question::one_of] properties may be used to express possible answers,
/// but a Question object **must not** have both properties.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Question",
///   "name": "What is the answer?",
///   "oneOf": [
///     {
///       "type": "Note",
///       "name": "Option A"
///     },
///     {
///       "type": "Note",
///       "name": "Option B"
///     }
///   ]
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Question {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-anyof)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#anyOf`
    ///
    /// Identifies an inclusive option for a [Question].
    /// Use of [Question::any_of] implies that the [Question] can have multiple answers.
    /// To indicate that a [Question] can have only one answer, use [Question::one_of].
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub any_of: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-oneof)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#oneOf`
    ///
    /// Identifies an exclusive option for a [Question].
    /// Use of [Question::one_of] implies that the [Question] can have only a single answer.
    /// To indicate that a [Question] can have multiple answers, use [Question::any_of].
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub one_of: RemotableObjectOrLinkProp,
    /// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-closed)
    ///
    /// uri: `https://www.w3.org/ns/activitystreams#closed`
    ///
    /// Describes an indirect object of the activity from which the activity is directed.
    /// The precise meaning of the origin is the object of the English preposition "from".
    /// For instance, in the activity "John moved an item to List B from List A", the origin of the activity is "List A".
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub closed: RemotableObjectOrLinkProp,
}

def_subtypes!(
    Question,
    QuestionSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Question }
);
