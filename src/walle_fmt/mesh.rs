use binwrite::{BinWrite, WriterOption};
use nom::multi::count;
use nom::IResult;
use nom_derive::*;
use serde::{Deserialize, Serialize};
use std::convert::TryInto;
// use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{Error, ErrorKind, Write};
// use std::iter::Map;
use std::path::Path;
// use std::sync::Arc;
// use three_d_asset::prelude;

// use crate::base_dpc::DPC;
// use crate::base_dpc::{Options, DPC};
// use crate::walle_dpc::WALLEDPC;
use crate::walle_fmt::common::{
    FixedVec, HasReferences, Mat4f, PascalArray, Quat, RangeBeginEnd, RangeBeginSize, Vec2f, Vec3,
    Vec3f, VertexVectorComponent, WALLEObjectFormatTrait,
};
// use crate::walle_fmt::material::MaterialObject;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Strip {
    strip_vertices_indices: PascalArray<u16>,
    material_name: u32,
    tri_order: u32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct CollisionAABB {
    min: Vec3f,
    collision_aabb_range: RangeBeginEnd,
    max: Vec3f,
    collision_faces_range: RangeBeginSize,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct CollisionFace {
    short_vec_weirds_indices: FixedVec<u16, 3>,
    surface_type: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayoutPositionUV {
    position: Vec3f,
    unknown0: f32,
    uv: Vec2f,
}

type VertexVector3u8 = Vec3<VertexVectorComponent>;

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayoutNoBlend {
    position: Vec3f,
    tangent: VertexVector3u8,
    pad0: u8,
    normal: VertexVector3u8,
    pad1: u8,
    uv: Vec2f,
    luv: Vec2f,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexBlendIndex {
    index: f32,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayout1Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    pad0: u8,
    normal: VertexVector3u8,
    pad1: u8,
    uv: Vec2f,
    unknown0: FixedVec<f32, 5>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct VertexLayout4Blend {
    position: Vec3f,
    tangent: VertexVector3u8,
    pad0: u8,
    normal: VertexVector3u8,
    pad1: u8,
    uv: Vec2f,
    blend_indices: FixedVec<VertexBlendIndex, 4>,
    blend_weights: FixedVec<f32, 4>,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum VertexBufferData {
    VertexLayout4BlendCase(Vec<VertexLayout4Blend>),
    VertexLayout1BlendCase(Vec<VertexLayout1Blend>),
    VertexLayoutNoBlendCase(Vec<VertexLayoutNoBlend>),
    VertexLayoutPositionUVCase(Vec<VertexLayoutPositionUV>),
}

impl VertexBufferData {
    fn parse(i: &[u8], vertex_size: u32, vertex_count: usize) -> IResult<&[u8], VertexBufferData> {
        match vertex_size {
            60 => {
                let parse_result = count(VertexLayout4Blend::parse, vertex_count)(i)?;
                Ok((
                    parse_result.0,
                    VertexBufferData::VertexLayout4BlendCase(parse_result.1),
                ))
            }
            48 => {
                let parse_result = count(VertexLayout1Blend::parse, vertex_count)(i)?;
                Ok((
                    parse_result.0,
                    VertexBufferData::VertexLayout1BlendCase(parse_result.1),
                ))
            }
            36 => {
                let parse_result = count(VertexLayoutNoBlend::parse, vertex_count)(i)?;
                Ok((
                    parse_result.0,
                    VertexBufferData::VertexLayoutNoBlendCase(parse_result.1),
                ))
            }
            24 => {
                let parse_result = count(VertexLayoutPositionUV::parse, vertex_count)(i)?;
                Ok((
                    parse_result.0,
                    VertexBufferData::VertexLayoutPositionUVCase(parse_result.1),
                ))
            }
            _ => {
                panic!("Invalid vertex size {}", vertex_size)
            }
        }
    }
}

impl BinWrite for VertexBufferData {
    fn write<W: Write>(&self, writer: &mut W) -> std::io::Result<()> {
        match self {
            VertexBufferData::VertexLayout4BlendCase(data) => data.write(writer),
            VertexBufferData::VertexLayout1BlendCase(data) => data.write(writer),
            VertexBufferData::VertexLayoutNoBlendCase(data) => data.write(writer),
            VertexBufferData::VertexLayoutPositionUVCase(data) => data.write(writer),
        }
    }

    fn write_options<W: Write>(
        &self,
        writer: &mut W,
        options: &WriterOption,
    ) -> std::io::Result<()> {
        match self {
            VertexBufferData::VertexLayout4BlendCase(data) => data.write_options(writer, options),
            VertexBufferData::VertexLayout1BlendCase(data) => data.write_options(writer, options),
            VertexBufferData::VertexLayoutNoBlendCase(data) => data.write_options(writer, options),
            VertexBufferData::VertexLayoutPositionUVCase(data) => {
                data.write_options(writer, options)
            }
        }
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[serde(from = "VertexBufferExtShadow")]
#[nom(Debug)]
struct VertexBufferExt {
    #[serde(skip)]
    vertex_count: u32,
    #[serde(skip)]
    vertex_size: u32,
    vertex_buffer_id: u32,
    #[nom(Parse = "{ |i| VertexBufferData::parse(i, vertex_size, vertex_count as usize) }")]
    vertices: VertexBufferData,
}

#[derive(Deserialize)]
struct VertexBufferExtShadow {
    vertex_buffer_id: u32,
    vertices: VertexBufferData,
}

impl From<VertexBufferExtShadow> for VertexBufferExt {
    fn from(shadow: VertexBufferExtShadow) -> Self {
        let (vertex_count, vertex_size) = match &shadow.vertices {
            VertexBufferData::VertexLayout4BlendCase(data) => (data.len(), 60),
            VertexBufferData::VertexLayout1BlendCase(data) => (data.len(), 48),
            VertexBufferData::VertexLayoutNoBlendCase(data) => (data.len(), 36),
            VertexBufferData::VertexLayoutPositionUVCase(data) => (data.len(), 12),
        };
        VertexBufferExt {
            vertex_count: vertex_count as u32,
            vertex_size,
            vertex_buffer_id: shadow.vertex_buffer_id,
            vertices: shadow.vertices,
        }
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
// #[nom(DebugDerive)]
// #[serde(from = "IndexBufferExtShadow")]
struct IndexBufferExt {
    // #[serde(skip)]
    index_count: u32,
    index_buffer_id: u32,
    #[nom(Count(index_count))]
    indices: Vec<i16>,
}

#[derive(Deserialize)]
struct IndexBufferExtShadow {
    index_buffer_id: u32,
    indices: Vec<i16>,
}

impl From<IndexBufferExtShadow> for IndexBufferExt {
    fn from(shadow: IndexBufferExtShadow) -> Self {
        IndexBufferExt {
            index_count: shadow.indices.len() as u32,
            index_buffer_id: shadow.index_buffer_id,
            indices: shadow.indices,
        }
    }
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
struct Quad {
    vertices: FixedVec<Vec3f, 4>,
    normal: Vec3f,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Debug)]
struct VertexGroup {
    zeros: FixedVec<u32, 3>,
    maybe_primitive: u32,
    vertex_offset_in_buffer: u16,
    unknown0: u16,
    vertex_count: u32,
    index_buffer_offset: u32,
    face_count: u32,
    unknown1: u32,
    unknown2: u32,
    vertex_size: u16,
    cdcdcdcd: u16,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
// #[nom(Exact, Debug)]
pub struct MeshZ {
    unknown0: u32,
    unknown1: u32,
    morpher: PascalArray<u32>,
    unknown2s: FixedVec<u32, 4>,
    material_crc32s: PascalArray<u32>,
    unknown3s: FixedVec<u8, 24>,
    sphere_col_count: u32,
    box_col_count: u32,
    cylinder_col_count: u32,
    aabb_col_rel_count: u32,
    aabb_col_count: u32,
    vertices: PascalArray<Vec3<i16>>,
    unknown4: u32,
    vertex_buffers: PascalArray<VertexBufferExt>,
    indices: PascalArray<IndexBufferExt>,
    vertex_groups: PascalArray<VertexGroup>,
    pad0: FixedVec<u32, 4>,
}

#[derive(BinWrite)]
#[binwrite(little)]
#[derive(Serialize, Deserialize, NomLE)]
#[nom(Exact)]
pub struct MeshZHeader {
    friendly_name_crc32: u32,
    crc32s: PascalArray<u32>,
    mesh_data_crc32: u32,
    rot: Quat,
    transform: Mat4f,
    radius: f32,
    flags: u32,
    mesh_type: u16,
}

impl HasReferences for MeshZHeader {
    fn hard_links(&self) -> Vec<u32> {
        vec![]
    }

    fn soft_links(&self) -> Vec<u32> {
        self.crc32s.data.clone()
    }
}

impl HasReferences for MeshZ {
    fn hard_links(&self) -> Vec<u32> {
        self.material_crc32s.data.clone()
    }

    fn soft_links(&self) -> Vec<u32> {
        vec![]
    }
}

// pub type MeshObjectFormat = WALLEObjectFormat<MeshZHeader, MeshZ>;

#[derive(Serialize, Deserialize)]
struct MeshObject {
    mesh_header: MeshZHeader,
    mesh: MeshZ,
}

pub struct MeshObjectFormat;

impl MeshObjectFormat {
    pub fn new<'a>() -> &'a Self {
        &Self {}
    }
}

impl WALLEObjectFormatTrait for MeshObjectFormat {
    fn pack(
        self: &Self,
        input_path: &Path,
        header: &mut Vec<u8>,
        body: &mut Vec<u8>,
    ) -> Result<(Vec<u32>, Vec<u32>), Error> {
        let json_path = input_path.join("object.json");
        let json_file = File::open(json_path)?;

        let object: MeshObject = serde_json::from_reader(json_file)?;

        object.mesh_header.write(header)?;
        object.mesh.write(body)?;

        Ok((
            object.mesh_header.hard_links(),
            object.mesh_header.soft_links(),
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

        let mesh_header = match MeshZHeader::parse(&header) {
            Ok((_, h)) => h,
            Err(_) => return Err(Error::from(ErrorKind::Other)),
        };

        let mesh = match MeshZ::parse(body) {
            Ok((_, h)) => h,
            Err(_) => return Err(Error::from(ErrorKind::Other)),
        };

        // struct GLTFScene {
        //     nodes: Vec<usize>,
        // }

        // struct GLTFNode {
        //     mesh: usize,
        // }

        // struct GLTFPrimitive {
        //     attributes: Map<String, f32>,
        //     indices: usize,
        // }

        // struct GLTFMesh {
        //     primitives: Vec<GLTFPrimitive>,
        // }

        // struct GLTFAsset {
        //     version: f32,
        // }

        // struct GLTFFile {
        //     scene: usize,
        //     scenes: Vec<GLTFScene>,
        //     nodes: Vec<GLTFNode>,
        //     meshes: Vec<GLTFMesh>,
        //     asset: GLTFAsset,
        // }

        // pb.println(format!("Extracting {}", oh.crc32));

        let mut vertex_positions: Vec<[f32; 3]> = Vec::new();
        // let mut indices: Vec<u16> = Vec::new();
        let mut vertex_uvs: Vec<[f32; 2]> = Vec::new();
        let mut vertex_normals: Vec<[f32; 3]> = Vec::new();
        let mut polygons: Vec<obj::SimplePolygon> = Vec::new();

        for vbuffer in &mesh.vertex_buffers.data {
            match &vbuffer.vertices {
                VertexBufferData::VertexLayout4BlendCase(buffer_vertices) => {
                    for vertex in buffer_vertices {
                        let position: [f32; 3] = vertex.position.data.clone().try_into().unwrap();
                        let uv: [f32; 2] = vertex.uv.data.clone().try_into().unwrap();
                        let mut normal_vec: Vec<f32> = vec![];
                        for component in &vertex.normal.data {
                            normal_vec.push(-((component.data.clone() as f32 / 255.0) * 2.0 - 1.0));
                        }
                        let normal: [f32; 3] = normal_vec.try_into().unwrap();
                        vertex_positions.push(position);
                        vertex_uvs.push(uv);
                        vertex_normals.push(normal);
                    }
                }
                VertexBufferData::VertexLayout1BlendCase(buffer_vertices) => {
                    for vertex in buffer_vertices {
                        let position: [f32; 3] = vertex.position.data.clone().try_into().unwrap();
                        let uv: [f32; 2] = vertex.uv.data.clone().try_into().unwrap();
                        let mut normal_vec: Vec<f32> = vec![];
                        for component in &vertex.normal.data {
                            normal_vec.push(-((component.data.clone() as f32 / 255.0) * 2.0 - 1.0));
                        }
                        let normal: [f32; 3] = normal_vec.try_into().unwrap();
                        vertex_positions.push(position);
                        vertex_uvs.push(uv);
                        vertex_normals.push(normal);
                    }
                }
                VertexBufferData::VertexLayoutNoBlendCase(buffer_vertices) => {
                    for vertex in buffer_vertices {
                        let position: [f32; 3] = vertex.position.data.clone().try_into().unwrap();
                        let uv: [f32; 2] = vertex.uv.data.clone().try_into().unwrap();
                        let mut normal_vec: Vec<f32> = vec![];
                        for component in &vertex.normal.data {
                            normal_vec.push(-((component.data.clone() as f32 / 255.0) * 2.0 - 1.0));
                        }
                        let normal: [f32; 3] = normal_vec.try_into().unwrap();
                        vertex_positions.push(position);
                        vertex_uvs.push(uv);
                        vertex_normals.push(normal);
                    }
                }
                VertexBufferData::VertexLayoutPositionUVCase(buffer_vertices) => {
                    for vertex in buffer_vertices {
                        let position: [f32; 3] = vertex.position.data.clone().try_into().unwrap();
                        let uv: [f32; 2] = vertex.uv.data.clone().try_into().unwrap();
                        vertex_positions.push(position);
                        vertex_uvs.push(uv);
                        vertex_normals.push([0.0; 3]);
                    }
                }
            }
        }
        for ibuffer in &mesh.indices.data {
            for triangle in ibuffer.indices.chunks(3).map(|x| x.to_vec()) {
                let mut tuple: Vec<obj::IndexTuple> = Vec::new();
                for index in triangle {
                    tuple.push(obj::IndexTuple(index as usize, None, Some(index as usize)))
                }
                polygons.push(obj::SimplePolygon(tuple));
                // indices.push(*index as u16);
            }
        }

        // let mut materials: Vec<obj::Material> = vec![];
        // for material_crc32 in &mesh.material_crc32s.data {
        //     let dir = output_path.parent().unwrap();
        //     let mtl_input_path = dir.join(format!("{}.Material_Z", &material_crc32));
        //     let mut mtl_output_path = OsString::new();
        //     mtl_output_path.push(mtl_input_path.as_os_str());
        //     mtl_output_path.push(".d");
        //     let mtl_json_path = Path::new(&mtl_output_path).join("object.json");
        //     if !mtl_json_path.try_exists().unwrap() {
        //         let dpc = WALLEDPC::new(
        //             &Options {
        //                 is_force: true,
        //                 is_quiet: true,
        //                 is_lz: true,
        //                 is_unsafe: true,
        //                 is_optimization: false,
        //                 is_recursive: false,
        //             },
        //             &Vec::<&OsStr>::new(),
        //         );
        //         match dpc.fmt_extract(&mtl_input_path, &PathBuf::from(&mtl_output_path)) {
        //             Ok(_) => {
        //                 println!("parsed material from mesh parser");
        //             }
        //             Err(ref e) => {
        //                 if e.kind() != ErrorKind::Other {
        //                     panic!(
        //                         "{} {} {}",
        //                         e,
        //                         mtl_input_path.to_string_lossy(),
        //                         mtl_output_path.to_string_lossy()
        //                     );
        //                 }
        //             }
        //         }
        //     }
        //     let mtl_json_file = File::open(mtl_json_path)?;
        //     let mtl_object: MaterialObject = serde_json::from_reader(mtl_json_file)?;

        //     let color: [f32; 3] = mtl_object.material.color.data[..3].try_into().unwrap();

        //     let mut material = obj::Material::new(String::from("material"));
        //     material.ka = Some(color);
        //     materials.push(material);
        // }
        // let arc_materials = Arc::new(materials.get(0).unwrap().to_owned());

        let group = obj::Group {
            name: String::from("meshgroup"),
            index: 0,
            material: None,
            // material: Some(obj::ObjMaterial::Mtl(arc_materials.clone())),
            polys: polygons,
        };
        let object = obj::Object {
            name: String::from("mesh"),
            groups: vec![group],
        };

        let obj_path = output_path.join("data.obj");
        let mut output_obj_file = File::create(obj_path)?;

        let obj_data = obj::ObjData {
            position: vertex_positions,
            texture: vertex_uvs,
            normal: vertex_normals,
            objects: vec![object],
            material_libs: Vec::new(),
            // material_libs: vec![obj::Mtl {
            //     filename: String::from("materials.mtl"),
            //     materials: vec![arc_materials.clone()],
            // }],
        };
        obj_data.write_to_buf(&mut output_obj_file).unwrap();

        // GLTF EXPORT ATTEMPT...
        // let tri_mesh = three_d_asset::geometry::TriMesh {
        //     positions: three_d_asset::geometry::Positions::F32(vertex_positions),
        //     indices: three_d_asset::geometry::Indices::U16(indices),
        //     uvs: Some(vertex_uvs),
        //     normals: None,
        //     tangents: None,
        //     colors: None,
        // };
        // let mut transform: [[f32; 4]; 4] = [[0.0; 4]; 4];
        // for item in mesh_header
        //     .transform
        //     .data
        //     .chunks(4)
        //     .map(|x| x.to_vec())
        //     .enumerate()
        // {
        //     let (i, x): (usize, Vec<f32>) = item;
        //     transform[i] = x.try_into().unwrap();
        // }

        // let save_mesh = three_d_asset::Primitive {
        //     name: String::from("mesh"),
        //     transformation: prelude::Matrix4::from(transform),
        //     geometry: three_d_asset::geometry::Geometry::Triangles(tri_mesh),
        //     animations: Vec::new(),
        //     material_index: None,
        // };
        // let save_model = three_d_asset::Model {
        //     name: String::from("model"),
        //     geometries: vec![save_mesh],
        //     materials: vec![],
        // };

        // let mut assets = three_d_asset::io::RawAssets::new();
        // assets.insert(data_path, save_model);
        // assets.save();

        let object = MeshObject { mesh_header, mesh };

        output_file.write(serde_json::to_string_pretty(&object)?.as_bytes())?;

        Ok((
            object.mesh_header.hard_links(),
            object.mesh_header.soft_links(),
        ))
    }
}
