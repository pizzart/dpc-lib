use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{HasReferences, ObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct CameraZ {
    angle_of_view: f32,
    zero: f32,
    node_crc32: u32,
}

impl HasReferences for CameraZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type CameraObjectFormat = WALLEObjectFormat<ObjectZ, CameraZ>;
