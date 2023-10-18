pub struct Code {
    pub language: String,
    pub code: string,
    pub time_created: chrono::NaiveDateTime,
}

pub struct SendingCode {
    pub author: String,
    pub code: Code,
    pub receiver: String,
    pub timestamp_of_sending_code: chrono::NaiveDateTime,
}

