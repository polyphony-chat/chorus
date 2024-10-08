// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::errors::ChorusError;
use crate::types::{Shared, Snowflake};

use super::{arc_rwlock_ptr_eq, PublicUser};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
/// See <https://discord-userdoccers.vercel.app/resources/user#relationship-structure>
pub struct Relationship {
    /// The ID of the target user
    #[cfg_attr(feature = "sqlx", sqlx(rename = "to_id"))]
    pub id: Snowflake,
    #[serde(rename = "type")]
    #[cfg_attr(feature = "sqlx", sqlx(rename = "type"))]
    pub relationship_type: RelationshipType,
    #[cfg_attr(feature = "sqlx", sqlx(skip))] // Can be derived from the user id
    /// The nickname of the user in this relationship
    pub nickname: Option<String>,
    #[cfg_attr(feature = "sqlx", sqlx(skip))] // Can be derived from the user id
    /// The target user
    pub user: Shared<PublicUser>,
    /// When the user requested a relationship
    pub since: Option<DateTime<Utc>>,
}

#[cfg(not(tarpaulin_include))]
impl PartialEq for Relationship {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.relationship_type == other.relationship_type
            && self.nickname == other.nickname
            && arc_rwlock_ptr_eq(&self.user, &other.user)
            && self.since == other.since
    }
}

#[derive(
    Serialize_repr,
    Deserialize_repr,
    Debug,
    Clone,
    Default,
    Eq,
    PartialEq,
    PartialOrd,
    Ord,
    Copy,
    Hash,
)]
#[repr(u8)]
/// See <https://discord-userdoccers.vercel.app/resources/user#relationship-type>
pub enum RelationshipType {
    Suggestion = 6,
    Implicit = 5,
    Outgoing = 4,
    Incoming = 3,
    Blocked = 2,
    #[default]
    Friends = 1,
    None = 0,
}

#[cfg(feature = "sqlx")]
impl sqlx::Type<sqlx::Postgres> for RelationshipType {
    fn type_info() -> <sqlx::Postgres as sqlx::Database>::TypeInfo {
        <sqlx_pg_uint::PgU8 as sqlx::Type<sqlx::Postgres>>::type_info()
    }
}

#[cfg(feature = "sqlx")]
impl<'q> sqlx::Encode<'q, sqlx::Postgres> for RelationshipType {
    fn encode_by_ref(
        &self,
        buf: &mut <sqlx::Postgres as sqlx::Database>::ArgumentBuffer<'q>,
    ) -> Result<sqlx::encode::IsNull, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::from(*self as u8);
        sqlx_pg_uint.encode_by_ref(buf)
    }
}

#[cfg(feature = "sqlx")]
impl<'r> sqlx::Decode<'r, sqlx::Postgres> for RelationshipType {
    fn decode(
        value: <sqlx::Postgres as sqlx::Database>::ValueRef<'r>,
    ) -> Result<Self, sqlx::error::BoxDynError> {
        let sqlx_pg_uint = sqlx_pg_uint::PgU8::decode(value)?;
        Self::try_from(sqlx_pg_uint.to_uint()).map_err(|e| e.into())
    }
}

impl TryFrom<u8> for RelationshipType {
    type Error = ChorusError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Self::Suggestion),
            5 => Ok(Self::Implicit),
            4 => Ok(Self::Outgoing),
            3 => Ok(Self::Incoming),
            2 => Ok(Self::Blocked),
            1 => Ok(Self::Friends),
            0 => Ok(Self::None),
            _ => Err(ChorusError::InvalidArguments {
                error: format!("Value {} is not a valid RelationshipType", value),
            }),
        }
    }
}
