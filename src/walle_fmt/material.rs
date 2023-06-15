use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{
    write_option, FixedVec, HasReferences, ResourceObjectZ, Vec3f, Vec4f, WALLEObjectFormat,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialZ {
    color: Vec4f,
    emission: Vec3f,
    unknown0: u32,
    uv_transform_matrix: FixedVec<f32, 9>,
    unknown0s: FixedVec<f32, 8>,
    unknown1s: FixedVec<u32, 3>,
    diffuse_translation: FixedVec<f32, 2>,
    diffuse_scale: FixedVec<f32, 2>,
    diffuse_rotation: f32,
    flags: FixedVec<u32, 3>,
    texture_flag: u8,
    diffuse_bitmap_crc32: u32,
    #[nom(Cond(texture_flag == 1))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown2s: Option<FixedVec<u32, 7>>,
    #[nom(Cond(texture_flag == 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown3: Option<u32>,
    #[nom(Cond(texture_flag == 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown4s: Option<FixedVec<u32, 6>>,
    #[nom(Cond(texture_flag != 1 && texture_flag != 3))]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[binwrite(with(write_option))]
    unknown5s: Option<FixedVec<u32, 3>>,
}

impl HasReferences for MaterialZ {
    fn hard_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        if self.diffuse_bitmap_crc32 != 0 {
            v.push(self.diffuse_bitmap_crc32)
        }
        v
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type MaterialObjectFormat = WALLEObjectFormat<ResourceObjectZ, MaterialZ>;
