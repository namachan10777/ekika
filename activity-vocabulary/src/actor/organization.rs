use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Object};

/// [W3C recommendation](https://www.w3.org/TR/activitystreams-vocabulary/#dfn-organization)
///
/// uri: `https://www.w3.org/ns/activitystreams#Organization`
///
/// Represents an organization.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Organization {
    #[serde(flatten)]
    pub _super: Object,
}

def_subtypes!(Organization, OrganizationSubtypes, [Object], {
    Organization
});
