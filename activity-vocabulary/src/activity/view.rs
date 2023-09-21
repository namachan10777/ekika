use serde::{Deserialize, Serialize};

use crate::{Activity, def_subtypes, Object, TentativeAccept};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct View {
    #[serde(flatten)]
    pub _super: Activity,
}

def_subtypes!(View, ViewSubtypes, [Activity, Object], {View});
