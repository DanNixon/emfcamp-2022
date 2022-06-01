use super::{venue::Venue, Event, EventKind};
use anyhow::Result;
use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime};
use serde::Serialize;
use std::fmt;
use url::Url;

pub(crate) fn load() -> Result<Vec<Event>> {
    Ok(
        reqwest::blocking::get("https://www.emfcamp.org/schedule/2022.json")?
            .json::<Vec<api::Event>>()?
            .into_iter()
            .map(|e| e.into())
            .collect(),
    )
}

impl From<api::Event> for Event {
    fn from(event: api::Event) -> Self {
        match event {
            api::Event::Talk(event) => Self {
                start: fix_shitty_timestamps(event.start_date),
                venue: event.venue.clone(),
                event: EventKind::Talk(event.into()),
            },
            api::Event::Workshop(event) => Self {
                start: fix_shitty_timestamps(event.start_date),
                venue: event.venue.clone(),
                event: EventKind::Workshop(event.into()),
            },
            api::Event::YouthWorkshop(event) => Self {
                start: fix_shitty_timestamps(event.start_date),
                venue: event.venue.clone(),
                event: EventKind::YouthWorkshop(event.into()),
            },
            api::Event::Performance(event) => Self {
                start: fix_shitty_timestamps(event.start_date),
                venue: event.venue.clone(),
                event: EventKind::Performance(event.into()),
            },
        }
    }
}

/// Fix broken by design timestamping on the emfcamp schedule API.
///
/// It's almost as if someone needs to come up with a standard way of representing
/// time on the internet to avoid issues like this. /s
/// But really, every programming language includes a library capable of handling RFC 3339 and RFC
/// 2822, what possible reason is there for formatting timestamps in anything different.
/// At least chrono makes correcting this fuckup easy.
fn fix_shitty_timestamps(shitty_timestamp: NaiveDateTime) -> DateTime<FixedOffset> {
    DateTime::from_utc(
        shitty_timestamp - Duration::hours(1),
        FixedOffset::east(60 * 60),
    )
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Talk {
    pub title: String,
    speaker: String,
    pronouns: Option<String>,
    description: String,
    link: Url,
}

impl fmt::Display for Talk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.speaker)
    }
}

impl From<api::GenericEvent> for Talk {
    fn from(event: api::GenericEvent) -> Self {
        Self {
            title: event.title,
            speaker: event.speaker,
            pronouns: event.pronouns,
            description: event.description,
            link: event.link,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Workshop {
    pub title: String,
    speaker: String,
    pronouns: Option<String>,
    description: String,
    link: Url,
}

impl fmt::Display for Workshop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.speaker)
    }
}

impl From<api::GenericEvent> for Workshop {
    fn from(event: api::GenericEvent) -> Self {
        Self {
            title: event.title,
            speaker: event.speaker,
            pronouns: event.pronouns,
            description: event.description,
            link: event.link,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) struct YouthWorkshop {
    pub title: String,
    speaker: String,
    pronouns: Option<String>,
    description: String,
    link: Url,
}

impl fmt::Display for YouthWorkshop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.speaker)
    }
}

impl From<api::GenericEvent> for YouthWorkshop {
    fn from(event: api::GenericEvent) -> Self {
        Self {
            title: event.title,
            speaker: event.speaker,
            pronouns: event.pronouns,
            description: event.description,
            link: event.link,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Performance {
    pub title: String,
    speaker: String,
    pronouns: Option<String>,
    description: String,
    link: Url,
}

impl fmt::Display for Performance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} by {}", self.title, self.speaker)
    }
}

impl From<api::GenericEvent> for Performance {
    fn from(event: api::GenericEvent) -> Self {
        Self {
            title: event.title,
            speaker: event.speaker,
            pronouns: event.pronouns,
            description: event.description,
            link: event.link,
        }
    }
}

mod api {
    use super::Venue;
    use chrono::NaiveDateTime;
    use serde::Deserialize;
    use url::Url;

    #[derive(Debug, Deserialize)]
    #[serde(tag = "type", rename_all = "lowercase")]
    pub(super) enum Event {
        Talk(GenericEvent),
        Workshop(GenericEvent),
        YouthWorkshop(GenericEvent),
        Performance(GenericEvent),
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct GenericEvent {
        #[serde(with = "parse_datetime")]
        pub start_date: NaiveDateTime,
        pub venue: Venue,
        pub title: String,
        pub speaker: String,
        pub pronouns: Option<String>,
        pub description: String,
        pub link: Url,
    }

    mod parse_datetime {
        use chrono::NaiveDateTime;
        use serde::{Deserialize, Deserializer};

        pub(crate) fn deserialize<'de, D>(
            deserializer: D,
        ) -> std::result::Result<NaiveDateTime, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            match NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S") {
                Ok(t) => Ok(t),
                Err(e) => Err(serde::de::Error::custom(e)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_load() {
        let events = load().unwrap();
        assert!(events.len() > 0);
    }
}
