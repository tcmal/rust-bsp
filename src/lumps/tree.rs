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

//! Parses the BSP tree into a usable format

use crate::types::{Error, Result, IVector3};
use crate::lumps::helpers::slice_to_i32;

const NODE_SIZE: usize = 4 + (4 * 2) + (4 * 3) + (4 * 3);
const LEAF_SIZE: usize = 4 * 6 + (4 * 3 * 2);

/// Represents a BSP / binary tree.
#[derive(Debug, Clone)]
pub struct BSPTree {
    /// The root of this tree, first in the nodes lump for q3 files.
    pub root: BSPNode
}

impl BSPTree {
    /// Parses the nodes & leaves lumps into a usable BSP tree.
    pub fn from_lumps(nodes: &[u8], leaves: &[u8]) -> Result<'static, BSPTree> {
        if nodes.len() % NODE_SIZE != 0 || leaves.len() % LEAF_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        Ok(BSPTree { root: BSPTree::compile_node(0, nodes, leaves) })
    }

    /// Internal function. Visits given node and all its children. Used to recursively build tree.
    fn compile_node(i: i32, nodes: &[u8], leaves: &[u8]) -> BSPNode {
        if i < 0 {
            // Leaf.
            let i = i.abs() - 1;
            
            let raw = &leaves[i as usize * LEAF_SIZE..(i as usize * LEAF_SIZE) + LEAF_SIZE];

            let leaf = BSPLeaf {
                cluster: slice_to_i32(&raw[0..4]),
                area: slice_to_i32(&raw[4..8]),
                // 8..20 = min
                // 20..32 = max
                face: slice_to_i32(&raw[32..36]),
                n_faces: slice_to_i32(&raw[36..40]),
                brush: slice_to_i32(&raw[40..44]),
                n_brushes: slice_to_i32(&raw[44..48]),
            };

            BSPNode {
                children: None,
                min: IVector3::from_slice(&raw[8..20]),
                max: IVector3::from_slice(&raw[20..32]),
                leaf: Some(leaf)
            }
        } else {
            // Node.
            let raw = &nodes[i as usize * NODE_SIZE..(i as usize * NODE_SIZE) + NODE_SIZE];

            // 0..4 = i
            let child_one = BSPTree::compile_node(slice_to_i32(&raw[4..8]), nodes, leaves);
            let child_two = BSPTree::compile_node(slice_to_i32(&raw[8..12]), nodes, leaves);
            let min = IVector3::from_slice(&raw[12..24]);
            let max = IVector3::from_slice(&raw[24..36]);

            BSPNode {
                children: Some(Box::new([child_one, child_two])),
                min, max,
                leaf: None
            }
        }
    }
}

/// A node in a BSP tree.
/// Either has two children *or* a leaf entry.
#[derive(Debug, Clone)]
pub struct BSPNode {
    pub children: Option<Box<[BSPNode; 2]>>,
    pub min: IVector3,
    pub max: IVector3,
    pub leaf: Option<BSPLeaf>
}

/// A leaf in a BSP tree.
/// Will be under a `BSPNode`, min and max values are stored there.
#[derive(Debug, Clone, Copy)]
pub struct BSPLeaf {
    pub cluster: i32,
    pub area: i32,
    pub face: i32,
    pub n_faces: i32,
    pub brush: i32,
    pub n_brushes: i32
}