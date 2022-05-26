use super::mqtt_actor::Message;
use crate::schedule::{Event, Schedule};
use chrono::Duration;
use itertools::Itertools;

pub(crate) fn generate(schedule: &Schedule) -> Vec<Message> {
    let delay = Duration::minutes(10);

    let mut grouped = Vec::new();
    for (k, g) in &schedule.events.iter().group_by(|e| e.start) {
        grouped.push((k, g.collect::<Vec<&Event>>()));
    }

    grouped
        .iter()
        .map(|(k, g)| Message {
            timestamp: *k - delay,
            topic: "emfcamp/timely_schedule".to_string(),
            text: serde_json::to_string(g).unwrap(),
        })
        .collect()
}
