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

use std::{fmt::Debug, hash::Hash};

/// A type that can be used to identify a query parameter for both the query itself and any value
/// that can be bound to the query.
///
/// - If your query passes in parameters by index, you can use `usize` as your query parameter
///   type.
/// - You can use a custom struct or enum to identify your query parameters. Just make sure to
///   implement `IdentifyQueryParameter` for it. See the below example for more details.
///
/// # Examples
///
/// This is an example of using a custom type to identify query parameters:
///
/// ```
/// # use bedrock_orm::query_builder::IdentifyQueryParameter;
/// #
/// #[derive(Debug, Hash, PartialEq)]
/// enum MyQueryParameterEnum {
///     A,
///     B,
/// }
///
/// impl IdentifyQueryParameter for MyQueryParameterEnum {}
/// ```
pub trait IdentifyQueryParameter: Debug + Hash + PartialEq {}

impl IdentifyQueryParameter for usize {}

#[cfg(test)]
mod tests {
    use super::*;

    fn uses_query_parameter<T: IdentifyQueryParameter>(_: T) {}

    #[derive(Debug, Hash, PartialEq)]
    enum MyQueryParameterEnum {
        A,
        B,
    }

    impl IdentifyQueryParameter for MyQueryParameterEnum {}

    #[test]
    fn integer_indexed() {
        uses_query_parameter(0usize);
        uses_query_parameter(1usize);
    }

    #[test]
    fn enum_indexed() {
        uses_query_parameter(MyQueryParameterEnum::A);
        uses_query_parameter(MyQueryParameterEnum::B);
    }
}
