use core::fmt;
use chrono::{LocalResult, NaiveDateTime};
use serde::{de, Deserialize, Deserializer};
use serde::de::Error;

#[doc(hidden)]
#[derive(Debug)]
pub struct SecondsStringTimestampVisitor;


/// Ser/de to/from timestamps in seconds
///
/// Intended for use with `serde`'s `with` attribute.
///
/// # Example:
///
/// ```rust
/// # use chrono::{TimeZone, DateTime, Utc};
/// # use serde::{Deserialize, Serialize};
/// use chorus::types::serde::ts_seconds_str;
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(with = "ts_seconds_str")]
///     time: DateTime<Utc>
/// }
///
/// let time = Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap();
/// let my_s = S {
///     time: time.clone(),
/// };
///
/// let as_string = serde_json::to_string(&my_s)?;
/// assert_eq!(as_string, r#"{"time":"1431684000"}"#);
/// let my_s: S = serde_json::from_str(&as_string)?;
/// assert_eq!(my_s.time, time);
/// # Ok::<(), serde_json::Error>(())
/// ```

pub mod ts_seconds_str {
    use core::fmt;
    use chrono::{DateTime, LocalResult, Utc};
    use super::SecondsStringTimestampVisitor;
    use serde::{de, ser};
    use chrono::TimeZone;
    use super::serde_from;

    /// Serialize a UTC datetime into an integer number of seconds since the epoch
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{TimeZone, DateTime, Utc};
    /// # use serde::Serialize;
    /// use chorus::types::serde::ts_seconds_str::serialize as to_ts;
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "to_ts")]
    ///     time: DateTime<Utc>
    /// }
    ///
    /// let my_s = S {
    ///     time: Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap(),
    /// };
    /// let as_string = serde_json::to_string(&my_s)?;
    /// assert_eq!(as_string, r#"{"time":"1431684000"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(dt: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        serializer.serialize_str(&format!("{}", dt.timestamp()))
    }

    /// Deserialize a `DateTime` from a seconds timestamp
    ///
    /// Intended for use with `serde`s `deserialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{DateTime, TimeZone, Utc};
    /// # use serde::Deserialize;
    /// use chorus::types::serde::ts_seconds_str::deserialize as from_ts;
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// struct S {
    ///     #[serde(deserialize_with = "from_ts")]
    ///     time: DateTime<Utc>
    /// }
    ///
    /// let my_s: S = serde_json::from_str(r#"{ "time": "1431684000" }"#)?;
    /// assert_eq!(my_s, S { time: Utc.timestamp_opt(1431684000, 0).unwrap() });
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn deserialize<'de, D>(d: D) -> Result<DateTime<Utc>, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        d.deserialize_str(SecondsStringTimestampVisitor)
    }

    impl<'de> de::Visitor<'de> for SecondsStringTimestampVisitor {
        type Value = DateTime<Utc>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unix timestamp in seconds")
        }

        /// Deserialize a timestamp in seconds since the epoch
        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            serde_from(Utc.timestamp_opt(value.parse::<i64>().map_err(|e| E::custom(e))?, 0), &value)
        }
    }
}

/// Ser/de to/from optional timestamps in seconds
///
/// Intended for use with `serde`'s `with` attribute.
///
/// # Example:
///
/// ```rust
/// # use chrono::{TimeZone, DateTime, Utc};
/// # use serde::{Deserialize, Serialize};
/// use chorus::types::serde::ts_seconds_option_str;
/// #[derive(Deserialize, Serialize)]
/// struct S {
///     #[serde(with = "ts_seconds_option_str")]
///     time: Option<DateTime<Utc>>
/// }
///
/// let time = Some(Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap());
/// let my_s = S {
///     time: time.clone(),
/// };
///
/// let as_string = serde_json::to_string(&my_s)?;
/// assert_eq!(as_string, r#"{"time":"1431684000"}"#);
/// let my_s: S = serde_json::from_str(&as_string)?;
/// assert_eq!(my_s.time, time);
/// # Ok::<(), serde_json::Error>(())
/// ```
pub mod ts_seconds_option_str {
    use core::fmt;
    use chrono::{DateTime, Utc};
    use serde::{de, ser};
    use super::SecondsStringTimestampVisitor;

