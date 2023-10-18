use time::{OffsetDateTime, /*Time*/};
// use serde::{Serialize, Deserialize};

pub struct Code {
    pub language: String,
    pub code: String,
    // #[serde(with = "time::serde::iso8601")]
    pub timestamp: OffsetDateTime,
}

// #[derive(Serialize, Deserialize)]
pub struct SendingCode {
    pub author: String,
    pub code: Code,
    pub receiver: String,
    pub timestamp: OffsetDateTime,
}

impl SendingCode {
    pub fn new(author: String, code: Code, receiver: String) -> Self {
        Self {
            author,
            code,
            receiver,
            timestamp: OffsetDateTime::now_utc(),
        }
    }
}