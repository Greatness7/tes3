// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Container {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: Option<String>,
    pub mesh: Option<String>,
    pub script: Option<String>,
    pub encumbrance: Option<f32>,
    pub container_flags: Option<u32>,
    pub inventory: Option<Vec<(i32, FixedString<32>)>>,
}

impl Load for Container {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.id = stream.load()?;
                }
                b"MODL" => {
                    this.mesh = Some(stream.load()?);
                }
                b"FNAM" => {
                    this.name = Some(stream.load()?);
                }
                b"CNDT" => {
                    stream.expect(4u32)?;
                    this.encumbrance = Some(stream.load()?);
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    this.container_flags = Some(stream.load()?);
                }
                b"SCRI" => {
                    this.script = Some(stream.load()?);
                }
                b"NPCO" => {
                    stream.expect(36u32)?;
                    this.inventory.get_or_insert_with(default).push(stream.load()?);
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

impl Save for Container {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.id)?;
        // MODL
        if let Some(value) = &self.mesh {
            stream.save(b"MODL")?;
            stream.save(value)?;
        }
        // FNAM
        if let Some(value) = &self.name {
            stream.save(b"FNAM")?;
            stream.save(value)?;
        }
        // CNDT
        if let Some(value) = &self.encumbrance {
            stream.save(b"CNDT")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // FLAG
        if let Some(value) = &self.container_flags {
            stream.save(b"FLAG")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // SCRI
        if let Some(value) = &self.script {
            stream.save(b"SCRI")?;
            stream.save(value)?;
        }
        // NPCO
        for (count, id) in self.inventory.iter().flatten() {
            stream.save(b"NPCO")?;
            stream.save(&36u32)?;
            stream.save(count)?;
            stream.save(id)?;
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
