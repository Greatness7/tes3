// rust std imports
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
    pub fn insert<T>(&mut self, object: T) -> NiLink<T>
    where
        T: Into<NiType>,
    {
        NiLink::new(self.objects.insert(object.into()))
    }

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
    pub fn get<'a, T>(&'a self, link: NiLink<T>) -> Option<&'a T>
    where
        &'a T: TryFrom<&'a NiType>,
    {
        self.get_as(link)
    }

    /// Retrieve an object of the specified type from the stream.
    pub fn get_as<'a, T, U>(&'a self, link: NiLink<T>) -> Option<&'a U>
    where
        &'a U: TryFrom<&'a NiType>,
    {
        self.objects.get(link.key).and_then(|object| object.try_into().ok())
    }

    pub fn get_mut<'a, T>(&'a mut self, link: NiLink<T>) -> Option<&'a mut T>
    where
        &'a mut T: TryFrom<&'a mut NiType>,
    {
        self.get_as_mut(link)
    }

    /// Retrieve an object of the specified type from the stream.
    pub fn get_as_mut<'a, T, U>(&'a mut self, link: NiLink<T>) -> Option<&'a mut U>
    where
        &'a mut U: TryFrom<&'a mut NiType>,
    {
        self.objects.get_mut(link.key).and_then(|object| object.try_into().ok())
    }

    /// Retrieve multiple objects from the stream.
    pub fn get_all<'a, T>(&'a self, links: &'a [NiLink<T>]) -> impl Iterator<Item = &'a T>
    where
        &'a T: TryFrom<&'a NiType>,
    {
        self.get_all_as(links)
    }

    /// Retrieve multiple objects of the specified type from the stream.
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
    pub fn objects_of_type<'a, T>(&'a self) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.objects.values().filter_map(|object| object.try_into().ok())
    }

    pub fn objects_of_type_mut<'a, T>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        &'a mut T: 'a + TryFrom<&'a mut NiType>,
    {
        self.objects.values_mut().filter_map(|object| object.try_into().ok())
    }

    pub fn objects_of_type_with_link<'a, T>(&'a self) -> impl Iterator<Item = (NiLink<T>, &'a T)>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.objects
            .iter()
            .filter_map(|(key, object)| Some((NiLink::new(key), object.try_into().ok()?)))
    }

    pub fn objects_of_type_mut_with_link<'a, T>(&'a mut self) -> impl Iterator<Item = (NiLink<T>, &'a mut T)>
    where
        &'a mut T: 'a + TryFrom<&'a mut NiType>,
    {
        self.objects
            .iter_mut()
            .filter_map(|(key, object)| Some((NiLink::new(key), object.try_into().ok()?)))
    }

    pub fn links_of_type<'a, T>(&'a self) -> impl Iterator<Item = NiLink<T>> + 'a
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.objects_of_type_with_link().map(|(link, _)| link)
    }

    /// Create an iterator over roots of the specified type.
    pub fn roots_of_type<'a, T>(&'a self) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType>,
    {
        self.get_all_as(self.roots.as_slice())
    }

    pub fn objects_with_name<'a, T>(&'a self, name: &'a str) -> impl Iterator<Item = &'a T>
    where
        &'a T: 'a + TryFrom<&'a NiType> + AsRef<NiObjectNET>,
    {
        self.objects_of_type::<T>()
            .filter(move |object| object.as_ref().name.eq_ignore_ascii_case(name))
    }

    pub fn objects_with_name_mut<'a, T>(&'a mut self, name: &'a str) -> impl Iterator<Item = &'a mut T>
    where
        &'a mut T: 'a + TryFrom<&'a mut NiType> + AsRef<NiObjectNET>,
    {
        self.objects_of_type_mut::<T>()
            .filter(move |object| object.as_ref().name.eq_ignore_ascii_case(name))
    }
}
