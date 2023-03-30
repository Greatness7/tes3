// external imports
use nalgebra::{Dyn, OMatrix, OVector, U3};

// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiTriShapeData {
    pub base: NiTriBasedGeomData,
    #[default(Empty::empty())]
    pub triangles: OMatrix<u16, U3, Dyn>,
    pub shared_normals: Vec<OVector<u16, Dyn>>,
}

impl Load for NiTriShapeData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let num_triangles = stream.load_as::<u16, _>()?;
        let _num_triangle_points: u32 = stream.load()?; // TODO: why does this exist?
        let triangles = stream.load_matrix(3, num_triangles)?;
        let num_shared_normals: u16 = stream.load()?;
        let shared_normals = (0..num_shared_normals).load(|_| {
            Ok({
                let num_indices = stream.load_as::<u16, _>()?;
                stream.load_matrix(num_indices, 1)?
            })
        })?;
        Ok(Self {
            base,
            triangles,
            shared_normals,
        })
    }
}

impl Save for NiTriShapeData {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.base)?;
        stream.save_as::<_, u16>(self.triangles.ncols())?;
        stream.save_as::<_, u32>(self.triangles.len())?;
        stream.save_matrix(&self.triangles)?;
        stream.save_as::<_, u16>(self.shared_normals.len())?;
        for indices in &self.shared_normals {
            stream.save_as::<_, u16>(indices.len())?;
            stream.save_matrix(indices)?;
        }
        Ok(())
    }
}
