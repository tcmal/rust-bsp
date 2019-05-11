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

const QUOTE: u8 = b'"';
const END_BRACKET: u8 = b'}';
const START_BRACKET: u8 = b'{';

use std::str;
use std::collections::HashMap;

use crate::types::{Result, Error};

#[derive(Debug, Clone)]
/// Game-related map information
pub struct EntitiesLump<'a> {
    /// The unparsed string from which entity data was extracted
    pub string: &'a str,

    /// The extracted entity data
    pub entities: Vec<Entity<'a>>
}

#[derive(Debug, Clone, PartialEq)]
/// A game entity
pub struct Entity<'a> {
    pub attributes: HashMap<&'a str, &'a str>,
}

/// Internal enum to parse through the entities string.
#[derive(PartialEq, Eq)]
enum ParseState {
    InKey, InValue, AfterKey, InsideEntity, OutsideEntity
}

impl<'a> EntitiesLump<'a> {
    /// Parse the given lump as an Entities Lump.
    pub fn from_lump(lump: &'a [u8]) -> Result<EntitiesLump<'a>> {
        use self::ParseState::*;

        let string = str::from_utf8(lump).unwrap();
        

        let mut attrs = HashMap::new();
        let mut entities = Vec::new();

        let mut state = ParseState::OutsideEntity;

        let mut key_start = 0;
        let mut key_end = 0;
        let mut val_start = 0;
        let mut val_end;

        
        for (i, chr) in string.bytes().enumerate() {
            match chr {
                QUOTE => match state {
                    InsideEntity => {
                        state = ParseState::InKey;
                        key_start = i + 1;
                    },
                    InKey => {
                        state = ParseState::AfterKey;
                        key_end = i;
                    },
                    AfterKey => {
                        state = ParseState::InValue;
                        val_start = i + 1;
                    },
                    InValue => {
                        state = ParseState::InsideEntity;
                        val_end = i;

                        attrs.insert(&string[key_start..key_end], &string[val_start..val_end]);
                    },
                    _ => {
                        return Err(Error::BadFormat);
                    }
                },
                END_BRACKET => {
                    if state != InsideEntity {
                        return Err(Error::BadFormat);
                    }

                    state = OutsideEntity;
                    
                    entities.push(Entity {attributes: attrs});
                    attrs = HashMap::new();
                },
                START_BRACKET => {
                    if state != OutsideEntity {
                        return Err(Error::BadFormat);
                    }
                    state = InsideEntity;
                }
                _ => {}
            }
        }
        Ok(EntitiesLump { string, entities })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
        };
    );

    #[test]
    fn entities_valid() {
        let valid_string = r#"
            {
                "classname" "weapon_rocketlauncher"
                "origin" "1 2 3"
                "angle" "90"
            }
            {
                "classname" "worldspawn"
                "message" "Hello, World!"
            }
        "#;

        let valid = EntitiesLump::from_lump(valid_string.as_bytes()).unwrap();

        assert_eq!(valid.entities, vec![
            Entity {
                attributes: map!(
                    "classname" => "weapon_rocketlauncher",
                    "origin" => "1 2 3",
                    "angle" => "90"
                )
            },
            Entity {
                attributes: map!(
                    "classname" => "worldspawn",
                    "message" => "Hello, World!"
                )
            }
        ]);
    }
}
