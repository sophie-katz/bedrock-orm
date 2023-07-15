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

#[derive(Debug, PartialEq)]
pub enum DataType {
    Bool,
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
    F32,
    F64,
    String,
    Bytestring,
    Date,
    DateTime,
}

impl DataType {
    pub fn is_integral(&self) -> bool {
        matches!(
            self,
            Self::U8
                | Self::U16
                | Self::U32
                | Self::U64
                | Self::I8
                | Self::I16
                | Self::I32
                | Self::I64
        )
    }

    pub fn is_signed(&self) -> Option<bool> {
        match self {
            Self::U8 => Some(false),
            Self::U16 => Some(false),
            Self::U32 => Some(false),
            Self::U64 => Some(false),
            Self::I8 => Some(true),
            Self::I16 => Some(true),
            Self::I32 => Some(true),
            Self::I64 => Some(true),
            _ => None,
        }
    }

    pub fn bit_width(&self) -> Option<u32> {
        match self {
            Self::U8 => Some(8),
            Self::U16 => Some(16),
            Self::U32 => Some(32),
            Self::U64 => Some(64),
            Self::I8 => Some(8),
            Self::I16 => Some(16),
            Self::I32 => Some(32),
            Self::I64 => Some(64),
            _ => None,
        }
    }

    pub fn is_sequential(&self) -> bool {
        matches!(self, Self::String | Self::Bytestring)
    }
}
