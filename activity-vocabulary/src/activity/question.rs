use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, IntransitiveActivity, Object, RemotableObjectOrLinkProp};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Question {
    #[serde(flatten)]
    pub _super: IntransitiveActivity,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub any_of: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub one_of: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub closed: RemotableObjectOrLinkProp,
}

def_subtypes!(
    Question,
    QuestionSubtypes,
    [IntransitiveActivity, Activity, Object],
    { Question }
);
