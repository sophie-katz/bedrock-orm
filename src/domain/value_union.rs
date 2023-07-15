// Copyright (c) 2023 Sophie Katz
//
// This file is part of Bedrock ORM.
//
// Bedrock ORM is free software: you can redistribute it and/or modify it under the terms of the
// GNU General Public License as published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// Bedrock ORM is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without
// even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//
// You should have received a copy of the GNU General Public License along with Bedrock ORM. If
// not, see <https://www.gnu.org/licenses/>.

use chrono::{NaiveDate, NaiveDateTime};

use super::DataType;

pub enum ValueUnion<'value> {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    String(&'value String),
    Bytestring(&'value [u8]),
    Date(&'value NaiveDate),
    DateTime(&'value NaiveDateTime),
}

impl<'value> ValueUnion<'value> {
    pub fn data_type(&self) -> DataType {
        match self {
            Self::Bool(_) => DataType::Bool,
            Self::U8(_) => DataType::U8,
            Self::U16(_) => DataType::U16,
            Self::U32(_) => DataType::U32,
            Self::U64(_) => DataType::U64,
            Self::I8(_) => DataType::I8,
            Self::I16(_) => DataType::I16,
            Self::I32(_) => DataType::I32,
            Self::I64(_) => DataType::I64,
            Self::F32(_) => DataType::F32,
            Self::F64(_) => DataType::F64,
            Self::String(_) => DataType::String,
            Self::Bytestring(_) => DataType::Bytestring,
            Self::Date(_) => DataType::Date,
            Self::DateTime(_) => DataType::DateTime,
        }
    }
}

impl<'value> From<bool> for ValueUnion<'value> {
    fn from(value: bool) -> Self {
        Self::Bool(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for bool {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::Bool(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::Bool,
            }),
        }
    }
}

impl<'value> From<u8> for ValueUnion<'value> {
    fn from(value: u8) -> Self {
        Self::U8(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for u8 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::U8(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::U8,
            }),
        }
    }
}

impl<'value> From<u16> for ValueUnion<'value> {
    fn from(value: u16) -> Self {
        Self::U16(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for u16 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::U16(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::U16,
            }),
        }
    }
}

impl<'value> From<u32> for ValueUnion<'value> {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for u32 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::U32(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::U32,
            }),
        }
    }
}

impl<'value> From<u64> for ValueUnion<'value> {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for u64 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::U64(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::U64,
            }),
        }
    }
}

impl<'value> From<i8> for ValueUnion<'value> {
    fn from(value: i8) -> Self {
        Self::I8(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for i8 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::I8(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::I8,
            }),
        }
    }
}

impl<'value> From<i16> for ValueUnion<'value> {
    fn from(value: i16) -> Self {
        Self::I16(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for i16 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::I16(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::I16,
            }),
        }
    }
}

impl<'value> From<i32> for ValueUnion<'value> {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for i32 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::I32(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::I32,
            }),
        }
    }
}

impl<'value> From<i64> for ValueUnion<'value> {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for i64 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::I64(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::I64,
            }),
        }
    }
}

impl<'value> From<f32> for ValueUnion<'value> {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for f32 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::F32(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::F32,
            }),
        }
    }
}

impl<'value> From<f64> for ValueUnion<'value> {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for f64 {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::F64(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::F64,
            }),
        }
    }
}

impl<'value> From<&'value String> for ValueUnion<'value> {
    fn from(value: &'value String) -> Self {
        Self::String(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for &'value String {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::String(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::String,
            }),
        }
    }
}

impl<'value> From<&'value [u8]> for ValueUnion<'value> {
    fn from(value: &'value [u8]) -> Self {
        Self::Bytestring(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for &'value [u8] {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::Bytestring(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::Bytestring,
            }),
        }
    }
}

impl<'value> From<&'value NaiveDate> for ValueUnion<'value> {
    fn from(value: &'value NaiveDate) -> Self {
        Self::Date(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for &'value NaiveDate {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::Date(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::Date,
            }),
        }
    }
}

impl<'value> From<&'value NaiveDateTime> for ValueUnion<'value> {
    fn from(value: &'value NaiveDateTime) -> Self {
        Self::DateTime(value)
    }
}

impl<'value> TryFrom<ValueUnion<'value>> for &'value NaiveDateTime {
    type Error = crate::Error;

    fn try_from(value: ValueUnion<'value>) -> Result<Self, Self::Error> {
        match value {
            ValueUnion::DateTime(value) => Ok(value),
            _ => Err(crate::Error::ValueCannotBeAccessedAsRequestedType {
                value_type: value.data_type(),
                requested_type: DataType::DateTime,
            }),
        }
    }
}