    /// Serialize a UTC datetime into an integer number of seconds since the epoch or none
    ///
    /// Intended for use with `serde`s `serialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{TimeZone, DateTime, Utc};
    /// # use serde::Serialize;
    /// use chorus::types::serde::ts_seconds_option_str::serialize as from_tsopt;
    /// #[derive(Serialize)]
    /// struct S {
    ///     #[serde(serialize_with = "from_tsopt")]
    ///     time: Option<DateTime<Utc>>
    /// }
    ///
    /// let my_s = S {
    ///     time: Some(Utc.with_ymd_and_hms(2015, 5, 15, 10, 0, 0).unwrap()),
    /// };
    /// let as_string = serde_json::to_string(&my_s)?;
    /// assert_eq!(as_string, r#"{"time":"1431684000"}"#);
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn serialize<S>(opt: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ser::Serializer,
    {
        match *opt {
            Some(ref dt) => serializer.serialize_some(&dt.timestamp().to_string()),
            None => serializer.serialize_none(),
        }
    }

    /// Deserialize a `DateTime` from a seconds timestamp or none
    ///
    /// Intended for use with `serde`s `deserialize_with` attribute.
    ///
    /// # Example:
    ///
    /// ```rust
    /// # use chrono::{DateTime, TimeZone, Utc};
    /// # use serde::Deserialize;
    /// use chorus::types::serde::ts_seconds_option_str::deserialize as from_tsopt;
    /// #[derive(Debug, PartialEq, Deserialize)]
    /// struct S {
    ///     #[serde(deserialize_with = "from_tsopt")]
    ///     time: Option<DateTime<Utc>>
    /// }
    ///
    /// let my_s: S = serde_json::from_str(r#"{ "time": "1431684000" }"#)?;
    /// assert_eq!(my_s, S { time: Utc.timestamp_opt(1431684000, 0).single() });
    /// # Ok::<(), serde_json::Error>(())
    /// ```
    pub fn deserialize<'de, D>(d: D) -> Result<Option<DateTime<Utc>>, D::Error>
        where
            D: de::Deserializer<'de>,
    {
        d.deserialize_option(OptionSecondsTimestampVisitor)
    }

    struct OptionSecondsTimestampVisitor;

    impl<'de> de::Visitor<'de> for OptionSecondsTimestampVisitor {
        type Value = Option<DateTime<Utc>>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a unix timestamp in seconds or none")
        }

        /// Deserialize a timestamp in seconds since the epoch
        fn visit_some<D>(self, d: D) -> Result<Self::Value, D::Error>
            where
                D: de::Deserializer<'de>,
        {
            d.deserialize_str(SecondsStringTimestampVisitor).map(Some)
        }

        /// Deserialize a timestamp in seconds since the epoch
        fn visit_none<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            Ok(None)
        }

        /// Deserialize a timestamp in seconds since the epoch
        fn visit_unit<E>(self) -> Result<Self::Value, E>
            where
                E: de::Error,
        {
            Ok(None)
        }
    }
}

pub(crate) fn serde_from<T, E, V>(me: LocalResult<T>, _ts: &V) -> Result<T, E>
    where
        E: de::Error,
        V: fmt::Display,
        T: fmt::Display,
{
    // TODO: Make actual error type
    match me {
        LocalResult::None => Err(E::custom("value is not a legal timestamp")),
        LocalResult::Ambiguous(_min, _max) => {
            Err(E::custom("value is an ambiguous timestamp"))
        }
        LocalResult::Single(val) => Ok(val),
    }
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(untagged)]
enum StringOrU64 {
    String(String),
    U64(u64),
}

pub fn string_or_u64<'de, D>(d: D) -> Result<u64, D::Error>
where D: Deserializer<'de> {
    match StringOrU64::deserialize(d)? {
        StringOrU64::String(s) => s.parse::<u64>().map_err(D::Error::custom),
        StringOrU64::U64(u) => Ok(u)
    }
}