// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Container {
    pub flags: ObjectFlags,
    pub id: String,
    pub name: String,
    pub mesh: String,
    pub script: String,
    pub encumbrance: f32,
    pub container_flags: u32,
    pub inventory: Vec<(i32, FixedString<32>)>,
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
                    this.mesh = stream.load()?;
                }
                b"FNAM" => {
                    this.name = stream.load()?;
                }
                b"CNDT" => {
                    stream.expect(4u32)?;
                    this.encumbrance = stream.load()?;
                }
                b"FLAG" => {
                    stream.expect(4u32)?;
                    this.container_flags = stream.load()?;
                }
                b"SCRI" => {
                    this.script = stream.load()?;
                }
                b"NPCO" => {
                    stream.expect(36u32)?;
                    this.inventory.push(stream.load()?);
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
        if !self.mesh.is_empty() {
            stream.save(b"MODL")?;
            stream.save(&self.mesh)?;
        }
        // FNAM
        if !self.name.is_empty() {
            stream.save(b"FNAM")?;
            stream.save(&self.name)?;
        }
        // CNDT
        stream.save(b"CNDT")?;
        stream.save(&4u32)?;
        stream.save(&self.encumbrance)?;
        // FLAG
        stream.save(b"FLAG")?;
        stream.save(&4u32)?;
        stream.save(&self.container_flags)?;
        // SCRI
        if !self.script.is_empty() {
            stream.save(b"SCRI")?;
            stream.save(&self.script)?;
        }
        // NPCO
        for (count, id) in &self.inventory {
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
