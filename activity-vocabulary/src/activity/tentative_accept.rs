use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Accept, Activity, Object};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TentativeAccept {
    #[serde(flatten)]
    pub _super: Accept,
}

def_subtypes!(
    TentativeAccept,
    IntransitiveAcceptSubtypes,
    [Accept, Activity, Object],
    { TentativeAccept }
);
