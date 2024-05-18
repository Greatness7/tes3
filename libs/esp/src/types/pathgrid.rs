// internal imports
use crate::prelude::*;
// wasm
#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::*;

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct PathGrid {
    pub flags: ObjectFlags,
    pub cell: Option<String>,
    pub data: Option<PathGridData>,
    pub points: Option<Vec<PathGridPoint>>,
    pub connections: Option<Vec<u32>>,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct PathGridData {
    pub grid: (i32, i32),
    pub granularity: u16,
    pub point_count: u16,
}

#[cfg_attr(feature = "wasm", wasm_bindgen)]
#[derive(Meta, Clone, Debug, Default, Eq, PartialEq)]
pub struct PathGridPoint {
    pub location: [i32; 3],
    pub auto_generated: u8,
    pub connection_count: u8,
}

impl Load for PathGrid {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.cell = Some(stream.load()?);
                }
                b"DATA" => {
                    stream.expect(12u32)?;
                    this.data = Some(stream.load()?);
                }
                b"PGRP" => {
                    let len: u32 = stream.load()?;
                    this.points = Some((0..len / 16).load(|_| stream.load())?);
                }
                b"PGRC" => {
                    let len: u32 = stream.load()?;
                    this.connections = Some((0..len / 4).load(|_| stream.load())?);
                }
                b"DELE" => {
                    let size: u32 = stream.load()?;
                    stream.skip(size)?;
                    this.flags.insert(ObjectFlags::DELETED);
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for PathGrid {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // DATA
        if let Some(value) = &self.data {
            stream.save(b"DATA")?;
            stream.save(&12u32)?;
            stream.save(value)?;
        }
        // NAME
        if let Some(value) = &self.cell {
            stream.save(b"NAME")?;
            stream.save(value)?;
        }
        // PGRP
        if let Some(values) = self.points.as_ref().filter(|x| !x.is_empty()) {
            stream.save(b"PGRP")?;
            stream.save_as::<_, u32>(values.len() * 16)?;
            for value in values {
                stream.save(value)?;
            }
        }
        // PGRC
        if let Some(values) = self.connections.as_ref().filter(|x| !x.is_empty()) {
            stream.save(b"PGRC")?;
            stream.save_as::<_, u32>(values.len() * 4)?;
            for value in values {
                stream.save(value)?;
            }
        }
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        Ok(())
    }
}

impl Load for PathGridPoint {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let location = stream.load()?;
        let auto_generated = stream.load()?;
        let connection_count = stream.load()?;
        stream.skip(2)?; // padding
        Ok(Self {
            location,
            auto_generated,
            connection_count,
        })
    }
}

impl Save for PathGridPoint {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.location)?;
        stream.save(&self.auto_generated)?;
        stream.save(&self.connection_count)?;
        stream.save(&[0u8; 2])?; // padding
        Ok(())
    }
}
