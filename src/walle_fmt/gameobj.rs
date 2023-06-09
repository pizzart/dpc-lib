use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{
    HasReferences, PascalArray, PascalStringNULL, ResourceObjectZ, WALLEObjectFormat,
};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct GameObjZChild {
    string: PascalStringNULL,
    is_in_world: u32,
    crc32s: PascalArray<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct GameObjZ {
    children: PascalArray<GameObjZChild>,
}

impl HasReferences for GameObjZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        for child in self.children.data.iter() {
            v.append(&mut child.crc32s.data.clone());
        }
        v
    }
}

pub type GameObjObjectFormat = WALLEObjectFormat<ResourceObjectZ, GameObjZ>;
