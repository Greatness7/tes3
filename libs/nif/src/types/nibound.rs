// internal imports
use crate::prelude::*;

#[derive(Meta, Clone, Copy, Debug, Default, PartialEq)]
pub struct NiBound {
    pub center: Vec3,
    pub radius: f32,
}

impl Load for NiBound {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let center = stream.load()?;
        let radius = stream.load()?;
        Ok(Self { center, radius })
    }
}

impl Save for NiBound {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.center)?;
        stream.save(&self.radius)?;
        Ok(())
    }
}

impl NiBound {
    /// Merges this bounding sphere with another, creating a new bounding sphere that encloses both.
    ///
    #[must_use]
    pub fn merged_with(self, other: Self) -> Self {
        let c_diff = self.center - other.center;
        let r_diff = other.radius - self.radius;
        let len_sq = c_diff.length_squared();

        if r_diff * r_diff >= len_sq {
            // One sphere encloses the other or coincident.
            return if r_diff >= 0.0 { other } else { self };
        }

        let dist = len_sq.sqrt();
        let alpha = (dist - r_diff) / (2.0 * dist);
        Self {
            center: other.center + alpha * c_diff,
            radius: 0.5 * (other.radius + dist + self.radius),
        }
    }

    /// Returns the bounding sphere transformed by the given transform.
    ///
    #[must_use]
    pub fn transformed_by(self, transform: &Affine3A) -> Self {
        let scale = transform.matrix3.z_axis.length(); // assume uniform
        let center = transform.transform_point3(self.center);
        let radius = self.radius * scale;
        Self { center, radius }
    }

    /// Computes the bounding sphere of the given geometries.
    ///
    #[allow(private_bounds)]
    pub fn from_geometries(geometries: impl IntoIterator<Item: GeometryTransform>) -> Option<Self> {
        let mut merged: Option<NiBound> = None;

        for item in geometries {
            let (data, transform) = item.get();
            let bound = data.bound.transformed_by(transform);
            merged = Some(merged.map_or(bound, |inner| inner.merged_with(bound)));
        }

        merged
    }

    /// Computes the axis-aligned bounding box of the given geometries.
    ///
    #[allow(private_bounds)]
    pub fn aabb_from_geometries(geometries: impl IntoIterator<Item: GeometryTransform>) -> Option<(Vec3, Vec3)> {
        let mut min = Vec3::splat(f32::INFINITY);
        let mut max = Vec3::splat(f32::NEG_INFINITY);

        for item in geometries {
            let (data, transform) = item.get();
            for v in &data.vertices {
                let v = transform.transform_point3(*v);
                min = min.min(v);
                max = max.max(v);
            }
        }

        if min.is_finite() && max.is_finite() {
            Some((min, max))
        } else {
            None
        }
    }
}

/// Blanket trait to allow passing in either slices or iterators.
///
trait GeometryTransform {
    fn get(&self) -> (&NiGeometryData, &Affine3A);
}

impl<T> GeometryTransform for (T, Affine3A)
where
    T: AsRef<NiGeometryData>,
{
    #[inline]
    fn get(&self) -> (&NiGeometryData, &Affine3A) {
        (self.0.as_ref(), &self.1)
    }
}

impl<T> GeometryTransform for &(T, Affine3A)
where
    T: AsRef<NiGeometryData>,
{
    #[inline]
    fn get(&self) -> (&NiGeometryData, &Affine3A) {
        (self.0.as_ref(), &self.1)
    }
}

impl<X, T> GeometryTransform for &(X, T, Affine3A)
where
    T: AsRef<NiGeometryData>,
{
    #[inline]
    fn get(&self) -> (&NiGeometryData, &Affine3A) {
        (self.1.as_ref(), &self.2)
    }
}
