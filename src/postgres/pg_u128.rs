use sqlx::postgres::{PgArgumentBuffer, PgTypeInfo, PgValueRef};
use sqlx::{decode::Decode, encode::Encode, postgres::Postgres, Type, error::BoxDynError};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
use std::ops::Deref;

// This module provides rust support for https://github.com/pg-uint/pg-uint128
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PgU128(pub u128);

impl Type<Postgres> for PgU128 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("uint16")
    }
}

impl Encode<'_, Postgres> for PgU128 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, BoxDynError> {
        buf.extend_from_slice(&self.0.to_be_bytes());
        Ok(sqlx::encode::IsNull::No)
    }
}

impl Decode<'_, Postgres> for PgU128 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let bytes = value.as_bytes()?;
        if bytes.len() != 16 {
            return Err("Invalid length (expected 16 bytes)".into());
        }

        let array: [u8; 16] = bytes.try_into()?;
        Ok(PgU128(u128::from_be_bytes(array)))
    }
}

impl From<u128> for PgU128 {
    fn from(val: u128) -> Self {
        Self(val)
    }
}

impl From<PgU128> for u128 {
    fn from(val: PgU128) -> Self {
        val.0
    }
}

impl std::ops::Add for PgU128 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl std::ops::AddAssign for PgU128 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl std::fmt::Display for PgU128 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for PgU128 {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PgU64(pub u64);

impl Type<Postgres> for PgU64 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("uint8")
    }
}

impl Encode<'_, Postgres> for PgU64 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, BoxDynError> {
        buf.extend_from_slice(&self.0.to_be_bytes());
        Ok(sqlx::encode::IsNull::No)
    }
}

impl Decode<'_, Postgres> for PgU64 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let bytes = value.as_bytes()?;
        let array: [u8; 8] = bytes.try_into()?;
        Ok(PgU64(u64::from_be_bytes(array)))
    }
}

impl From<u64> for PgU64 {
    fn from(val: u64) -> Self {
        Self(val)
    }
}

impl From<PgU64> for u64 {
    fn from(val: PgU64) -> Self {
        val.0
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PgU8(pub u8);

impl Type<Postgres> for PgU8 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("INT2")
    }
}

impl Encode<'_, Postgres> for PgU8 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, BoxDynError> {
        let val = self.0 as i16;
        <i16 as Encode<Postgres>>::encode_by_ref(&val, buf)
    }
}

impl Decode<'_, Postgres> for PgU8 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let val = <i16 as Decode<Postgres>>::decode(value)?;
        Ok(PgU8(val as u8))
    }
}

impl From<u8> for PgU8 {
    fn from(v: u8) -> Self { Self(v) }
}

impl From<PgU8> for u8 {
    fn from(v: PgU8) -> Self { v.0 }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
#[repr(transparent)]
pub struct PgU32(pub u32);

impl Type<Postgres> for PgU32 {
    fn type_info() -> PgTypeInfo {
        PgTypeInfo::with_name("INT4")
    }
}

impl Encode<'_, Postgres> for PgU32 {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<sqlx::encode::IsNull, BoxDynError>{
        let val = i32::from_be_bytes(self.0.to_be_bytes());
        <i32 as Encode<Postgres>>::encode_by_ref(&val, buf)
    }
}

impl Decode<'_, Postgres> for PgU32 {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let val = <i32 as Decode<Postgres>>::decode(value)?;
        Ok(PgU32(u32::from_be_bytes(val.to_be_bytes())))
    }
}

impl From<u32> for PgU32 {
    fn from(v: u32) -> Self { Self(v) }
}

impl From<PgU32> for u32 {
    fn from(v: PgU32) -> Self { v.0 }
}
