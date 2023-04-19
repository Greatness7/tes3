// external imports
use nalgebra::{Dyn, OMatrix, U3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiMorphData {
    pub base: NiObject,
    #[default(true)]
    pub relative_targets: bool,
    pub targets: Vec<MorphTarget>,
}

impl Load for NiMorphData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_targets: u32 = stream.load()?;
        let num_vertices = stream.load_as::<u32, _>()?;
        let relative_targets = stream.load::<u8>()? != 0;
        let targets = (0..num_targets).load(|_| MorphTarget::load(stream, num_vertices))?;
        Ok(Self {
            base,
            relative_targets,
            targets,
        })
    }
}

impl Save for NiMorphData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u32>(self.targets.len())?;
        stream.save_as::<_, u32>(self.targets.first().map_or(0, |target| target.vertices.ncols()))?;
        stream.save_as::<_, u8>(self.relative_targets)?;
        for target in &self.targets {
            stream.save(target)?;
        }
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct MorphTarget {
    pub base: NiObject,
    pub keys: NiFloatKey,
    #[default(Empty::empty())]
    pub vertices: OMatrix<f32, U3, Dyn>,
}

impl MorphTarget {
    pub(crate) fn load(stream: &mut Reader<'_>, num_vertices: usize) -> io::Result<Self> {
        let base = stream.load()?;
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = stream.load()?;
        let keys = match key_type {
            // `NoInterp` still uses linear key sizes, see: base_anim_female.1st.nif
            KeyType::LinKey | KeyType::NoInterp => LinFloatKeys::load(stream, num_keys)?.into(),
            KeyType::BezKey => BezFloatKeys::load(stream, num_keys)?.into(),
            KeyType::TCBKey => TCBFloatKeys::load(stream, num_keys)?.into(),
            KeyType::EulerKey => panic!("MorphTarget does not support {key_type:?}"),
        };
        let vertices = stream.load_matrix(3, num_vertices)?;
        Ok(Self { base, keys, vertices })
    }
}

impl Save for MorphTarget {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        match &self.keys {
            NiFloatKey::LinKey(keys) => {
                stream.save_as::<_, u32>(keys.ncols())?;
                stream.save(&KeyType::LinKey)?;
                stream.save_matrix(keys)?;
            }
            NiFloatKey::BezKey(keys) => {
                stream.save_as::<_, u32>(keys.ncols())?;
                stream.save(&KeyType::BezKey)?;
                stream.save_matrix(keys)?;
            }
            NiFloatKey::TCBKey(keys) => {
                stream.save_as::<_, u32>(keys.ncols())?;
                stream.save(&KeyType::TCBKey)?;
                stream.save_matrix(keys)?;
            }
        };
        stream.save_matrix(&self.vertices)?;
        Ok(())
    }
}
