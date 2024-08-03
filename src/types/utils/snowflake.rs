// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

use chrono::{DateTime, TimeZone, Utc};

/// 2015-01-01
const EPOCH: i64 = 1420070400000;

/// Unique identifier including a timestamp.
///
/// # Reference
/// See <https://discord.com/developers/docs/reference#snowflakes>
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Snowflake(pub u64);

impl Snowflake {
    /// Generates a snowflake for the current timestamp, with worker id 0 and process id 1.
    pub fn generate() -> Self {
        const WORKER_ID: u64 = 0;
        const PROCESS_ID: u64 = 1;
        static INCREMENT: AtomicUsize = AtomicUsize::new(0);

        let time = (Utc::now().naive_utc().timestamp_millis() - EPOCH) << 22;
        let worker = WORKER_ID << 17;
        let process = PROCESS_ID << 12;
        let increment = INCREMENT.fetch_add(1, Ordering::Relaxed) as u64 % 32;

        Self(time as u64 | worker | process | increment)
    }

    /// Returns the snowflake's timestamp
    pub fn timestamp(self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt((self.0 >> 22) as i64 + EPOCH)
            .unwrap()
    }
}

impl Default for Snowflake {
    fn default() -> Self {
        Self::generate()
    }
}

impl Display for Snowflake {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T> From<T> for Snowflake
where
    T: Into<u64>,
{
    fn from(item: T) -> Self {
        Self(item.into())
    }
}

impl serde::Serialize for Snowflake {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0.to_string())
    }
}

impl<'de> serde::Deserialize<'de> for Snowflake {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SnowflakeVisitor;
        impl<'de> serde::de::Visitor<'de> for SnowflakeVisitor {
            type Value = Snowflake;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("snowflake string")
            }

            fn visit_str<E>(self, value: &str) -> Result<Snowflake, E>
            where
                E: serde::de::Error,
            {
                match value.parse() {
                    Ok(value) => Ok(Snowflake(value)),
                    Err(_) => Err(serde::de::Error::custom("")),
                }
            }
        }
        deserializer.deserialize_str(SnowflakeVisitor)
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Any> for Snowflake {
    fn type_info() -> <sqlx::Any as sqlx::Database>::TypeInfo {
        <String as sqlx::Type<sqlx::Any>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Any> for Snowflake {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Any as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <String as sqlx::Encode<'q, sqlx::Any>>::encode_by_ref(&self.0.to_string(), buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'d> sqlx::Decode<'d, sqlx::Any> for Snowflake {
    fn decode(
        value: <sqlx::Any as sqlx::Database>::ValueRef<'d>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <String as sqlx::Decode<'d, sqlx::Any>>::decode(value)
            .map(|s| s.parse::<u64>().map(Snowflake).unwrap())
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use super::Snowflake;

    #[test]
    fn generate() {
        let snow_1 = Snowflake::generate();
        let snow_2 = Snowflake::generate();
        assert!(snow_1.0 < snow_2.0)
    }

    #[test]
    fn timestamp() {
        let snow: Snowflake = serde_json::from_str("\"175928847299117063\"").unwrap();
        let timestamp = "2016-04-30 11:18:25.796Z".parse::<DateTime<Utc>>().unwrap();
        assert_eq!(snow.timestamp(), timestamp);
    }
}
