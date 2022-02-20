use chrono::{DateTime, Utc};

use thiserror::Error;

pub type TimeSpecResult<T> = std::result::Result<T, TimeSpecError>;

#[derive(Error, Debug)]
pub enum TimeSpecError {
    #[error("Datetime parse error, reason {0}")]
    FailedDateTimeParse(String),
}

#[derive(Clone)]
pub enum StartFrom {
    Now,
    DateTime { date_time: String, format: String },
}

impl TryInto<DateTime<Utc>> for StartFrom {
    type Error = TimeSpecError;

    fn try_into(self) -> TimeSpecResult<DateTime<Utc>> {
        match self {
            StartFrom::Now => Ok(Utc::now()),
            StartFrom::DateTime { date_time, format } => {
                DateTime::parse_from_str(&date_time, &format)
                    .map_err(|err| TimeSpecError::FailedDateTimeParse(err.to_string()))
                    .map(|time| DateTime::<Utc>::from(time))
            }
        }
    }
}
