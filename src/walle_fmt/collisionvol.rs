use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{FixedVec, HasReferences, Mat4f, ObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct CollisionVolZ {
    unknown0: u32,
    local_transform: Mat4f,
    local_transform_inverse: Mat4f,
    zeros: FixedVec<u32, 28>,
    volume_type: u32,
    unknown1: u32,
}

impl HasReferences for CollisionVolZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type CollisionVolObjectType = WALLEObjectFormat<ObjectZ, CollisionVolZ>;
