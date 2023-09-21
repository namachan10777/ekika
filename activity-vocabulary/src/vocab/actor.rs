use super::*;
use crate::def_subtypes;
use serde::{Deserialize, Serialize};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-application)
///
/// uri: `https://www.w3.org/ns/activitystreams#Application`
///
/// Describes a software application.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Application",
///   "name": "Exampletron 3000"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Application {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Application, ApplicationSubtypes, [Object], { Application });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-group)
///
/// uri: `https://www.w3.org/ns/activitystreams#Group`
///
/// Represents a formal or informal collective of Actors.
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Group",
///   "name": "Big Beards of Austin"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Group {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Group, GroupSubtypes, [Object], { Group });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization)
///
/// uri: `https://www.w3.org/ns/activitystreams#Organization`
///
/// Represents an organization.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Organization",
///   "name": "Example Co."
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Organization {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Organization, OrganizationSubtypes, [Object], {
    Organization
});

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-person)
///
/// uri: `https://www.w3.org/ns/activitystreams#Person`
///
/// Represents an individual person.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Person",
///   "name": "Sally Smith"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Person {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Person, PersonSubtypes, [Object], { Person });

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-service)
///
/// uri: `https://www.w3.org/ns/activitystreams#Service`
///
/// Represents a service of any kind.
///
/// ```json
/// {
///   "@context": "https://www.w3.org/ns/activitystreams",
///   "type": "Service",
///   "name": "Acme Web Service"
/// }
/// ```
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Service {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Service, ServiceSubtypes, [Object], { Service });
