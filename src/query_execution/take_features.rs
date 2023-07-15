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

use crate::domain::ValueUnion;

use super::IdentifyFeature;

pub trait TakeFeatures {
    type Identifier: IdentifyFeature;

    fn take_feature(&self, identifier: &Self::Identifier) -> crate::Result<Option<ValueUnion>>;
}
