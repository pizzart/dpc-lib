use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{HasReferences, ResourceObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct SurfaceDatasZ {
    one: u32,
}

impl HasReferences for SurfaceDatasZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type SurfaceDatasObjectFormat = WALLEObjectFormat<ResourceObjectZ, SurfaceDatasZ>;
