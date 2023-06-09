use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{HasReferences, PascalArray, ResourceObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct MaterialObjZEntry {
    array_name_crc32: u32,
    material_anim_crc32s: PascalArray<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MaterialObjZ {
    entries: PascalArray<MaterialObjZEntry>,
}

impl HasReferences for MaterialObjZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type MaterialObjObjectFormat = WALLEObjectFormat<ResourceObjectZ, MaterialObjZ>;
