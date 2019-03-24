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

use crate::{Result, Error};
use std::convert::TryInto;

/// "IBSP"
const MAGIC_HEADER: &[u8] = &[0x49, 0x42, 0x53, 0x50];
const HEADER_LEN: usize = 4 + 4 + (17 * 4 * 2);

/// The header found at the start of a (Q3) bsp file
#[derive(Clone, Copy, Debug)]
pub struct Header {
    pub version: u32,
    pub dir_entries: [DirEntry; 17]
}

/// A directory entry, pointing to a lump in the file
#[derive(Clone, Copy, Debug)]
pub struct DirEntry {
    
    /// Offset from beginning of file to start of lump
    pub offset: u32,

    /// Length of lump, multiple of 4.
    pub length: u32
}

impl Header {

    /// Deserialise from buffer.
    /// # Format
    /// string[4] magic             Magic number. Always "IBSP".
    /// int version                 Version number. 0x2e for the BSP files distributed with Quake 3.
    /// direntry[17] direntries     Lump directory, seventeen entries. 
    pub fn from(v: &'_ [u8]) -> Result<'_, Header> {
        if v.len() < HEADER_LEN {
            return Err(Error::BadSize {req: 17});
        }
        let magic = &v[0..4];
        
        if magic != MAGIC_HEADER {
            return Err(Error::BadMagic {expected: MAGIC_HEADER, actual: magic});
        }


        let version: &[u8; 4] = v[4..8].try_into().unwrap();

        let entries: &[u8] = &v[8..144];
        let mut dir_entries: [DirEntry; 17] = [DirEntry { offset: 0, length: 0}; 17];
        
        for n in 0..17 {
            let base = &entries[(n * 8)..(n * 8) + 8];
            dir_entries[n] = DirEntry {
                offset: u32::from_le_bytes(base[0..4].try_into().unwrap()),
                length: u32::from_le_bytes(base[4..8].try_into().unwrap())
            }
        }

        Ok(Header {version: u32::from_le_bytes(*version), dir_entries})
    }

}

#[test]
fn header() {
    let data = [0x49, 0x42, 0x53, 0x50, // magic number (IBSP)
                0x2e, 0x00, 0x00, 0x00, // version
                0x00, 0x00, 0x00, 0x00,     0xff, 0x00, 0x00, 0x00, // 17 lump directory entries.
                0x01, 0x00, 0x00, 0x00,     0xfe, 0x00, 0x00, 0x00,
                0x02, 0x00, 0x00, 0x00,     0xfd, 0x00, 0x00, 0x00,
                0x03, 0x00, 0x00, 0x00,     0xfc, 0x00, 0x00, 0x00,
                0x04, 0x00, 0x00, 0x00,     0xfb, 0x00, 0x00, 0x00,
                0x05, 0x00, 0x00, 0x00,     0xfa, 0x00, 0x00, 0x00,
                0x06, 0x00, 0x00, 0x00,     0xf9, 0x00, 0x00, 0x00,
                0x07, 0x00, 0x00, 0x00,     0xf8, 0x00, 0x00, 0x00,
                0x08, 0x00, 0x00, 0x00,     0xf7, 0x00, 0x00, 0x00,
                0x09, 0x00, 0x00, 0x00,     0xf6, 0x00, 0x00, 0x00,
                0x0a, 0x00, 0x00, 0x00,     0xf5, 0x00, 0x00, 0x00,
                0x0b, 0x00, 0x00, 0x00,     0xf4, 0x00, 0x00, 0x00,
                0x0c, 0x00, 0x00, 0x00,     0xf3, 0x00, 0x00, 0x00,
                0x0d, 0x00, 0x00, 0x00,     0xf2, 0x00, 0x00, 0x00,
                0x0e, 0x00, 0x00, 0x00,     0xf1, 0x00, 0x00, 0x00,
                0x0f, 0x00, 0x00, 0x00,     0xf0, 0x00, 0x00, 0x00,
                0x10, 0x00, 0x00, 0x00,     0xef, 0x00, 0x00, 0x00,
                ];

    // attempt to parse
    let header = Header::from(&data).unwrap();

    // validity checks
    assert_eq!(header.version, 46);

    let mut n = 0;
    for entry in &header.dir_entries {
        assert_eq!(entry.offset, n);
        assert_eq!(entry.length, 0xff - n);
        n += 1;
    }
}