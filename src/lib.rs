//!
//! A library for working with content from [The Elder Scrolls 3: Morrowind](https://en.wikipedia.org/wiki/The_Elder_Scrolls_III:_Morrowind).
//!
#![allow(non_snake_case)]

/// Module for working with `.esp` files.
#[cfg(feature = "esp")]
pub use esp;

/// Module for working with `.nif` files.
#[cfg(feature = "nif")]
pub use nif;

// ----------------------------------------------------------------

mod nif_ext;
pub use nif_ext::*;

use glam::Affine3A;
use interoptopus::inventory::Inventory;
use interoptopus::{ffi, ffi_function, ffi_type};

#[ffi_type]
pub struct Scene {
    pub Nodes: ffi::Vec<Node>,
    pub VisualMeshes: ffi::Vec<Mesh>,
    pub CollisionMeshes: ffi::Vec<Mesh>,
}

#[ffi_type]
#[derive(Clone)]
pub struct Node {
    pub Name: ffi::String,
    pub Transform: Transform,
}

#[ffi_type]
#[derive(Clone)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[ffi_type]
#[derive(Clone)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[ffi_type]
#[derive(Clone)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[ffi_type]
#[derive(Clone)]
pub struct Triangle {
    pub v0: u16,
    pub v1: u16,
    pub v2: u16,
}

#[ffi_type]
#[derive(Clone)]
pub struct Mesh {
    pub Transform: Transform,
    pub Texture: ffi::String,
    pub Vertices: ffi::Vec<Vec3>,
    pub Triangles: ffi::Vec<Triangle>,
    pub Normals: ffi::Vec<Vec3>,
    pub Colors: ffi::Vec<Vec4>,
    pub UvSet0: ffi::Vec<Vec2>,
}

#[ffi_type]
#[derive(Default, Clone)]
pub struct Transform {
    pub Translation: [f32; 3],
    pub Rotation: [f32; 4],
    pub Scale: f32,
}

impl From<Affine3A> for Transform {
    fn from(value: Affine3A) -> Self {
        let (scale, rotation, translation) = value.to_scale_rotation_translation();
        Self {
            Translation: translation.into(),
            Rotation: rotation.into(),
            Scale: scale.z,
        }
    }
}

impl Mesh {
    pub fn new(
        transform: impl Into<Transform>, //
        texture: impl Into<ffi::String>,
        data: &nif::NiTriShapeData,
    ) -> Self {
        Self {
            Transform: transform.into(),
            Vertices: ffi::Vec::from_vec(
                data.vertices //
                    .iter()
                    .map(|v| Vec3 { x: v.x, y: v.y, z: v.z })
                    .collect(),
            ),
            Triangles: ffi::Vec::from_vec(
                data.triangles //
                    .iter()
                    .map(|&[v0, v1, v2]| Triangle { v0, v1, v2 })
                    .collect(),
            ),
            Normals: ffi::Vec::from_vec(
                data.normals //
                    .iter()
                    .map(|n| Vec3 { x: n.x, y: n.y, z: n.z })
                    .collect(),
            ),
            Colors: ffi::Vec::from_vec(
                data.vertex_colors //
                    .iter()
                    .map(|c| Vec4 {
                        x: c.x,
                        y: c.y,
                        z: c.z,
                        w: c.w,
                    })
                    .collect(),
            ),
            UvSet0: ffi::Vec::from_vec(
                data.uv_set(0) //
                    .unwrap_or_default()
                    .iter()
                    .map(|v| Vec2 { x: v.x, y: v.y })
                    .collect(),
            ),
            Texture: texture.into(),
        }
    }
}

#[ffi_function]
pub fn LoadScene(path: ffi::String) -> ffi::Result<Scene, ffi::String> {
    let path = path.as_str();

    let mut stream = nif::NiStream::new();

    if let Err(e) = stream.load_path(path) {
        return ffi::Err(ffi::String::from_string(e.to_string()));
    }

    stream.clear_root_node_transforms();
    stream.discard_editor_markers();
    stream.flatten_properties();

    let visual_meshes: Vec<_> = stream
        .visible_geometries()
        .map(|(shape, data, transform)| {
            let texture = stream.get_texture(shape);
            Mesh::new(transform, texture, data)
        })
        .collect();

    let collision_meshes: Vec<_> = stream
        .collision_geometries()
        .map(|(shape, data, transform)| {
            let texture = stream.get_texture(shape);
            Mesh::new(transform, texture, data)
        })
        .collect();

    ffi::Ok(Scene {
        Nodes: vec![].into(),
        VisualMeshes: visual_meshes.into(),
        CollisionMeshes: collision_meshes.into(),
    })
}

pub fn ffi_inventory() -> Inventory {
    use interoptopus::{builtins_string, builtins_vec, extra_type, function};

    let inventory = Inventory::builder() //
        .register(builtins_string!())
        .register(builtins_vec!(Node))
        .register(builtins_vec!(Mesh))
        .register(builtins_vec!(Vec2))
        .register(builtins_vec!(Vec3))
        .register(builtins_vec!(Vec4))
        .register(builtins_vec!(Triangle))
        .register(extra_type!(Scene))
        .register(extra_type!(Node))
        .register(extra_type!(Mesh))
        .register(function!(LoadScene))
        .validate()
        .build();

    inventory
}

#[test]
fn generate_bindings() {
    use interoptopus_backend_csharp::Interop;

    let inventory = ffi_inventory();

    Interop::builder()
        // .class("Functions")
        .dll_name("tes3")
        .namespace_mappings(interoptopus::lang::NamespaceMappings::new("TES3"))
        .inventory(inventory)
        .build()
        .unwrap()
        .write_file("bindings/TES3.cs")
        .unwrap()
}
