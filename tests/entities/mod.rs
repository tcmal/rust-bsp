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

use stockton_bsp::lumps::entities::{EntitiesLump, Entity};

macro_rules! map(
    { $($key:expr => $value:expr),+ } => {
        {
            let mut m = ::std::collections::HashMap::new();
            $(
                m.insert(($key).to_string(), ($value).to_string());
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

    assert_eq!(
        valid.entities,
        vec![
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
        ]
    );
}
