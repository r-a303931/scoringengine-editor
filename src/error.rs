// error.rs: Common errors that could be used throughout the application
//
// Copyright (C) 2023 Andrew Rioux
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::{error::Error, fmt::Display};

use crate::config::ConversionError;

#[derive(Debug)]
pub enum EditorError {
    Conversion(ConversionError),
    Serialize(serde_yaml::Error),
}

impl Display for EditorError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Conversion(err) => write!(f, "error converting configuration: {err}"),
            Self::Serialize(err) => write!(f, "error serializing configuration: {err}"),
        }
    }
}

impl Error for EditorError {}

impl From<ConversionError> for EditorError {
    fn from(err: ConversionError) -> Self {
        Self::Conversion(err)
    }
}

impl From<serde_yaml::Error> for EditorError {
    fn from(err: serde_yaml::Error) -> Self {
        Self::Serialize(err)
    }
}
