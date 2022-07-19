// external imports
use nalgebra::{dvector, Dynamic, OVector};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiParticlesData {
    pub base: NiGeometryData,
    pub num_particles: u16,
    pub particle_radius: f32,
    pub num_active: u16,
    #[default(dvector![])]
    pub sizes: OVector<f32, Dynamic>,
}

impl Load for NiParticlesData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base: NiGeometryData = stream.load()?;
        let num_particles: u16 = stream.load()?;
        let particle_radius = stream.load()?;
        let num_active: u16 = stream.load()?;
        let has_sizes = stream.load::<u32>()? != 0;
        let num_sizes = if has_sizes { base.vertices.ncols() } else { 0 };
        let sizes = stream.load_matrix(num_sizes, 1)?;
        Ok(Self {
            base,
            num_particles,
            particle_radius,
            num_active,
            sizes,
        })
    }
}

impl Save for NiParticlesData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save(&self.num_particles)?;
        stream.save(&self.particle_radius)?;
        stream.save(&self.num_active)?;
        stream.save_as::<_, u32>(!self.sizes.is_empty())?;
        stream.save_matrix(&self.sizes)?;
        Ok(())
    }
}
