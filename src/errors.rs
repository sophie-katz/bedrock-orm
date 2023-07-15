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

use crate::domain::DataType;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(
        "value of type `{value_type:?}` cannot be accessed as requested type `{requested_type:?}`"
    )]
    ValueCannotBeAccessedAsRequestedType {
        value_type: DataType,
        requested_type: DataType,
    },
    #[error("sqlite error: {sqlite_error}")]
    SqliteError { sqlite_error: sqlite::Error },
    #[error("invalid feature name: {feature_name:?}")]
    InvalidFeatureName { feature_name: String },
}

impl From<sqlite::Error> for Error {
    fn from(value: sqlite::Error) -> Self {
        Self::SqliteError {
            sqlite_error: value,
        }
    }
}

pub type Result<Value> = std::result::Result<Value, Error>;
