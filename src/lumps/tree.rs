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

use crate::types::{Error, Result, IVector3, TransparentNonNull};
use super::faces::{Face, FaceLump};
use super::brushes::{Brush, BrushesLump};
use crate::lumps::helpers::slice_to_i32;

const NODE_SIZE: usize = 4 + (4 * 2) + (4 * 3) + (4 * 3);
const LEAF_SIZE: usize = 4 * 6 + (4 * 3 * 2);

/// Represents a BSP / binary tree.
#[derive(Debug, Clone)]
pub struct BSPTree<'a> {
    /// The root of this tree, first in the nodes lump for q3 files.
    pub root: BSPNode<'a>
}

impl<'a> BSPTree<'a> {
    /// Parses the nodes & leaves lumps into a usable BSP tree.
    pub fn from_lumps(nodes: &[u8], leaves: &[u8], leaf_faces: &[u8], leaf_brushes: &[u8], faces: &FaceLump<'a>, brushes: &BrushesLump<'a>) -> Result<'a, BSPTree<'a>> {
        if nodes.len() % NODE_SIZE != 0 || leaves.len() % LEAF_SIZE != 0 {
            return Err(Error::BadFormat);
        }

        Ok(BSPTree { root: BSPTree::compile_node(0, nodes, leaves, leaf_faces, leaf_brushes, faces, brushes)? })
    }

    /// Internal function. Visits given node and all its children. Used to recursively build tree.
    fn compile_node(i: i32, nodes: &[u8], leaves: &[u8], leaf_faces: &[u8], leaf_brushes: &[u8], faces_lump: &FaceLump<'a>, brushes_lump: &BrushesLump<'a>) -> Result<'a, BSPNode<'a>> {
        if i < 0 {
            // Leaf.
            let i = i.abs() - 1;
            
            let raw = &leaves[i as usize * LEAF_SIZE..(i as usize * LEAF_SIZE) + LEAF_SIZE];

            let start = slice_to_i32(&raw[32..36]) as usize;
            let n = slice_to_i32(&raw[36..40]) as usize;
            let mut faces = Vec::with_capacity(n);
            if n > 0 {
                if start + n > leaf_faces.len() / 4 {
                    return Err(Error::BadRef { loc: "Tree.Leaf.LeafFaces", val: start + n })
                }

                for i in start..start+n {
                    let face_index = slice_to_i32(&leaf_faces[i * 4..(i+1) * 4]) as usize;
                    if face_index >= faces_lump.faces.len() {
                        return Err(Error::BadRef { loc: "Tree.LeafFace", val: face_index })
                    }
                    faces.push((&faces_lump.faces[face_index]).into());
                }
            }

            let faces = faces.into_boxed_slice();

            let start = slice_to_i32(&raw[40..44]) as usize;
            let n =  slice_to_i32(&raw[44..48]) as usize;
            let mut brushes =  Vec::with_capacity(n);
            if n > 0 {
                if start + n > leaf_brushes.len() / 4 {
                    return Err(Error::BadRef { loc: "Tree.Leaf.LeafBrushes", val: start + n })
                }

                for i in start..start+n {
                    let brush_index = slice_to_i32(&leaf_brushes[i * 4..(i+1) * 4]) as usize;
                    if brush_index >= brushes_lump.brushes.len() {
                        return Err(Error::BadRef { loc: "Tree.LeafBrushes", val: brush_index })
                    }
                    brushes.push((&brushes_lump.brushes[brush_index]).into());
                }
            }

            let brushes = brushes.into_boxed_slice();

            let leaf = BSPLeaf {
                cluster_id: slice_to_i32(&raw[0..4]),
                area: slice_to_i32(&raw[4..8]),
                // 8..20 = min
                // 20..32 = max
                faces, brushes
            };

            Ok(BSPNode {
                children: None,
                min: IVector3::from_slice(&raw[8..20]),
                max: IVector3::from_slice(&raw[20..32]),
                leaf: Some(leaf)
            })
        } else {
            // Node.
            let raw = &nodes[i as usize * NODE_SIZE..(i as usize * NODE_SIZE) + NODE_SIZE];

            // 0..4 = i
            let child_one = BSPTree::compile_node(slice_to_i32(&raw[4..8]), nodes, leaves, leaf_faces, leaf_brushes, faces_lump, brushes_lump)?;
            let child_two = BSPTree::compile_node(slice_to_i32(&raw[8..12]), nodes, leaves, leaf_faces, leaf_brushes, faces_lump, brushes_lump)?;
            let min = IVector3::from_slice(&raw[12..24]);
            let max = IVector3::from_slice(&raw[24..36]);

            Ok(BSPNode {
                children: Some(Box::new([child_one, child_two])),
                min, max,
                leaf: None
            })
        }
    }

    pub fn empty() -> BSPTree<'static> {
        BSPTree {
            root: BSPNode {
                children: None,
                min: IVector3::zero(),
                max: IVector3::zero(),
                leaf: None
            }
        }
    }
}

/// A node in a BSP tree.
/// Either has two children *or* a leaf entry.
#[derive(Debug, Clone)]
pub struct BSPNode<'a> {
    pub children: Option<Box<[BSPNode<'a>; 2]>>,
    pub min: IVector3,
    pub max: IVector3,
    pub leaf: Option<BSPLeaf<'a>>
}

/// A leaf in a BSP tree.
/// Will be under a `BSPNode`, min and max values are stored there.
#[derive(Debug, Clone)]
pub struct BSPLeaf<'a> {
    pub cluster_id: i32,
    pub area: i32,
    pub faces: Box<[TransparentNonNull<Face<'a>>]>,
    pub brushes: Box<[TransparentNonNull<Brush<'a>>]>
}