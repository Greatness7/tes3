// rust std imports
use std::io::Read;
use std::path::Path;

// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, PartialEq, SmartDefault)]
pub struct Header {
    pub flags: ObjectFlags,
    #[default(1.3)]
    pub version: f32,
    pub file_type: FileType,
    pub author: FixedString<32>,
    pub description: FixedString<256>,
    pub num_objects: u32,
    pub masters: Vec<(String, u64)>,
}

impl Load for Header {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"HEDR" => {
                    stream.expect(300u32)?;
                    this.version = stream.load()?;
                    this.file_type = stream.load()?;
                    this.author = stream.load()?;
                    this.description = stream.load()?;
                    this.num_objects = stream.load()?;
                }
                b"MAST" => {
                    let master_name = stream.load()?;
                    // DATA
                    stream.expect(*b"DATA")?;
                    stream.expect(8u32)?;
                    let master_size = stream.load()?;
                    //
                    this.masters.push((master_name, master_size));
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }
        }

        Ok(this)
    }
}

impl Save for Header {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // HEDR
        stream.save(b"HEDR")?;
        stream.save(&300u32)?;
        stream.save(&self.version)?;
        stream.save(&self.file_type)?;
        stream.save(&self.author)?;
        stream.save(&self.description)?;
        stream.save(&self.num_objects)?;
        //
        for (master_name, master_size) in &self.masters {
            // MAST
            stream.save(b"MAST")?;
            stream.save(master_name)?;
            // DATA
            stream.save(b"DATA")?;
            stream.save(&8u32)?;
            stream.save(master_size)?;
        }
        Ok(())
    }
}

impl Header {
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let mut file = std::fs::File::open(path)?;
        let mut buffer = [0; 4];

        // tag
        file.read_exact(&mut buffer)?;
        if buffer != *Header::TAG {
            Reader::error("invalid header tag")?;
        }

        // size
        file.read_exact(&mut buffer)?;
        let size = u32::from_le_bytes(buffer) as usize;

        // padding
        file.read_exact(&mut buffer)?;

        // content
        let mut buffer = vec![0; size + 4];
        file.read_exact(&mut buffer)?;
        Reader::new(&buffer).load()
    }
}
