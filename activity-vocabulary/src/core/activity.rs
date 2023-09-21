use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
#[serde(tag = "type")]
pub struct Activity {
    #[serde(flatten)]
    pub _super: Object,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub actor: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub object: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub target: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub result: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub origin: RemotableObjectOrLinkProp,
    #[serde(skip_serializing_if = "RemotableObjectOrLinkProp::is_empty")]
    pub instrument: RemotableObjectOrLinkProp,
}

def_subtypes!(
    Activity,
    ActivitySubtypes,
    [Object],
    {
        Activity,
        IntransitiveActivity,
        Accept,
        TentativeAccept,
        Add,
        Announce,
        Arrive,
        Block,
        Create,
        Delete,
        Dislike,
        Flag,
        Follow,
        Ignore,
        Invite,
        Join,
        Leave,
        Like,
        Listen,
        Move,
        Offer,
        Question,
        Read,
        Reject,
        TentativeReject,
        Remove,
        Undo,
        Travel,
        Update
    }
);
