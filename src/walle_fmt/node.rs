use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{
    Color, FixedVec, HasReferences, Mat4f, Quat, Rect, ResourceObjectZ, SphereZ, Vec3f,
    WALLEObjectFormat,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct NodeZ {
    parent_crc32: u32,
    head_child_crc32: u32,
    prev_node_crc32: u32,
    next_node_crc32: u32,
    object_crc32: u32,
    user_define_crc32: u32,
    light_data_crc32: u32,
    bitmap_crc32: u32,
    unknown_crc32: u32,
    inverse_world_transform: Mat4f,
    unknown1: Vec3f,
    collide_seads_id0: u32,
    unknown2: Vec3f,
    placeholder_world_matrix_ptr: u32,
    unknown3: Vec3f,
    display_seads_id0: u32,
    unknown4: Mat4f,
    translation: Vec3f,
    flags: u32,
    rotation: Quat,
    scale: f32,
    other_scale: f32,
    one_over_scale: f32,
    unknown5: f32,
    color: Color,
    sphere: SphereZ,
    display_seads_rect: Rect,
    collide_seads_rect: Rect,
    world_transform: Mat4f,
    collide_seads_id1: u32,
    display_seads_id1: u32,
    unknown6: i16,
    unknown7: u32,
    unknown8: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct NodeZAlt {
    parent_crc32: u32,
    some_node_crc320: u32,
    some_node_crc321: u32,
    some_node_crc322: u32,
    some_crc320: u32,
    some_crc321: u32,
    some_crc322: u32,
    some_crc323: u32,
    some_crc324: u32,
    mat0: Mat4f,
    unknown0s: FixedVec<u8, 208>,
    mat1: Mat4f,
    unknown2: u32,
    unknown3: u32,
    unknown4: u16,
    unknown5: u32,
    unknown6: u32,
}

impl HasReferences for NodeZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        if self.parent_crc32 != 0 {
            v.push(self.parent_crc32)
        }
        if self.head_child_crc32 != 0 {
            v.push(self.head_child_crc32)
        }
        if self.prev_node_crc32 != 0 {
            v.push(self.prev_node_crc32)
        }
        if self.next_node_crc32 != 0 {
            v.push(self.next_node_crc32)
        }
        if self.object_crc32 != 0 {
            v.push(self.object_crc32)
        }
        if self.user_define_crc32 != 0 {
            v.push(self.user_define_crc32)
        }
        if self.light_data_crc32 != 0 {
            v.push(self.light_data_crc32)
        }
        if self.bitmap_crc32 != 0 {
            v.push(self.bitmap_crc32)
        }
        if self.unknown_crc32 != 0 {
            v.push(self.unknown_crc32)
        }
        v
    }
}

impl HasReferences for NodeZAlt {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type NodeObjectFormat = WALLEObjectFormat<ResourceObjectZ, NodeZ>;
pub type NodeObjectFormatAlt = WALLEObjectFormat<ResourceObjectZ, NodeZAlt>;
