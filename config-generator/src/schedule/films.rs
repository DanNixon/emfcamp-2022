use super::{venue::Venue, Event, EventKind};
use anyhow::Result;
use serde::Serialize;
use std::fmt;
use url::Url;

pub(crate) fn load() -> Result<Vec<Event>> {
    Ok(
        reqwest::blocking::get("https://emffilms.org/api/2022/schedule")?
            .json::<api::Films>()?
            .films
            .into_iter()
            .map(|f| f.into())
            .collect(),
    )
}

impl From<api::Film> for Event {
    fn from(film: api::Film) -> Self {
        Self {
            start: film.showing.timestamp,
            venue: Venue::Film,
            event: EventKind::Film(film.into()),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize)]
pub(crate) struct Film {
    pub title: String,
    pub certificate: String,
    pub runtime: String,
    pub description: String,
    pub short_description: String,
    pub imdb_url: Url,
}

impl fmt::Display for Film {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({}, {})", self.title, self.certificate, self.runtime)
    }
}

impl From<api::Film> for Film {
    fn from(film: api::Film) -> Self {
        Self {
            title: film.title,
            certificate: film.certificate,
            runtime: film.runtime.text,
            description: film.precis.full,
            short_description: film.precis.oneline,
            imdb_url: film.imdb,
        }
    }
}

mod api {
    use chrono::{DateTime, FixedOffset};
    use serde::Deserialize;
    use url::Url;

    #[derive(Debug, Deserialize)]
    pub(super) struct Films {
        pub films: Vec<Film>,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Film {
        pub title: String,
        pub certificate: String,
        pub precis: Precis,
        pub imdb: Url,
        pub showing: Showing,
        #[serde(rename = "runTime")]
        pub runtime: Runtime,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Precis {
        pub full: String,
        #[serde(rename = "oneLine")]
        pub oneline: String,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Showing {
        pub timestamp: DateTime<FixedOffset>,
    }

    #[derive(Debug, Deserialize)]
    pub(super) struct Runtime {
        pub text: String,
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
