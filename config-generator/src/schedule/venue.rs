use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum Venue {
    #[serde(rename = "Stage A")]
    StageA,
    #[serde(rename = "Stage B")]
    StageB,
    #[serde(rename = "Stage C")]
    StageC,
    Film,
}

impl fmt::Display for Venue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::StageA => "Stage A",
                Self::StageB => "Stage B",
                Self::StageC => "Stage C",
                Self::Film => "emffilms",
            }
        )
    }
}
