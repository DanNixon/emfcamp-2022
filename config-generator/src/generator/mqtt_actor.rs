use chrono::{DateTime, FixedOffset};
use std::fmt;

#[derive(Debug)]
pub(crate) struct Message {
    pub timestamp: DateTime<FixedOffset>,
    pub topic: String,
    pub text: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} | {} | {}",
            self.timestamp.to_rfc2822(),
            self.topic,
            self.text.replace('|', "/") // | is used as a delimiter, but some people have used it in their talk submission
        )
    }
}
