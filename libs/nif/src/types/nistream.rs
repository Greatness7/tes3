// rust std imports
use std::collections::VecDeque;
use std::io::{Read, Seek, Write};
use std::path::Path;

// external imports
use slotmap::{new_key_type, DenseSlotMap, Key};

// internal imports
use crate::prelude::*;

new_key_type! { pub struct NiKey; }

#[derive(Clone, Debug, Default)]
pub struct NiStream {
    pub objects: DenseSlotMap<NiKey, NiType>,
    pub roots: Vec<NiLink<NiObject>>,
}

impl NiStream {
    pub const HEADER: [u8; 40] = *b"NetImmerse File Format, Version 4.0.0.2\n";
    pub const VERSION: u32 = 0x4000002;

    pub fn new() -> Self {
        default()
    }

    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut stream = Self::new();
        stream.load_path(path)?;
        Ok(stream)
    }

    pub fn from_path_offset(path: impl AsRef<Path>, offset: u64, size: usize) -> io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        file.seek(io::SeekFrom::Start(offset))?;

        let mut bytes = vec![0; size];
        file.read_exact(&mut bytes)?;

        let mut stream = Self::new();
        stream.load_bytes(&bytes)?;
        Ok(stream)
    }

    pub fn load_path(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        self.load_bytes(&std::fs::read(path)?)
    }

    pub fn from_bytes(bytes: &[u8]) -> io::Result<Self> {
        let mut stream = Self::new();
        stream.load_bytes(bytes)?;
        Ok(stream)
    }

    pub fn load_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut stream = Reader::new(bytes);

        // validate header
        let header: [u8; 40] = stream.load()?;
        if header != Self::HEADER {
            return Reader::error("Invalid NIF Header");
        }

        // validate version
        let version: u32 = stream.load()?;
        if version != Self::VERSION {
            return Reader::error("Invalid NIF Version");
        }

        // allocate objects
        let num_objects = stream.load_as::<u32, usize>()?;
        self.objects.reserve(num_objects);

        // populate objects
        for _ in 0..num_objects {
            self.objects.insert(stream.load()?);
        }

        // allocate roots
        let num_roots = stream.load_as::<u32, usize>()?;
        self.roots.reserve(num_roots);

        // populate roots
        for _ in 0..num_roots {
            self.roots.push(stream.load()?);
        }

        Ok(())
    }

    pub fn save_path(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = std::fs::File::create(path)?;
        file.write_all(self.save_bytes()?.as_slice())?;
        Ok(())
    }

    pub fn save_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut stream = Writer::new(vec![]);

        // write header
        stream.save(&Self::HEADER)?;

        // write version
        stream.save(&Self::VERSION)?;

        // parse objects
        let objects: Vec<_> = self.objects().collect();

        // objects count
        stream.save_as::<u32>(objects.len())?;

        // resolve links
        for (key, _) in &objects {
            stream.context.insert(key.data().as_ffi(), stream.context.len() as u64);
        }

        // write objects
        for (_, object) in objects {
            stream.save(object)?;
        }

        // write roots
        stream.save(&self.roots)?;

        Ok(stream.cursor.into_inner())
    }

    fn objects(&self) -> impl Iterator<Item = (NiKey, &NiType)> {
        let mut seen = HashSet::new();
        let mut keys = Vec::new();
        self.roots.visitor(&mut |key| keys.push(key));

        std::iter::from_fn(move || {
            while let Some(key) = keys.pop() {
                if !key.is_null() && seen.insert(key) {
                    if let Some(object) = self.objects.get(key) {
                        object.visitor(&mut |key| keys.push(key));
                        return Some((key, object));
                    }
                }
            }
            None
        })
    }

    pub fn retain_reachable(&mut self) {
        let mut seen = HashSet::new();
        let mut keys = Vec::new();
        self.roots.visitor(&mut |key| keys.push(key));
        while let Some(key) = keys.pop() {
            if !key.is_null() && seen.insert(key) {
                if let Some(object) = self.objects.get(key) {
                    object.visitor(&mut |key| keys.push(key));
                }
            }
        }
        self.objects.retain(|key, _| seen.contains(&key));
    }

    /// Insert an object into the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use nif::*;
    ///
    /// let mut stream = NiStream::new();
    ///
    /// let link1 = stream.insert(NiTriShape::default());
    /// let link2 = stream.insert(NiTriShapeData::default());
    ///
    /// let object1 = stream.get(link1).unwrap();
    /// let object2 = stream.get(link2).unwrap();
    ///
    /// assert_eq!(object1.type_name(), b"NiTriShape");
    /// assert_eq!(object2.type_name(), b"NiTriShapeData");
    /// ```
    #[inline]
    pub fn insert<T>(&mut self, object: T) -> NiLink<T>
    where
        T: Into<NiType>,
    {
        NiLink::new(self.objects.insert(object.into()))
    }

    #[inline]
    pub fn remove<T>(&mut self, link: NiLink<T>) -> Option<T>
    where
        T: TryFrom<NiType>,
    {
        self.objects.remove(link.key)?.try_into().ok()
    }

    /// Retrieve an object from the stream.
    ///
    /// # Examples
    ///
    /// ```
    /// use nif::*;
    ///
    /// let mut stream = NiStream::new();
    ///
    /// let link = stream.insert(NiNode::default());
    ///
    /// let object = stream.get(link).unwrap();
    ///
    /// assert_eq!(object.type_name(), b"NiNode")
    /// ```
    #[inline]
    pub fn get<'a, T>(&'a self, link: NiLink<T>) -> Option<&'a T>
    where
        &'a T: TryFrom<&'a NiType>,
    {
        self.get_as(link)
    }

    /// Retrieve an object of the specified type from the stream.
    #[inline]
    pub fn get_as<'a, T, U>(&'a self, link: NiLink<T>) -> Option<&'a U>
    where
        &'a U: TryFrom<&'a NiType>,
    {
        self.objects.get(link.key).and_then(|object| object.try_into().ok())
    }

    #[inline]
    pub fn get_mut<'a, T>(&'a mut self, link: NiLink<T>) -> Option<&'a mut T>
    where
        &'a mut T: TryFrom<&'a mut NiType>,
    {
        self.get_as_mut(link)
    }

    /// Retrieve an object of the specified type from the stream.
    #[inline]
    pub fn get_as_mut<'a, T, U>(&'a mut self, link: NiLink<T>) -> Option<&'a mut U>
    where
        &'a mut U: TryFrom<&'a mut NiType>,
    {
        self.objects.get_mut(link.key).and_then(|object| object.try_into().ok())
    }

    /// Retrieve multiple objects from the stream.
    #[inline]
    pub fn get_all<'a, T>(&'a self, links: &'a [NiLink<T>]) -> impl Iterator<Item = &'a T>
    where
        &'a T: TryFrom<&'a NiType>,
    {
        self.get_all_as(links)
    }

    /// Retrieve multiple objects of the specified type from the stream.
    #[inline]
    pub fn get_all_as<'a, T, U>(&'a self, links: &'a [NiLink<T>]) -> impl Iterator<Item = &'a U>
    where
        &'a U: 'a + TryFrom<&'a NiType>,
    {
        links.iter().filter_map(move |link| self.get_as(*link))
    }

    /// Create an iterator over objects of the specified type.
    ///
    /// # Examples
    ///
    /// ```
    /// use nif::*;
    ///
    /// let mut stream = NiStream::new();
    ///
    /// stream.insert(NiNode::default());
    /// stream.insert(NiTriShape::default());
    ///
    /// for object in stream.objects_of_type::<NiTriShape>() {
    ///     assert_eq!(object.type_name(), b"NiTriShape");
    /// }
    /// ```
    #[inline]
    pub fn objects_of_type<'a, T>(&'a self) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.objects.values().filter_map(|object| object.try_into().ok())
    }

    #[inline]
    pub fn objects_of_type_mut<'a, T>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        &'a mut T: 'a + TryFrom<&'a mut NiType>,
    {
        self.objects.values_mut().filter_map(|object| object.try_into().ok())
    }

    #[inline]
    pub fn objects_of_type_with_link<'a, T>(&'a self) -> impl Iterator<Item = (NiLink<T>, &'a T)>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.objects
            .iter()
            .filter_map(|(key, object)| Some((NiLink::new(key), object.try_into().ok()?)))
    }

    #[inline]
    pub fn objects_of_type_mut_with_link<'a, T>(&'a mut self) -> impl Iterator<Item = (NiLink<T>, &'a mut T)>
    where
        &'a mut T: 'a + TryFrom<&'a mut NiType>,
    {
        self.objects
            .iter_mut()
            .filter_map(|(key, object)| Some((NiLink::new(key), object.try_into().ok()?)))
    }

    #[inline]
    pub fn links_of_type<'a, T>(&'a self) -> impl Iterator<Item = NiLink<T>> + 'a
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.objects_of_type_with_link().map(|(link, _)| link)
    }

    /// Create an iterator over roots of the specified type.
    #[inline]
    pub fn roots_of_type<'a, T>(&'a self) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.get_all_as(self.roots.as_slice())
    }

    #[inline]
    pub fn objects_with_name<'a, T>(&'a self, name: &'a str) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType> + AsRef<NiObjectNET>,
    {
        self.objects_of_type::<T>()
            .filter(move |object| object.as_ref().name.eq_ignore_ascii_case(name))
    }

    #[inline]
    pub fn objects_with_name_mut<'a, T>(&'a mut self, name: &'a str) -> impl Iterator<Item = &'a mut T>
    where
        &'a mut T: 'a + TryFrom<&'a mut NiType> + AsRef<NiObjectNET>,
    {
        self.objects_of_type_mut::<T>()
            .filter(move |object| object.as_ref().name.eq_ignore_ascii_case(name))
    }

    /// Yields all geometries and their world transforms.
    ///
    pub fn geometries<'a, T>(&'a self) -> impl Iterator<Item = (&'a T, Affine3A)>
    where
        &'a T: 'a + TryFrom<&'a NiType> + AsRef<NiGeometry>,
    {
        let mut queue = VecDeque::new();

        for root in &self.roots {
            queue.push_back((root.key, Affine3A::IDENTITY));
        }

        std::iter::from_fn(move || {
            while let Some((key, transform)) = queue.pop_front() {
                let Some(object) = self.objects.get(key) else {
                    continue;
                };

                if let Ok(node) = <&NiNode>::try_from(object) {
                    if !node.children.is_empty() {
                        let transform = transform * node.transform();
                        queue.reserve(node.children.len());
                        for child in &node.children {
                            queue.push_back((child.key, transform));
                        }
                    }
                    continue;
                }

                if let Ok(geometry) = <&T>::try_from(object) {
                    let transform = transform * geometry.as_ref().transform();
                    return Some((geometry, transform));
                };
            }
            None
        })
    }

    /// Bounding sphere encompassing all geometries in the stream.
    ///
    pub fn bounding_sphere(&self) -> Option<NiBound> {
        NiBound::from_geometries(
            self.geometries::<NiGeometry>()
                .filter_map(|(geom, transform)| Some((self.get(geom.geometry_data)?, transform))),
        )
    }

    /// Axis-aligned bounding box encompassing all geometries in the stream.
    ///
    pub fn bounding_box(&self) -> Option<(Vec3, Vec3)> {
        NiBound::aabb_from_geometries(
            self.geometries::<NiGeometry>()
                .filter_map(|(geom, transform)| Some((self.get(geom.geometry_data)?, transform))),
        )
    }

    /// Convenience function for case-insensitive prefix searches.
    ///
    pub fn root_has_string_data_starting_with(&self, prefix: &str) -> bool {
        for root in self.roots_of_type::<NiObjectNET>() {
            for data in root.extra_datas_of_type::<NiStringExtraData>(self) {
                if data.starts_with_ignore_ascii_case(prefix) {
                    return true;
                }
            }
        }
        false
    }

    /// Get a link to the given object, or null if it is not in this stream.
    ///
    pub fn get_link(&self, object: impl AsRef<NiObject>) -> NiLink<NiObject> {
        let object = object.as_ref();
        self.objects_of_type_with_link::<NiObject>()
            .find_map(|(link, other)| std::ptr::eq(object, other).then_some(link))
            .unwrap_or_default()
    }

    pub fn clear_root_transforms(&mut self) {
        for root in &self.roots {
            let _ = self
                .objects
                .get_mut(root.key)
                .and_then(|object| object.try_into().ok())
                .map(NiAVObject::clear_transform);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use glam::vec3;

    #[ignore = "visual"]
    #[test]
    fn test_bounding_sphere() {
        let src_path = "tests/assets/random_objects.nif";
        let dst_path = "tests/assets/random_objects~1.nif";

        let mut stream = NiStream::from_path(src_path).unwrap();

        let NiBound { center, radius } = stream.bounding_sphere().unwrap();

        for shape in stream.objects_with_name_mut::<NiTriShape>("unitSphere") {
            shape.translation = center;
            shape.scale = radius;
        }

        stream.save_path(dst_path).unwrap();
    }

    #[ignore = "visual"]
    #[test]
    fn test_bounding_box() {
        let src_path = "tests/assets/random_objects.nif";
        let dst_path = "tests/assets/random_objects~1.nif";

        let mut stream = NiStream::from_path(src_path).unwrap();

        let (min, max) = stream.bounding_box().unwrap();

        let shape = stream.objects_with_name::<NiTriShape>("unitCube").next().unwrap();
        let data = stream.get_mut(shape.geometry_data).unwrap();

        data.vertices = vec![
            min,
            vec3(max.x, min.y, min.z),
            vec3(min.x, max.y, min.z),
            vec3(max.x, max.y, min.z),
            vec3(min.x, min.y, max.z),
            vec3(max.x, min.y, max.z),
            vec3(min.x, max.y, max.z),
            max,
        ];

        stream.save_path(dst_path).unwrap();
    }
}
