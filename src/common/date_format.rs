use serde::{self, Deserialize, Deserializer, Serializer};
use sqlx::types::chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub const FORMAT: &str = "%Y年%m月%d日 %H:%M";

pub const FORMAT_DATE: &str = "%Y-%m-%d";

pub fn serialize<S>(date: &DateTime<Local>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = date.format(FORMAT).to_string();
    serializer.serialize_str(&formatted)
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Local>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Local
        .datetime_from_str(&s, FORMAT)
        .map_err(serde::de::Error::custom)
}

pub fn time_stamp_to_date(time_stamp: i64) -> DateTime<Local> {
    let time = NaiveDateTime::from_timestamp_millis(time_stamp).unwrap();
    return Local.from_utc_datetime(&time);
}
