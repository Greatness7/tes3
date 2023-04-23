// external imports
use bytemuck::cast_slice;

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
        stream.save_as::<u32>(self.targets.len())?;
        stream.save_as::<u32>(self.targets.first().map_or(0, |target| target.vertices.len()))?;
        stream.save_as::<u8>(self.relative_targets)?;
        stream.save_seq(&self.targets)?;
        Ok(())
    }
}

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct MorphTarget {
    pub keys: NiFloatKey,
    pub vertices: Vec<Vec3>,
}

impl MorphTarget {
    #[allow(clippy::match_same_arms)]
    pub(crate) fn load(stream: &mut Reader<'_>, num_vertices: usize) -> io::Result<Self> {
        let num_keys = stream.load_as::<u32, _>()?;
        let key_type = stream.load()?;
        let keys = match key_type {
            // `NoInterp` still uses linear key sizes (see: base_anim_female.1st.nif)
            KeyType::NoInterp => NiFloatKey::LinKey(stream.load_vec(num_keys)?),
            KeyType::LinKey => NiFloatKey::LinKey(stream.load_vec(num_keys)?),
            KeyType::BezKey => NiFloatKey::BezKey(stream.load_vec(num_keys)?),
            KeyType::TCBKey => NiFloatKey::TCBKey(stream.load_vec(num_keys)?),
            _ => Reader::error(format!("NiMorphData does not support {key_type:?}"))?,
        };
        let vertices = stream.load_vec(num_vertices)?;
        Ok(Self { keys, vertices })
    }
}

impl Save for MorphTarget {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        let (len, key_type, bytes) = match &self.keys {
            NiFloatKey::LinKey(keys) => (keys.len(), KeyType::LinKey, cast_slice(keys)),
            NiFloatKey::BezKey(keys) => (keys.len(), KeyType::BezKey, cast_slice(keys)),
            NiFloatKey::TCBKey(keys) => (keys.len(), KeyType::TCBKey, cast_slice(keys)),
        };
        stream.save_as::<u32>(len)?;
        stream.save(&key_type)?;
        stream.save_bytes(bytes)?;
        stream.save_vec(&self.vertices)?;
        Ok(())
    }
}
