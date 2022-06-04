use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub(crate) enum Venue {
    #[serde(rename = "Stage A")]
    StageA,
    #[serde(rename = "Stage B")]
    StageB,
    #[serde(rename = "Stage C")]
    StageC,
    #[serde(rename = "Workshop 1")]
    Workshop1,
    #[serde(rename = "Workshop 2")]
    Workshop2,
    #[serde(rename = "Workshop 3")]
    Workshop3,
    #[serde(rename = "Workshop 4")]
    Workshop4,
    #[serde(rename = "Workshop 5")]
    Workshop5,
    #[serde(rename = "Youth Workshop")]
    YouthWorkshop,
    Lounge,
    #[serde(rename = "AMSAT-UK")]
    AmsatUk,
    Blacksmiths,
    #[serde(rename = "Main Bar")]
    MainBar,
    #[serde(rename = "The Bomb")]
    TheBomb,
    #[serde(rename = "Outside the bar")]
    OutsideTheBar,
    #[serde(rename = "Family Lounge")]
    FamilyLounge,
    #[serde(rename = "Hackspace Foundation")]
    HackspaceFoundation,
    #[serde(rename = "Null Sector")]
    NullSector,
    #[serde(rename = "Null Sector SEM")]
    NullSectorSem,
    #[serde(rename = "Badge Tent")]
    BadgeTent,
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
                Self::Workshop1 => "Workshop 1",
                Self::Workshop2 => "Workshop 2",
                Self::Workshop3 => "Workshop 3",
                Self::Workshop4 => "Workshop 4",
                Self::Workshop5 => "Workshop 5",
                Self::YouthWorkshop => "Youth Workshop",
                Self::Lounge => "Lounge",
                Self::AmsatUk => "AMSAT-UK",
                Self::Blacksmiths => "Blacksmithing",
                Self::MainBar => "Main Bar",
                Self::TheBomb => "The Bomb",
                Self::OutsideTheBar => "Outside the Bar",
                Self::FamilyLounge => "Family Lounge",
                Self::HackspaceFoundation => "Hackspace Foundation",
                Self::NullSector => "Null Sector",
                Self::NullSectorSem => "Null Sector SEM",
                Self::BadgeTent => "Badge Tent",
            }
        )
    }
}
