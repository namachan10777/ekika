use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, Reject};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct TentativeReject {
    #[serde(flatten)]
    pub _super: Reject,
}

def_subtypes!(
    TentativeReject,
    TentativeRejectSubtypes,
    [Reject, Activity, Object],
    { TentativeReject }
);
