use nom_derive::{NomLE, Parse};
use serde::{Serialize, Deserialize};
use std::path::Path;
use crate::File;
use std::io::Result;
use std::io::Write;
use nom::number::complete::*;

use crate::fuel_fmt::common::{ObjectZ, Vec3f};

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown0 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown1 {
    unknown0: u32,
    unknown1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown2 {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<u16>,
    unknown1: u32,
    unknown2: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown4 {
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<MeshZUnknown1>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown6 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown7 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZSubMesh {
    vertex_count: u32,
    vertex_size: u32,
    vertex_group_crc32: u32,
    #[nom(Count(vertex_count as usize * vertex_size as usize))]
    data: Vec<u8>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZIndices
{
    index_count: u32,
    vertex_group_crc32: u32,
    #[nom(Count(index_count))]
    data: Vec<u16>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown11 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
    unknown8: u32,
    unknown9: u32,
    unknown10: u32,
    unknown11: u32,
    unknown12: u32,
    unknown13: u32,
    unknown14: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown13Unknown1
{
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown13 {
    #[nom(Count(12))]
    unknown0s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<MeshZUnknown13Unknown1>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown16 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
    unknown4: u32,
    unknown5: u32,
    unknown6: u32,
    unknown7: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZPair {
    first: u16,
    second: u16,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown15 {
    unknown0: u32,
    unknown1: u32,
    unknown2: u32,
    unknown3: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown14 {
    #[nom(LengthCount(le_u32))]
    name: Vec<u8>,
    unknown1: u32,
    unknown2: u16,
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<u16>,
    #[nom(LengthCount(le_u32))]
    unknown15s: Vec<MeshZUnknown15>,
}

#[derive(Serialize, Deserialize, NomLE)]
struct MeshZUnknown12 {
    u0: u16,
    u1: u32,
}

#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
struct MeshZ {
    #[nom(LengthCount(le_u32))]
    vecs: Vec<Vec3f>,
    #[nom(LengthCount(le_u32))]
    unknown0s: Vec<MeshZUnknown0>,
    #[nom(LengthCount(le_u32))]
    unknown1s: Vec<MeshZUnknown1>,
    #[nom(LengthCount(le_u32))]
    vertices1: Vec<Vec3f>,
    #[nom(LengthCount(le_u32))]
    unknown2s: Vec<MeshZUnknown2>,
    // if (someHeaderValue)
    // {
    //     PascalArray<std::uint32_t> unknown3s;
    // }
    #[nom(LengthCount(le_u32))]
    unknown4s: Vec<MeshZUnknown4>,
    #[nom(LengthCount(le_u32))]
    material_crc32s: Vec<u32>,
    #[nom(LengthCount(le_u32))]
    unknown6s: Vec<MeshZUnknown6>,
    #[nom(LengthCount(le_u32))]
    unknown7s: Vec<MeshZUnknown7>,
    #[nom(LengthCount(le_u32))]
    unknown8s: Vec<MeshZUnknown6>,
    #[nom(LengthCount(le_u32))]
    sub_meshes: Vec<MeshZSubMesh>,
    #[nom(LengthCount(le_u32))]
    indices: Vec<MeshZIndices>,
    #[nom(LengthCount(le_u32))]
    unknown11s: Vec<MeshZUnknown11>,
    #[nom(LengthCount(le_u32))]
    unknown13s: Vec<MeshZUnknown13>,
    #[nom(LengthCount(le_u32))]
    unknown16s: Vec<MeshZUnknown16>,
    #[nom(LengthCount(le_u32))]
    pairs: Vec<MeshZPair>,
    #[nom(LengthCount(le_u32))]
    unknown18s: Vec<u16>,
    #[nom(LengthCount(le_u32))]
    unknown14s: Vec<MeshZUnknown14>,
    #[nom(LengthCount(le_u32))]
    unknown12s: Vec<MeshZUnknown12>,
}

#[derive(Serialize, Deserialize)]
struct MeshObject {
    object: ObjectZ,
    mesh: MeshZ,
}

pub fn fuel_fmt_extract_mesh_z(header: &[u8], data: &[u8], output_path: &Path) -> Result<()> {
    let json_path = output_path.join("object.json");
    let mut output_file = File::create(json_path)?;

    let object = match ObjectZ::parse(&header) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let mesh = match MeshZ::parse(&data) {
        Ok((_, h)) => h,
        Err(error) => panic!("{}", error),
    };

    let object = MeshObject {
        object,
        mesh,
    };

    output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

    Ok(())
}
