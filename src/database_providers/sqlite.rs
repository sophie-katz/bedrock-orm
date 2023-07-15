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

use lazy_static::lazy_static;
use regex::Regex;
use sqlite::{Connection, Statement};
use std::{marker::PhantomData, ops::Index};

use crate::{
    domain::ValueUnion,
    query_execution::{
        ExecuteQuery, GetQueryResultType, InjectFeatures, QueryResultType, TakeFeatures,
    },
};

const DEFAULT_TRUE_STRING: &str = "true";
const DEFAULT_FALSE_STRING: &str = "false";
const DEFAULT_DATE_FORMAT: &str = "%F";
const DEFAULT_DATETIME_FORMAT: &str = "%+";

lazy_static! {
    static ref IDENTIFIER_REGEX: Regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*$").unwrap();
}

pub struct SqliteConnection<'connection> {
    sqlite_connection: Connection,
    true_string: String,
    false_string: String,
    date_format: String,
    datetime_format: String,
    phantom: PhantomData<&'connection ()>,
}

pub struct SqliteQuery<'connection> {
    connection: &'connection SqliteConnection<'connection>,
    statement: Statement<'connection>,
    result_type: QueryResultType,
}

impl<'connection> GetQueryResultType for SqliteQuery<'connection> {
    fn query_result_type(&self) -> QueryResultType {
        self.result_type
    }
}

impl<'connection> SqliteConnection<'connection> {
    pub fn connect_memory() -> crate::Result<Self> {
        Ok(Self {
            sqlite_connection: sqlite::open(":memory:")?,
            true_string: DEFAULT_TRUE_STRING.to_owned(),
            false_string: DEFAULT_FALSE_STRING.to_owned(),
            date_format: DEFAULT_DATE_FORMAT.to_owned(),
            datetime_format: DEFAULT_DATETIME_FORMAT.to_owned(),
            phantom: PhantomData,
        })
    }
}

impl<'connection> SqliteQuery<'connection> {
    fn new_with_result_type(
        connection: &'connection SqliteConnection,
        query_text: &str,
        result_type: QueryResultType,
    ) -> crate::Result<Self> {
        Ok(Self {
            connection,
            statement: connection.sqlite_connection.prepare(query_text)?,
            result_type,
        })
    }

    pub fn new_without_results(
        connection: &'connection SqliteConnection,
        query_text: &str,
    ) -> crate::Result<Self> {
        Self::new_with_result_type(connection, query_text, QueryResultType::None)
    }

    pub fn new_with_change_count(
        connection: &'connection SqliteConnection,
        query_text: &str,
    ) -> crate::Result<Self> {
        Self::new_with_result_type(connection, query_text, QueryResultType::ChangeCount)
    }

    pub fn new_with_iterator(
        connection: &'connection SqliteConnection,
        query_text: &str,
    ) -> crate::Result<Self> {
        Self::new_with_result_type(connection, query_text, QueryResultType::Iterator)
    }
}

impl<'connection> ExecuteQuery for SqliteConnection<'connection> {
    type Query = SqliteQuery<'connection>;
    type Row = Result<sqlite::Row, sqlite::Error>;
    type RowIterator<'query>
    = sqlite::Cursor<'connection, 'query>
    where
        'connection: 'query;

    fn execute_without_results(&self, query: &mut Self::Query) -> crate::Result<()> {
        query.statement.next()?;

        Ok(())
    }

    fn execute_with_change_count(&self, query: &mut Self::Query) -> crate::Result<usize> {
        query.statement.next()?;

        Ok(self.sqlite_connection.change_count())
    }

    fn execute_with_iterator<'query>(
        &self,
        query: &'query mut Self::Query,
    ) -> crate::Result<Self::RowIterator<'query>> {
        println!("columns: {:?}", query.statement.column_names());

        Ok(query.statement.iter())
    }
}

impl<'connection> InjectFeatures for SqliteQuery<'connection> {
    type Identifier = String;

