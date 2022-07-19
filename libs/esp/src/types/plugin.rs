// rust std imports
use std::io::{self, Write};
use std::path::Path;

// external imports
use rayon::prelude::*;

// internal imports
use crate::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Plugin {
    pub objects: Vec<TES3Object>,
}

impl Plugin {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn load_path(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        self.load_bytes(&std::fs::read(path)?)
    }

    pub fn save_path(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let mut file = std::fs::File::create(&path)?;

        if let Some(header) = self.header_mut() {
            if let Some(extension) = path.as_ref().extension() {
                if extension.eq_ignore_ascii_case("esp") {
                    header.file_type = FileType::Esp;
                } else if extension.eq_ignore_ascii_case("esm") {
                    header.file_type = FileType::Esm;
                }
            }
        }

        file.write_all(&self.save_bytes()?)
    }

    pub fn load_bytes(&mut self, bytes: &[u8]) -> io::Result<()> {
        let mut stream = Reader::new(bytes);

        // do a quick pass calculating the positions of objects
        let mut offsets = Vec::new();
        while let Ok([_, len, ..]) = stream.load::<[u32; 4]>() {
            if let Ok((start, end)) = stream.skip(len as usize) {
                offsets.push((start - 16, end)); // keep header
            }
        }

        // now visit each chunk and decode them all in parellel
        self.objects = offsets
            .into_par_iter()
            .map(|(start, end)| Reader::new(&bytes[start..end]).load())
            .collect::<io::Result<_>>()?;

        Ok(())
    }

    pub fn save_bytes(&mut self) -> io::Result<Vec<u8>> {
        let mut stream = Writer::new(vec![]);

        // update header
        let num_objects = self.objects.len();
        if let Some(header) = self.header_mut() {
            header.num_objects = (num_objects - 1)
                .try_into()
                .map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "too many objects"))?;
        }

        // write objects
        for object in &self.objects {
            stream.save(object)?;
        }

        Ok(stream.cursor.into_inner())
    }

    pub fn header(&self) -> Option<&Header> {
        self.objects_of_type().next()
    }

    pub fn header_mut(&mut self) -> Option<&mut Header> {
        self.objects_of_type_mut().next()
    }

    pub fn into_objects_of_type<T>(self) -> impl Iterator<Item = T>
    where
        TES3Object: TryInto<T>,
    {
        self.objects.into_iter().filter_map(|object| object.try_into().ok())
    }

    pub fn objects_of_type<'a, T: 'a + ?Sized>(&'a self) -> impl Iterator<Item = &'a T>
    where
        &'a TES3Object: TryInto<&'a T>,
    {
        self.objects.iter().filter_map(|object| object.try_into().ok())
    }

    pub fn objects_of_type_mut<'a, T: 'a + ?Sized>(&'a mut self) -> impl Iterator<Item = &'a mut T>
    where
        &'a mut TES3Object: TryInto<&'a mut T>,
    {
        self.objects.iter_mut().filter_map(|object| object.try_into().ok())
    }

    pub fn drain_objects_of_type<T>(&mut self) -> impl Iterator<Item = T> + '_
    where
        TES3Object: TryInto<T>,
        for<'a> &'a mut TES3Object: TryInto<&'a mut T>,
    {
        self.objects
            .drain_filter(|obj| obj.try_into().is_ok())
            .filter_map(|obj| obj.try_into().ok())
    }
}
