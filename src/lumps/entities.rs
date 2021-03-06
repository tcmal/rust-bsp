// Copyright (C) 2019 Oscar Shrimpton
//
// This file is part of stockton-bsp.
//
// stockton-bsp is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// stockton-bsp is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with stockton-bsp.  If not, see <http://www.gnu.org/licenses/>.

const QUOTE: u8 = b'"';
const END_BRACKET: u8 = b'}';
const START_BRACKET: u8 = b'{';

use std::collections::HashMap;
use std::str;

use crate::types::Result;

#[derive(Debug, Clone)]
/// Game-related map information
pub struct EntitiesLump {
    /// The extracted entity data
    pub entities: Vec<Entity>,
}

#[derive(Debug, Clone, PartialEq)]
/// A game entity
pub struct Entity {
    pub attributes: HashMap<String, String>,
}

/// Internal enum to parse through the entities string.
#[derive(PartialEq, Eq)]
enum ParseState {
    InKey,
    InValue,
    AfterKey,
    InsideEntity,
    OutsideEntity,
}

impl EntitiesLump {
    /// Parse the given lump as an Entities Lump.
    pub fn from_lump(lump: &[u8]) -> Result<EntitiesLump> {
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
                    }
                    InKey => {
                        state = ParseState::AfterKey;
                        key_end = i;
                    }
                    AfterKey => {
                        state = ParseState::InValue;
                        val_start = i + 1;
                    }
                    InValue => {
                        state = ParseState::InsideEntity;
                        val_end = i;

                        attrs.insert(string[key_start..key_end].to_owned(), string[val_start..val_end].to_owned());
                    }
                    _ => {
                        return Err(invalid_error!("Entity definition is malformed"));
                    }
                },
                END_BRACKET => {
                    if state != InsideEntity {
                        return Err(invalid_error!("Entity definition is malformed"));
                    }

                    state = OutsideEntity;

                    entities.push(Entity { attributes: attrs });
                    attrs = HashMap::new();
                }
                START_BRACKET => {
                    if state != OutsideEntity {
                        return Err(invalid_error!("Entity definition is malformed"));
                    }
                    state = InsideEntity;
                }
                _ => {}
            }
        }
        Ok(EntitiesLump { entities })
    }
}