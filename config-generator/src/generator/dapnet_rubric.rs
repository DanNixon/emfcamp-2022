use super::mqtt_actor::Message;
use crate::schedule::{venue::Venue, Event, EventKind, Schedule};
use chrono::Duration;

pub(crate) fn generate(schedule: &Schedule) -> Vec<Message> {
    let delay = Duration::minutes(5);

    schedule
        .events
        .iter()
        .map(|e| Message {
            timestamp: e.start - delay,
            topic: venue_to_topic(&e.venue),
            text: event_to_message(e),
        })
        .collect()
}

fn venue_to_topic(venue: &Venue) -> String {
    match venue {
        Venue::StageA => "dapnet/emfcamp/stage_a",
        Venue::StageB => "dapnet/emfcamp/stage_b",
        Venue::StageC => "dapnet/emfcamp/stage_c",
        Venue::Film => "dapnet/emfcamp/film",
    }
    .to_string()
}

fn venue_short_name(venue: &Venue) -> String {
    match venue {
        Venue::StageA => "Stg A",
        Venue::StageB => "Stg B",
        Venue::StageC => "Stg C",
        Venue::Film => "Flm",
    }
    .to_string()
}

fn event_to_message(event: &Event) -> String {
    let mut msg = match &event.event {
        EventKind::Talk(talk) => {
            format!("{}: {}", venue_short_name(&event.venue), talk.title)
        }
        EventKind::Film(film) => {
            format!(
                "Film in 10 mins: {} ({}, {})",
                film.title, film.certificate, film.runtime
            )
        }
    };

    let char_limit = 80;
    msg = match msg.char_indices().nth(char_limit) {
        None => msg,
        Some((_, _)) => {
            let (i, _) = msg.char_indices().nth(char_limit - 3).unwrap();
            let s: String = msg[..i].into();
            format!("{}...", s)
        }
    };

    msg
}
