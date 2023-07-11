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

use crate::query_builder::IdentifyQueryParameter;

/// A trait for types that can provide query parameters.
///
/// This is usually used for structs that can be bound to queries to provide parameter values.
pub trait ProvideQueryParameters {
    /// Checks if the value provides a parameter with the given identifier.
    ///
    /// # Performance
    ///
    /// It is implementation-specific, but since identifiers can be hashed ideally it will be
    /// `O(1)`.
    fn contains_key(&self, identifier: &impl IdentifyQueryParameter) -> bool;

    /// Tries to get the value of the parameter with the given identifier.
    ///
    /// # Returns
    ///
    /// - `Some(parameter_value)` if the value provides a parameter with the given identifier.
    /// - `None` otherwise.
    ///
    /// # Performance
    ///
    /// It is implementation-specific, but since identifiers can be hashed ideally it will be
    /// `O(1)`.
    fn get<ParameterValue>(
        &self,
        identifier: &impl IdentifyQueryParameter,
    ) -> Option<&ParameterValue>;

    /// Tries to get a mutable reference to the value of the parameter with the given identifier.
    ///
    /// # Returns
    ///
    /// - `Some(parameter_value)` if the value provides a parameter with the given identifier.
    /// - `None` otherwise.
    ///
    /// # Performance
    ///
    /// It is implementation-specific, but since identifiers can be hashed ideally it will be
    /// `O(1)`.
    fn get_mut<ParameterValue>(
        &mut self,
        identifier: &impl IdentifyQueryParameter,
    ) -> Option<&mut ParameterValue>;
}
