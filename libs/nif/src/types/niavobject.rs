// external imports
use nalgebra::{Matrix3, Vector3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiAVObject {
    pub base: NiObjectNET,
    pub flags: u16,
    pub translation: Vector3<f32>,
    #[default(Matrix3::identity())]
    pub rotation: Matrix3<f32>,
    #[default(1.0)]
    pub scale: f32,
    pub velocity: Vector3<f32>,
    pub properties: Vec<NiLink<NiProperty>>,
    pub bounding_volume: Option<NiBoundingVolume>,
}

impl Load for NiAVObject {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let flags = stream.load()?;
        let translation = stream.load()?;
        let rotation = stream.load()?;
        let scale = stream.load()?;
        let velocity = stream.load()?;
        let properties = stream.load()?;
        let has_bounding_volume: u32 = stream.load()?;
        let bounding_volume = match has_bounding_volume {
            0 => None,
            _ => Some(stream.load()?),
        };
        Ok(Self {
            base,
            flags,
            translation,
            rotation,
            scale,
            velocity,
            properties,
            bounding_volume,
        })
    }
}

impl Save for NiAVObject {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.flags)?;
        stream.save(&self.translation)?;
        stream.save(&self.rotation)?;
        stream.save(&self.scale)?;
        stream.save(&self.velocity)?;
        stream.save(&self.properties)?;
        stream.save_as::<_, u32>(self.bounding_volume.is_some())?;
        if let Some(bounding_volume) = &self.bounding_volume {
            stream.save(bounding_volume)?;
        }
        Ok(())
    }
}
