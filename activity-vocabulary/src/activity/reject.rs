use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, TentativeReject};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Reject {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Reject, RejectSubtypes, [Activity, Object], {Reject, TentativeReject});
