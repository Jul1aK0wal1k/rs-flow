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

#[cfg(test)]
mod tests {
    use chrono::{DateTime, Utc};
    use rstest::rstest;

    use crate::StartFrom;

    #[rstest]
    #[case("1983 Apr 13 12:09:14.274 +0000", "%Y %b %d %H:%M:%S%.3f %z")]
    #[case("1983.04.13T12:09:14+09", "%Y.%m.%dT%H:%M:%S%#z")]
    #[case("1983.04.13T12:09:14-09", "%Y.%m.%dT%H:%M:%S%#z")]
    fn parse_utc_time(#[case] date: &str, #[case] format: &str) {
        let expected = DateTime::parse_from_str(date, format).unwrap();

        let parsed: DateTime<Utc> = (StartFrom::DateTime {
            date_time: date.to_string(),
            format: format.to_string(),
        })
        .try_into()
        .unwrap();

        assert_eq!(expected, parsed);
    }
}
