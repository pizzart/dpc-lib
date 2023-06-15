use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{Color, HasReferences, ResourceObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshDataZ {
    unknown0: f32,
    color: Color,
}

impl HasReferences for MeshDataZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type MeshDataObjectFormat = WALLEObjectFormat<ResourceObjectZ, MeshDataZ>;
