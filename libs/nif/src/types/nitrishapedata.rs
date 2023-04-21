// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Debug, PartialEq, SmartDefault)]
pub struct NiTriShapeData {
    pub base: NiTriBasedGeomData,
    pub triangles: Vec<[u16; 3]>,
    pub shared_normals: Vec<Vec<u16>>,
}

impl Load for NiTriShapeData {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let base = stream.load()?;
        let _num_triangles: u16 = stream.load()?;
        let num_triangle_points = stream.load_as::<u32, usize>()?;
        let triangles = stream.load_vec(num_triangle_points / 3)?;
        let num_shared_normals: u16 = stream.load()?;
        let shared_normals = (0..num_shared_normals).load(|_| {
            Ok({
                let num_indices = stream.load_as::<u16, _>()?;
                stream.load_vec(num_indices)?
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
        stream.save_as::<u16>(self.triangles.len())?;
        stream.save_as::<u32>(self.triangles.len() * 3)?;
        stream.save_vec(&self.triangles)?;
        stream.save_as::<u16>(self.shared_normals.len())?;
        for indices in &self.shared_normals {
            stream.save_as::<u16>(indices.len())?;
            stream.save_vec(indices)?;
        }
        Ok(())
    }
}
