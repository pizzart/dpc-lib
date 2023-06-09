use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{HasReferences, ResourceObjectZ, Vec3f, Vec3i32, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct LightDataZ {
    unknown0: u32,
    color: Vec3f,
    unknown1: Vec3f,
    unknown2: Vec3i32,
    unknown_flag: u32,
    unknown3: Vec3f,
}

impl HasReferences for LightDataZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type LightDataObjectFormat = WALLEObjectFormat<ResourceObjectZ, LightDataZ>;
