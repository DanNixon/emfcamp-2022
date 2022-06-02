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
        Venue::Workshop1 => "dapnet/emfcamp/workshop",
        Venue::Workshop2 => "dapnet/emfcamp/workshop",
        Venue::Workshop3 => "dapnet/emfcamp/workshop",
        Venue::Workshop4 => "dapnet/emfcamp/workshop",
        Venue::Workshop5 => "dapnet/emfcamp/workshop",
        Venue::Lounge => "dapnet/emfcamp/workshop",
        Venue::AmsatUk => "dapnet/emfcamp/workshop",
        Venue::YouthWorkshop => "dapnet/emfcamp/youth_workshop",
        Venue::Blacksmiths => "dapnet/emfcamp/workshop",
    }
    .to_string()
}

fn venue_short_name(venue: &Venue) -> String {
    match venue {
        Venue::StageA => "Stg A",
        Venue::StageB => "Stg B",
        Venue::StageC => "Stg C",
        Venue::Workshop1 => "Ws 1",
        Venue::Workshop2 => "Ws 2",
        Venue::Workshop3 => "Ws 3",
        Venue::Workshop4 => "Ws 4",
        Venue::Workshop5 => "Ws 5",
        Venue::Lounge => "Lounge",
        Venue::AmsatUk => "AMSAT-UK",
        Venue::YouthWorkshop => "Youth Ws",
        Venue::Blacksmiths => "Blksmth",
    }
    .to_string()
}

fn event_to_message(event: &Event) -> String {
    let mut msg = match &event.event {
        EventKind::Talk(talk) => {
            format!("{}: {}", venue_short_name(&event.venue), talk.title)
        }
        EventKind::Workshop(workshop) => {
            format!("{}: {}", venue_short_name(&event.venue), workshop.title)
        }
        EventKind::YouthWorkshop(workshop) => {
            format!("{}: {}", venue_short_name(&event.venue), workshop.title)
        }
        EventKind::Performance(performance) => {
            format!("{}: {}", venue_short_name(&event.venue), performance.title)
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
