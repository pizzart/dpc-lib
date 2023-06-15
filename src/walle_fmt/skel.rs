use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{
    FixedVec, HasReferences, Mat4f, PascalArray, Quat, ResourceObjectZ, SphereZ, Vec3f, Vec3i32,
    WALLEObjectFormat,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SkelZBone {
    user_define_crc32: u32,
    local_rotation: Quat,
    scale: Vec3f,
    bone_flags: u32,
    local_translation: Vec3f,
    placeholder_child_ptr: u32,
    model_rot_matrix_row1: Vec3f,
    model_matrix_id: i16,
    inverse_model_matrix_id: i16,
    model_rot_matrix_row2: Vec3f,
    placeholder_model_matrix_ptr: u32,
    model_rot_matrix_row3: Vec3f,
    placeholder_inverse_model_matrix_ptr: u32,
    local_rotation_inverse: Quat,
    unknown_ptr0s: FixedVec<u32, 3>,
    placeholder_parent_ptr: u32,
    unknown_ptr1s: FixedVec<u32, 3>,
    placeholder_prev_sibling_ptr: u32,
    unknown_ptr2s: FixedVec<u32, 3>,
    placeholder_next_sibling_ptr: u32,
    original_model_transform: Mat4f,
    child_index: i32,
    parent_index: i32,
    next_sibling_index: i32,
    prev_sibling_index: i32,
    bone_name: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct SphereColBone {
    sphere: SphereZ,
    flag: u32,
    name_crc32: u32,
    bone_node_crc32: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct BoxColBone {
    mat: Mat4f,
    flag: u32,
    name_crc32: u32,
    bone_node_crc32: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact, Debug)]
pub struct SkelZ {
    flag: u32,
    sphere_local: SphereZ,
    bones: PascalArray<SkelZBone>,
    material_crc32s: PascalArray<u32>,
    mesh_data_crc32s: PascalArray<u32>,
    bone_node_groups: PascalArray<PascalArray<u32>>,
    unknown0s: PascalArray<u32>,
    sphere_col_bones1: PascalArray<SphereColBone>,
    sphere_col_bones2: PascalArray<SphereColBone>,
    box_col_bones: PascalArray<BoxColBone>,
}

impl HasReferences for SkelZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.append(
            &mut self
                .bones
                .data
                .iter()
                .map(|x| x.user_define_crc32)
                .filter(|x| *x != 0)
                .collect(),
        );
        v.append(&mut self.material_crc32s.data.clone());
        v.append(&mut self.mesh_data_crc32s.data.clone());
        v
    }
}

pub type SkelObjectFormat = WALLEObjectFormat<ResourceObjectZ, SkelZ>;
