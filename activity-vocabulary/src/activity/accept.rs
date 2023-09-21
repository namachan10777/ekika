use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Activity, Object, TentativeAccept};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct Accept {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(Accept, AcceptSubtypes, [Activity, Object], {Accept, TentativeAccept});
