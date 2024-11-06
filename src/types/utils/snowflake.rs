// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{
    fmt::Display,
    sync::atomic::{AtomicUsize, Ordering},
};

use serde::{Serialize, Deserialize};

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

        let time = (Utc::now().naive_utc().and_utc().timestamp_millis() - EPOCH) << 22;
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

impl From<u64> for Snowflake {
    fn from(item: u64) -> Self {
        Self(item)
    }
}

impl From<Snowflake> for u64 {
    fn from(item: Snowflake) -> Self {
        item.0
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
impl sqlx::Type<sqlx::Postgres> for Snowflake {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU64 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl sqlx::postgres::PgHasArrayType for Snowflake {
    fn array_type_info() -> sqlx::postgres::PgTypeInfo {
        <Vec<sqlx_pg_uint::PgU64> as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for Snowflake {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        <sqlx_pg_uint::PgU64 as sqlx::Encode<'q, sqlx::Postgres>>::encode_by_ref(
            &sqlx_pg_uint::PgU64::from(self.0),
            buf,
        )
    }
}

#[cfg(feature = "sqlx")]
impl<'d> sqlx::Decode<'d, sqlx::Postgres> for Snowflake {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'d>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        <sqlx_pg_uint::PgU64 as sqlx::Decode<'d, sqlx::Postgres>>::decode(value)
            .map(|s| s.to_uint().into())
    }
}

/// A type representing either a single [Snowflake] or a [Vec] of [Snowflake]s.
///
/// Useful for e.g. [RequestGuildMembers](crate::types::events::GatewayRequestGuildMembers), to
/// select either one specific user or multiple users.
///
/// Should (de)serialize either as a single [Snowflake] or as an array.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[serde(untagged)]
pub enum OneOrMoreSnowflakes {
	One(Snowflake),
	More(Vec<Snowflake>)
}

impl Display for OneOrMoreSnowflakes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		  match self {
			OneOrMoreSnowflakes::One(snowflake) => write!(f, "{}", snowflake.0),
			// Display as you would debug a vec of u64s
			OneOrMoreSnowflakes::More(snowflake_vec) => write!(f, "{:?}", snowflake_vec.iter().map(|x| x.0)),
		  }
    }
}

impl From<Snowflake> for OneOrMoreSnowflakes {
    fn from(item: Snowflake) -> Self {
        Self::One(item)
    }
}

impl From<Vec<Snowflake>> for OneOrMoreSnowflakes {
    fn from(item: Vec<Snowflake>) -> Self {
		  if item.len() == 1 {
			 return Self::One(item[0]);
		  }

        Self::More(item)
    }
}

impl From<u64> for OneOrMoreSnowflakes {
    fn from(item: u64) -> Self {
        Self::One(item.into())
    }
}

impl From<Vec<u64>> for OneOrMoreSnowflakes {
    fn from(item: Vec<u64>) -> Self {
		  if item.len() == 1 {
			 return Self::One(item[0].into());
		  }

        Self::More(item.into_iter().map(|x| x.into()).collect())
    }
}

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};

    use crate::types::utils::snowflake::OneOrMoreSnowflakes;

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

	 #[test]
	 fn serialize() {
		  let snowflake = Snowflake(1303390110099968072_u64);
		  let serialized = serde_json::to_string(&snowflake).unwrap();

		  assert_eq!(serialized, "\"1303390110099968072\"".to_string());
	 }

	 #[test]
	 fn serialize_one_or_more() {
		  let snowflake = Snowflake(1303390110099968072_u64);
		  let one_snowflake: OneOrMoreSnowflakes = snowflake.into();

		  let serialized = serde_json::to_string(&one_snowflake).unwrap();

		  assert_eq!(serialized, "\"1303390110099968072\"".to_string());

		  let more_snowflakes: OneOrMoreSnowflakes = vec![snowflake, snowflake, snowflake].into();

		  let serialized = serde_json::to_string(&more_snowflakes).unwrap();

		  assert_eq!(serialized, "[\"1303390110099968072\",\"1303390110099968072\",\"1303390110099968072\"]".to_string());

	 }
}
