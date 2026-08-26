// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, Default, PartialEq)]
pub struct NiSkinInstance {
    pub base: NiObject,
    pub data: NiLink<NiSkinData>,
    pub root: NiLink<NiAVObject>,
    pub bones: Vec<NiLink<NiAVObject>>,
}

impl Load for NiSkinInstance {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let data = stream.load()?;
        let root = stream.load()?;
        let bones = stream.load()?;
        Ok(Self { base, data, root, bones })
    }
}

impl Save for NiSkinInstance {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.data)?;
        stream.save(&self.root)?;
        stream.save(&self.bones)?;
        Ok(())
    }
}

impl NiSkinInstance {
    /// Applies skin deformation to the given geometry data, returning new vertices and normals.
    ///
    /// Prefer [`NiStream::apply_skins`] when deforming a whole stream, as this rebuilds the map of
    /// world transforms on every call.
    pub fn deform(&self, stream: &NiStream, data: &NiGeometryData) -> Option<(Vec<Vec3>, Vec<Vec3>)> {
        self.deform_with(stream, data, &stream.world_transforms())
    }

    pub(crate) fn deform_with(
        &self,
        stream: &NiStream,
        data: &NiGeometryData,
        world_transforms: &HashMap<NiKey, Affine3A>,
    ) -> Option<(Vec<Vec3>, Vec<Vec3>)> {
        let skin_data = stream.get(self.data)?;
        let root_transform = world_transforms.get(&self.root.key)?;

        let mut vertices = vec![Vec3::ZERO; data.vertices.len()];
        let mut normals = vec![Vec3::ZERO; data.normals.len()];

        let skin_transform = skin_data.transform() * uniform_inverse(*root_transform);

        for (bone, bone_data) in self.bones.iter().zip(&skin_data.bone_data) {
            let Some(&bone_transform) = world_transforms.get(&bone.key) else {
                continue;
            };

            let transform = skin_transform * bone_transform * bone_data.transform();

            let scale = transform.matrix3.z_axis.length();
            let rotation = transform.matrix3 * if scale > 0.0 { scale.recip() } else { 0.0 };

            for &(index, weight) in &bone_data.vertex_weights {
                let index = usize::from(index);
                if let Some(vertex) = data.vertices.get(index) {
                    vertices[index] += weight * transform.transform_point3(*vertex);
                }
                if let Some(normal) = data.normals.get(index) {
                    normals[index] += weight * (rotation * *normal);
                }
            }
        }

        for normal in &mut normals {
            *normal = normal.normalize_or_zero();
        }

        Some((vertices, normals))
    }
}

/// Inverts a transform whose `matrix3` is an orthonormal rotation times a uniform scale.
///
/// This mirrors the engine's `NiTransform::Invert`, which uses `R^T` and `1/s` rather than a
/// general inverse. The NIF format guarantees the form structurally, as rotation and scale are
/// separate fields. Degenerate transforms invert to zero rather than NaN.
fn uniform_inverse(transform: Affine3A) -> Affine3A {
    let scale_squared = transform.matrix3.z_axis.length_squared();
    let matrix3 = transform.matrix3.transpose() * if scale_squared > 0.0 { scale_squared.recip() } else { 0.0 };
    Affine3A {
        matrix3,
        translation: -(matrix3 * transform.translation),
    }
}
