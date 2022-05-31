mod films;
mod main_events;
pub(crate) mod venue;

use self::{
    films::Film,
    main_events::{Performance, Talk, Workshop, YouthWorkshop},
    venue::Venue,
};
use anyhow::Result;
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use std::{cmp::Ordering, fmt};

#[derive(Debug)]
pub(crate) struct Schedule {
    pub(crate) events: Vec<Event>,
}

impl Schedule {
    pub(crate) fn load() -> Result<Self> {
        let mut events = main_events::load()?;
        events.append(&mut films::load()?);

        events.sort();
        Ok(Self { events })
    }
}

#[derive(Debug, Eq, Serialize)]
pub(crate) struct Event {
    pub start: DateTime<FixedOffset>,
    pub venue: Venue,
    pub event: EventKind,
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.event {
            EventKind::Talk(t) => write!(f, "Talk @ {} in {}: {}", self.start, self.venue, t),
            EventKind::Workshop(t) => {
                write!(f, "Workshop @ {} in {}: {}", self.start, self.venue, t)
            }
            EventKind::YouthWorkshop(t) => write!(
                f,
                "Youth Workshop @ {} in {}: {}",
                self.start, self.venue, t
            ),
            EventKind::Performance(t) => {
                write!(f, "Performance @ {} in {}: {}", self.start, self.venue, t)
            }
            EventKind::Film(t) => write!(f, "Film @ {} in {}: {}", self.start, self.venue, t),
        }
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.start.cmp(&other.start)
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub(crate) enum EventKind {
    Talk(Talk),
    Workshop(Workshop),
    YouthWorkshop(YouthWorkshop),
    Performance(Performance),
    Film(Film),
}
