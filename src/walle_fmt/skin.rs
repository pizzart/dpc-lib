use std::{
    io::Error,
    io::{ErrorKind, Write},
    path::Path,
};

use binwrite::BinWrite;
use nom_derive::*;
use serde::{Deserialize, Serialize};
use std::fs::File;

use crate::walle_fmt::common::{
    write_option, FixedVec, HasReferences, Mat4f, ObjectZ, PascalArray, SphereZ,
    WALLEObjectFormatTrait,
};

static mut SKIN_DATA_COUNT: u32 = 0;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct DynArrayZ {
    size_capacity: u32,
    ptr: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct HashElt {
    value: i32,
    vref: i32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct BlendUnknown0 {
    unknown0: u32,
    unknown1: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct ObjectBlend {
    unknown0: u16,
    unknown1s: PascalArray<BlendUnknown0>,
    unknown2s: PascalArray<BlendUnknown0>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct BoneZ {
    bone_name_crc32: u32,
    // object_blend_count: u32,
    unknown0s: PascalArray<ObjectBlend>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct SkinZSkinSubsection {
    material_link_crc32: u32,
    bone_names_crc32: FixedVec<u32, 7>,
    placeholder_morph_packet_da: DynArrayZ,
    morph_packets: PascalArray<FixedVec<u32, 2>>,
    // vertex_group_crc32: u32,
    // unknown_crc320: u32,
    // unknown_crc321: u32,
    // unknown_crc322: u32,
    // #[nom(Count(unsafe { SKIN_DATA_COUNT as usize }))]
    // data: Vec<u32>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
pub struct SkinZ {
    mesh_crc32s: PascalArray<u32>,
    unknown0s: PascalArray<FixedVec<u8, 8>>,
    bones: PascalArray<BoneZ>,
    is_class_id: u8,
    // #[nom(Cond(is_class_id == 1))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[binwrite(with(write_option))]
    // anim_class_ids: Option<PascalArray<HashElt>>,
    // #[nom(Cond(is_class_id == 1))]
    // #[serde(skip_serializing_if = "Option::is_none")]
    // #[binwrite(with(write_option))]
    // sound_class_ids: Option<PascalArray<HashElt>>,
    matrix_cache_check: u32,
    skin_sections: PascalArray<PascalArray<SkinZSkinSubsection>>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
pub struct SkinZHeader {
    friendly_name_crc32: u32,
    crc32s: PascalArray<u8>,
    skel_crc32: u32,
    sphere_local: SphereZ,
    unknown0: Mat4f,
    fade_out_distance: f32,
    flags: u32,
    skin_type: u16,
}

impl HasReferences for SkinZ {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.append(&mut self.mesh_crc32s.data.clone());
        v
    }
}

impl HasReferences for SkinZHeader {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        let mut v = Vec::new();
        v.push(self.skel_crc32);
        v
    }
}

#[derive(Serialize, Deserialize)]
struct SkinObject {
    skin_header: SkinZHeader,
    skin: SkinZ,
}

impl HasReferences for SkinObject {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

pub struct SkinObjectFormat;

impl SkinObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl WALLEObjectFormatTrait for SkinObjectFormat {
    fn pack(
        self: &Self,
        input_path: &Path,
        header: &mut Vec<u8>,
        body: &mut Vec<u8>,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let mut object: SkinObject = serde_json::from_reader(json_file)?;

        object.skin_header.write(header)?;
        object.skin.write(body)?;

        Ok((
            object.skin_header.hard_links(),
            object.skin_header.soft_links(),
        ))
    }

    fn unpack(
        self: &Self,
        header: &[u8],
        body: &[u8],
        output_path: &Path,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = output_path.join("object.json");
        let mut output_file = File::create(json_path)?;

        let skin_header = match SkinZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(e) => {
                panic!(e.to_string());
                return Err(Error::from(ErrorKind::Other));
            }
        };

        let skin = match SkinZ::parse(body) {
            Ok((_, h)) => h,
            Err(e) => {
                panic!(e.to_string());
                return Err(Error::from(ErrorKind::Other));
            }
        };

        let object = SkinObject { skin_header, skin };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok((
            object.skin_header.hard_links(),
            object.skin_header.soft_links(),
        ))
    }
}
