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

use bedrock_orm::{
    database_providers::{SqliteConnection, SqliteQuery},
    query_execution::{ExecuteQuery, QueryResult, TakeFeatures},
};

fn connect_memory<'connection>() -> SqliteConnection<'connection> {
    SqliteConnection::connect_memory().expect("unable to connect to sqlite database in memory")
}

fn create_table_users(connection: &SqliteConnection) {
    let mut query = SqliteQuery::new_without_results(
        &connection,
        "CREATE TABLE users (name TEXT, age INTEGER)",
    )
    .expect("unable to create query");

    let query_result = connection
        .execute(&mut query)
        .expect("unable to execute query");

    assert!(matches!(query_result, QueryResult::None));
}

fn select_users(connection: &SqliteConnection) -> Vec<(String, u32)> {
    let mut query = SqliteQuery::new_with_iterator(connection, "SELECT name, age FROM users")
        .expect("unable to create query");

    let query_result = connection
        .execute(&mut query)
        .expect("unable to execute query");

    if let QueryResult::Iterator { row_iterator } = query_result {
        let name = "name".to_owned();
        let age = "age".to_owned();

        row_iterator
            .map(|row| {
                (
                    TryInto::<&String>::try_into(
                        row.take_feature(&name)
                            .expect("unable to take feature")
                            .expect("feature cannot be null"),
                    )
                    .expect("unable to convert feature")
                    .clone(),
                    TryInto::<u32>::try_into(
                        row.take_feature(&age)
                            .expect("unable to take feature")
                            .expect("feature cannot be null"),
                    )
                    .expect("unable to convert feature"),
                )
            })
            .collect::<Vec<_>>()
    } else {
        panic!("query result is not an iterator");
    }
}

fn select_users_count(connection: &SqliteConnection) -> i64 {
    let mut query =
        SqliteQuery::new_with_iterator(connection, "SELECT COUNT(*) as count FROM users")
            .expect("unable to create query");

    let query_result = connection
        .execute(&mut query)
        .expect("unable to execute query");

    if let QueryResult::Iterator { row_iterator } = query_result {
        let count = "count".to_owned();

        row_iterator
            .map(|row| {
                TryInto::<i64>::try_into(
                    row.take_feature(&count)
                        .expect("unable to take feature")
                        .expect("feature cannot be null"),
                )
                .expect("unable to convert feature")
                .clone()
            })
            .next()
            .expect("unable to get first row")
    } else {
        panic!("query result is not an iterator");
    }
}

#[test]
fn test_connect_memory() {
    connect_memory();
}

#[test]
fn test_create_table_users() {
    let connection = connect_memory();

    create_table_users(&connection);
}

#[test]
fn test_users_empty() {
    let connection = connect_memory();

    create_table_users(&connection);

    assert!(select_users(&connection).is_empty());
    assert_eq!(select_users_count(&connection), 0);
}
