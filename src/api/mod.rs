use chrono::{DateTime, NaiveDateTime, ParseError, TimeZone, Utc};

pub mod documents;

/// Format with which datetimes are formatted in the database.
pub const DATABASE_DATETIME_FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

pub fn from_local_to_utc(datetime: &NaiveDateTime) -> DateTime<Utc> {
    chrono_tz::Europe::Copenhagen
        .from_local_datetime(datetime)
        .unwrap()
        .with_timezone(&Utc)
}

fn parse_from_local_to_utc(datetime: &str) -> Result<DateTime<Utc>, ParseError> {
    Ok(from_local_to_utc(&NaiveDateTime::parse_from_str(
        datetime,
        DATABASE_DATETIME_FORMAT,
    )?))
}

#[cfg(test)]
mod tests {
    use super::parse_from_local_to_utc;
    use chrono::{TimeZone, Utc};

    #[test]
    fn test_time_conversion() {
        assert_eq!(
            parse_from_local_to_utc("2021-11-19T16:45:35.537Z").unwrap(),
            Utc.ymd(2021, 11, 19).and_hms_milli(15, 45, 35, 537)
        );
    }
}
