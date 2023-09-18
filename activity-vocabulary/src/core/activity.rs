use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{LinkableProperty, Object, ObjectSubtypes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Activity {
    #[serde(flatten)]
    pub _super: Object,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub actor: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub object: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub target: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub result: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub origin: LinkableProperty<ObjectSubtypes>,
    #[serde(skip_serializing_if = "LinkableProperty::is_empty")]
    pub instrument: LinkableProperty<ObjectSubtypes>,
}

impl Deref for Activity {
    type Target = Object;

    fn deref(&self) -> &Self::Target {
        &self._super
    }
}

impl From<Activity> for Object {
    fn from(value: Activity) -> Self {
        value._super
    }
}
