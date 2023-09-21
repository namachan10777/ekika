use serde::{Deserialize, Serialize};

use crate::{def_subtypes, Link};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Mention {
    #[serde(flatten)]
    _super: Link,
}

def_subtypes!(Mention, MentionSubtypes, [Link], { Mention });
