use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};

use crate::walle_fmt::common::{HasReferences, PascalArray, ResourceObjectZ, WALLEObjectFormat};

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct GameObjZ {
    node_crc32s: PascalArray<u32>,
}

impl HasReferences for GameObjZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        for node in self.node_crc32s.data.iter() {
            v.push(node.to_owned());
        }
        v
    }
}

pub type GameObjObjectFormat = WALLEObjectFormat<ResourceObjectZ, GameObjZ>;
