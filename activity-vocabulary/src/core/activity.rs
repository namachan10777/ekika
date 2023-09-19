use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{Object, RemotableObjectOrLinkProp};

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

impl Deref for Activity {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self._super
    }
}

impl AsMut<Object> for Activity {
    fn as_mut(&mut self) -> &mut Object {
        &mut self._super
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum ActivitySubtypes {
    Activity(Activity),
}

impl From<Activity> for Object {
    fn from(value: Activity) -> Self {
        value._super
    }
}

impl From<ActivitySubtypes> for Activity {
    fn from(value: ActivitySubtypes) -> Self {
        match value {
            ActivitySubtypes::Activity(x) => x.into(),
        }
    }
}

impl From<ActivitySubtypes> for Object {
    fn from(value: ActivitySubtypes) -> Self {
        Into::<Activity>::into(value).into()
    }
}
