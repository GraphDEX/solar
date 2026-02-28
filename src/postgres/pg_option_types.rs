use sqlx::postgres::{PgArgumentBuffer, PgValueRef};
use sqlx::encode::IsNull;
use sqlx::{Decode, Encode, Postgres, Type};
use sqlx::error::BoxDynError;

// A wrapper that treats NULL values as -1i32:
//  None -> -1
//  Some(v) -> v
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PgI32Option(pub Option<i32>);

impl Type<Postgres> for PgI32Option {
    fn type_info() -> <Postgres as sqlx::Database>::TypeInfo {
        <i32 as Type<Postgres>>::type_info()
    }
}

impl Encode<'_, Postgres> for PgI32Option {
    fn encode_by_ref(&self, buf: &mut PgArgumentBuffer) -> Result<IsNull, BoxDynError> {
        let val = match self.0 {
            Some(v) => v,
            None => -1i32,
        };
        <i32 as Encode<Postgres>>::encode_by_ref(&val, buf)
    }
}

impl Decode<'_, Postgres> for PgI32Option {
    fn decode(value: PgValueRef<'_>) -> Result<Self, BoxDynError> {
        let val = <i32 as Decode<Postgres>>::decode(value)?;
        Ok(PgI32Option(if val == -1 { None } else { Some(val) }))
    }
}

impl From<Option<i32>> for PgI32Option {
    fn from(value: Option<i32>) -> Self {
        PgI32Option(value)
    }
}

impl From<PgI32Option> for Option<i32> {
    fn from(value: PgI32Option) -> Self {
        value.0
    }
}

impl From<i32> for PgI32Option {
    fn from(value: i32) -> Self {
        PgI32Option(Some(value))
    }
}
