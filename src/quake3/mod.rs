//! Tools for representing Quake 3 BSP files.
//! Q3 BSP files start with "IBSP" and have version number 0x2e

/// The lump directory located at the start of the file
pub mod directory;

use crate::quake3::directory::Header;

/// Represents a parsed BSP file.
pub struct BSPFile {
    pub directory: Header
}
