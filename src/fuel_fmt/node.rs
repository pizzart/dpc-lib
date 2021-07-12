use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;

use crate::fuel_fmt::common::{ResourceObjectZ, Mat4f};

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct NodeZ {
    parent_crc32: u32,
    some_node_crc320: u32,
    some_node_crc321: u32,
    some_node_crc322: u32,
    some_crc320: u32,
    some_crc321: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: f32,
    #[nom(Count(32))]
    unknown10s: Vec<u8>,
    mat0: Mat4f,
    #[nom(Count(17))]
    unknown11s: Vec<u16>,
    mat1: Mat4f,
}

#[derive(Serialize, Deserialize)]
struct NodeObject {
    resource_object: ResourceObjectZ,
    node: NodeZ,
}

pub fn fuel_fmt_extract_node_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let resource_object = match ResourceObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let node = match NodeZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = NodeObject {
        resource_object,
        node,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
