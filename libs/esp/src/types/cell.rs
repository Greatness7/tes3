// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Cell {
    pub flags: ObjectFlags,
    pub name: String,
    pub data: CellData,
    pub region: Option<String>,
    pub map_color: Option<[u8; 4]>,
    pub water_height: Option<f32>,
    pub atmosphere_data: Option<AtmosphereData>,
    #[cfg_attr(feature = "serde", serde(with = "crate::features::serde::cell_references"))]
    pub references: HashMap<(u32, u32), Reference>,
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, Eq, PartialEq)]
pub struct CellData {
    pub flags: CellFlags,
    pub grid: (i32, i32),
}

#[esp_meta]
#[derive(LoadSave, Clone, Debug, Default, PartialEq)]
pub struct AtmosphereData {
    pub ambient_color: [u8; 4],
    pub sunlight_color: [u8; 4],
    pub fog_color: [u8; 4],
    pub fog_density: f32,
}

impl Load for Cell {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();

        this.flags = stream.load()?;

        let mut num_temp_refs: i32 = 0;
        let mut moved_refs = vec![];

        while let Ok(tag) = stream.load() {
            match &tag {
                b"NAME" => {
                    this.name = stream.load()?;
                }
                b"DATA" => {
                    stream.expect(12u32)?;
                    this.data = stream.load()?;
                }
                b"RGNN" => {
                    this.region = Some(stream.load()?);
                }
                b"NAM5" => {
                    stream.expect(4u32)?;
                    this.map_color = Some(stream.load()?);
                }
                b"WHGT" => {
                    stream.expect(4u32)?;
                    this.water_height = Some(stream.load()?);
                }
                b"AMBI" => {
                    let size: u32 = stream.load()?;
                    this.atmosphere_data = Some(stream.load()?);
                    // Apparently some editors may add extra padding to this subrecord.
                    // see: https://www.nexusmods.com/morrowind/mods/50999 (version: 1.02, offset: 196017)
                    stream.skip(size - 16)?;
                }
                b"NAM0" => {
                    stream.expect(4u32)?;
                    num_temp_refs = stream.load()?;
                }
                b"MVRF" => {
                    stream.expect(4u32)?;
                    let packed_indices = stream.load()?;
                    // "MVRF" is always followed by "CNDT"
                    stream.expect(*b"CNDT")?;
                    stream.expect(8u32)?;
                    let moved_cell = stream.load()?;
                    // MVRF/CNDT are independent of other subrecords
                    // the moved reference may not have been loaded yet at this point
                    // so postpone assignments until we know all references are loaded
                    moved_refs.push((packed_indices, moved_cell));
                }
                b"FRMR" => {
                    stream.expect(4u32)?;
                    let packed_indices = stream.load()?;
                    let indices = unpack(packed_indices);
                    // unpack indices
                    let mut reference: Reference = stream.load()?;
                    reference.mast_index = indices.0;
                    reference.refr_index = indices.1;
                    // set persistent
                    reference.temporary = num_temp_refs > 0;
                    num_temp_refs -= 1;
                    // insert the ref
                    this.references.insert(indices, reference);
                }
                b"INTV" => {
                    stream.expect(4u32)?;
                    let water_height: i32 = stream.load()?;
                    #[allow(clippy::cast_precision_loss)]
                    {
                        // esp version 1.3 uses float rather than int
                        this.water_height = Some(water_height as f32);
                    }
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

        // assign moved cells
        for (packed_indices, moved_cell) in moved_refs {
            let indices = unpack(packed_indices);
            if let Some(reference) = this.references.get_mut(&indices) {
                reference.moved_cell = Some(moved_cell);
            } else {
                // Since we don't require loading all master files, there is potential that
                // the indices refer to a reference not defined in the current plugin. We've
                // no choice but to trigger an error in this case. Note that the TESCS always
                // copies the associated reference to the plugin, which prevents this from
                // happening. Other tools may not be so nice.
                Reader::error(format!(
                    "Unable to resolve moved reference {:?} for cell {} {:?}",
                    indices, this.name, this.data.grid
                ))?;
            }
        }

        Ok(this)
    }
}

impl Save for Cell {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        // NAME
        stream.save(b"NAME")?;
        stream.save(&self.name)?;
        // DATA
        stream.save(b"DATA")?;
        stream.save(&12u32)?;
        stream.save(&self.data)?;
        // RGNN
        if let Some(value) = &self.region {
            stream.save(b"RGNN")?;
            stream.save(value)?;
        }
        // NAM5
        if let Some(value) = &self.map_color {
            stream.save(b"NAM5")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // WHGT
        if let Some(value) = &self.water_height {
            stream.save(b"WHGT")?;
            stream.save(&4u32)?;
            stream.save(value)?;
        }
        // AMBI
        if let Some(value) = &self.atmosphere_data {
            stream.save(b"AMBI")?;
            stream.save(&16u32)?;
            stream.save(value)?;
        }
        // DELE
        if self.flags.contains(ObjectFlags::DELETED) {
            stream.save(b"DELE")?;
            stream.save(&4u32)?;
            stream.save(&0u32)?;
        }
        //
        let mut num_temp_refs = 0;
        for (i, (key, reference)) in self.references_sorted().into_iter().enumerate() {
            let packed_indices = pack(*key);
            // NAM0
            if (num_temp_refs == 0) && !reference.persistent() {
                num_temp_refs = self.references.len() - i;
                stream.save(b"NAM0")?;
                stream.save(&4u32)?;
                stream.save_as::<u32>(num_temp_refs)?;
            }
            // MVRF
            if let Some(value) = &reference.moved_cell {
                stream.save(b"MVRF")?;
                stream.save(&4u32)?;
                stream.save(&packed_indices)?;
                // CNDT
                stream.save(b"CNDT")?;
                stream.save(&8u32)?;
                stream.save(value)?;
            }
            // FRMR
            stream.save(b"FRMR")?;
            stream.save(&4u32)?;
            stream.save(&packed_indices)?;
            // REFR
            stream.save(reference)?;
        }
        Ok(())
    }
}

impl Cell {
    pub const fn is_interior(&self) -> bool {
        self.data.flags.contains(CellFlags::IS_INTERIOR)
    }

    pub const fn is_exterior(&self) -> bool {
        !self.is_interior()
    }

    pub const fn exterior_coords(&self) -> Option<(i32, i32)> {
        if self.is_exterior() {
            Some(self.data.grid)
        } else {
            None
        }
    }

    pub fn get_region(&self) -> &str {
        self.region.as_deref().unwrap_or("Wilderness")
    }

    fn references_sorted(&self) -> Vec<(&(u32, u32), &Reference)> {
        let mut references: Vec<_> = self.references.iter().collect();

        // sort references such that:
        // 1. persistent references come before temporary references
        // 2. master-defined references come before plugin-defined references
        // 3. references from the same source file are sorted by object index

        references.sort_by_key(|((mast_index, refr_index), reference)| {
            (
                !reference.persistent(),
                match *mast_index {
                    0 => u32::MAX,
                    i => i,
                },
                *refr_index,
            )
        });

        references
    }
}

const fn unpack(packed_indices: u32) -> (u32, u32) {
    let mast_index = packed_indices >> 24;
    let refr_index = packed_indices & 0xFFFFFF;
    (mast_index, refr_index)
}

const fn pack(packed_indices: (u32, u32)) -> u32 {
    let (mast_index, refr_index) = packed_indices;
    debug_assert!(mast_index <= 0xFF);
    debug_assert!(refr_index <= 0xFFFFFF);
    refr_index | (mast_index << 24)
}
