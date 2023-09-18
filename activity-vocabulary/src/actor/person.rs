use std::ops::Deref;

use serde::{Deserialize, Serialize};

use crate::{Object, ObjectSubtypes};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Person {
    #[serde(flatten)]
    pub _super: Object,
}

impl From<Person> for Object {
    fn from(value: Person) -> Self {
        value._super
    }
}

impl From<Person> for ObjectSubtypes {
    fn from(value: Person) -> Self {
        ObjectSubtypes::Person(value)
    }
}

impl Deref for Person {
    type Target = Object;
    fn deref(&self) -> &Self::Target {
        &self._super
    }
}
