// Copyright (C) 2019 Oscar Shrimpton
// 
// This file is part of rust_bsp.
// 
// rust_bsp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// rust_bsp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with rust_bsp.  If not, see <http://www.gnu.org/licenses/>.

//! Tools for representing Quake 3 BSP files.
//! Q3 BSP files start with "IBSP" and have version number 0x2e

/// The lump directory located at the start of the file
pub mod directory;

use crate::quake3::directory::Header;

/// Represents a parsed BSP file.
pub struct BSPFile {
    pub directory: Header,
}
