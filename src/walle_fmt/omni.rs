use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{FixedVec, HasReferences, ObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct OmniZ {
    data: FixedVec<u32, 48>,
    crc32s: FixedVec<u32, 2>,
}

impl HasReferences for OmniZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub type OmniObjectFormat = WALLEObjectFormat<ObjectZ, OmniZ>;