    fn inject_feature(
        &mut self,
        identifier: &Self::Identifier,
        value: &ValueUnion,
    ) -> crate::Result<()> {
        if !IDENTIFIER_REGEX.is_match(identifier) {
            return Err(crate::Error::InvalidFeatureName {
                feature_name: identifier.clone(),
            });
        }

        let binding_index = format!(":{}", identifier);

        match value {
            ValueUnion::Bool(value) => {
                if *value {
                    self.statement
                        .bind((binding_index.as_str(), self.connection.true_string.as_str()))
                } else {
                    self.statement.bind((
                        binding_index.as_str(),
                        self.connection.false_string.as_str(),
                    ))
                }
            }
            ValueUnion::U8(value) => self.statement.bind((binding_index.as_str(), *value as i64)),
            ValueUnion::U16(value) => self.statement.bind((binding_index.as_str(), *value as i64)),
            ValueUnion::U32(value) => self.statement.bind((binding_index.as_str(), *value as i64)),
            ValueUnion::U64(value) => {
                if let Ok(value) = i64::try_from(*value) {
                    self.statement.bind((binding_index.as_str(), value))
                } else {
                    self.statement
                        .bind((binding_index.as_str(), value.to_string().as_str()))
                }
            }
            ValueUnion::I8(value) => self.statement.bind((binding_index.as_str(), *value as i64)),
            ValueUnion::I16(value) => self.statement.bind((binding_index.as_str(), *value as i64)),
            ValueUnion::I32(value) => self.statement.bind((binding_index.as_str(), *value as i64)),
            ValueUnion::I64(value) => self.statement.bind((binding_index.as_str(), *value)),
            ValueUnion::F32(value) => self.statement.bind((binding_index.as_str(), *value as f64)),
            ValueUnion::F64(value) => self.statement.bind((binding_index.as_str(), *value)),
            ValueUnion::String(value) => self
                .statement
                .bind((binding_index.as_str(), value.as_str())),
            ValueUnion::Bytestring(value) => self.statement.bind((binding_index.as_str(), *value)),
            ValueUnion::Date(value) => self.statement.bind((
                binding_index.as_str(),
                value
                    .format(self.connection.date_format.as_str())
                    .to_string()
                    .as_str(),
            )),
            ValueUnion::DateTime(value) => self.statement.bind((
                binding_index.as_str(),
                value
                    .format(self.connection.datetime_format.as_str())
                    .to_string()
                    .as_str(),
            )),
        }?;

        Ok(())
    }
}

fn take_feature_from_row<'value>(
    row: &'value sqlite::Row,
    identifier: &str,
) -> crate::Result<Option<ValueUnion<'value>>> {
    if !IDENTIFIER_REGEX.is_match(identifier) {
        return Err(crate::Error::InvalidFeatureName {
            feature_name: identifier.to_owned(),
        });
    }

    match row.index(identifier) {
        sqlite::Value::Binary(value) => Ok(Some(ValueUnion::Bytestring(value))),
        sqlite::Value::Float(value) => Ok(Some(ValueUnion::F64(*value))),
        sqlite::Value::Integer(value) => Ok(Some(ValueUnion::I64(*value))),
        sqlite::Value::String(value) => Ok(Some(ValueUnion::String(value))),
        sqlite::Value::Null => Ok(None),
    }
}

impl TakeFeatures for Result<sqlite::Row, sqlite::Error> {
    type Identifier = String;

    fn take_feature(&self, identifier: &Self::Identifier) -> crate::Result<Option<ValueUnion>> {
        match self {
            Ok(row) => take_feature_from_row(row, identifier),
            // sqlite::Error does not implement Clone, so we have to manually clone it
            Err(error) => Err(crate::Error::SqliteError {
                sqlite_error: sqlite::Error {
                    code: error.code,
                    message: error.message.clone(),
                },
            }),
        }
    }
}
