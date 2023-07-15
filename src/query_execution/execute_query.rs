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

use crate::Result;

use super::{GetQueryResultType, InjectFeatures, QueryResult, QueryResultType, TakeFeatures};

pub trait ExecuteQuery {
    type Query: InjectFeatures + GetQueryResultType;
    type Row: TakeFeatures;
    type RowIterator<'query>: Iterator<Item = Self::Row>
    where
        Self: 'query;

    fn execute_without_results(&self, query: &mut Self::Query) -> Result<()>;

    fn execute_with_change_count(&self, query: &mut Self::Query) -> Result<usize>;

    fn execute_with_iterator<'query>(
        &self,
        query: &'query mut Self::Query,
    ) -> Result<Self::RowIterator<'query>>;

    fn execute<'query>(
        &self,
        query: &'query mut Self::Query,
    ) -> Result<QueryResult<Self::Row, Self::RowIterator<'query>>> {
        match query.query_result_type() {
            QueryResultType::None => {
                self.execute_without_results(query)?;
                Ok(QueryResult::None)
            }
            QueryResultType::ChangeCount => {
                let count = self.execute_with_change_count(query)?;
                Ok(QueryResult::ChangeCount { count })
            }
            QueryResultType::Iterator => {
                let row_iterator = self.execute_with_iterator(query)?;
                Ok(QueryResult::Iterator { row_iterator })
            }
        }
    }
}
