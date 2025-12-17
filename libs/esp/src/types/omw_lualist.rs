// internal imports
use crate::prelude::*;

#[esp_meta]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct PerInstanceConfig {
    pub attach: bool,
    pub mast_index: i32,
    pub ref_index: u32,
    pub data: Vec<u8>,
}

#[esp_meta]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct PerRecordConfig {
    pub attach: bool,
    pub id: String,
    pub data: Vec<u8>,
}

#[esp_meta]
#[derive(Clone, Debug, Eq, PartialEq, Default)]
pub struct ScriptConfig {
    pub path: String,
    pub init_data: Vec<u8>, // We'll need to reimplement openmw's serializer too :/
    pub flags: OMWScriptAttachFlag,
    pub types: Vec<String>,
    pub records: Vec<PerRecordConfig>,
    pub instances: Vec<PerInstanceConfig>,
}

#[esp_meta]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ScriptConfigList {
    pub scripts: Vec<ScriptConfig>,
    pub flags: ObjectFlags,
}

impl Load for ScriptConfigList {
    fn load(stream: &mut Reader<'_>) -> io::Result<Self> {
        let mut this: Self = default();
        let mut script_config: ScriptConfig = ScriptConfig::default();

        this.flags = stream.load()?;

        let mut last_tag: Option<[u8; 4]> = None;
        while let Ok(tag) = stream.load() {
            match &tag {
                b"LUAS" => {
                    if script_config != ScriptConfig::default() {
                        this.scripts.push(std::mem::take(&mut script_config));
                    }

                    script_config.path = stream.load()?;
                }
                b"LUAF" => {
                    let size: u32 = stream.load()?;
                    assert!(size % 4 == 0, "Incorrect LUAF Size!");

                    script_config.flags = stream.load()?;

                    for _ in 0..(size / 4 - 1) {
                        let type_string: String = stream.load_string(4)?;
                        script_config.types.push(type_string);
                    }
                }
                b"LUAI" => {
                    let mut per_instance = PerInstanceConfig::default();
                    stream.skip(size_of::<u32>() as u32)?;

                    let attach = stream.load::<u8>()?;
                    per_instance.attach = attach != 0;

                    per_instance.ref_idx = stream.load()?;
                    per_instance.mast_idx = stream.load()?;

                    script_config.instances.push(per_instance);
                }
                b"LUAR" => {
                    let mut per_record = PerRecordConfig::default();
                    let size: u32 = stream.load()?;

                    let attach = stream.load::<u8>()?;
                    per_record.attach = attach != 0;

                    per_record.id = stream.load_string(size as usize - 1)?;

                    script_config.records.push(per_record);
                }
                b"LUAD" => {
                    let size: u32 = stream.load()?;
                    let data: Vec<u8> = stream.load_seq(size)?;

                    if let Some(tag) = &last_tag {
                        match tag {
                            b"LUAF" => script_config.init_data = data,
                            b"LUAI" => {
                                let last_instance = script_config
                                    .instances
                                    .last_mut()
                                    .expect("No instance found in scriptConfig!");
                                last_instance.data = data;
                            }
                            b"LUAR" => {
                                script_config
                                    .records
                                    .last_mut()
                                    .expect("No record found in scriptConfig!")
                                    .data = data
                            }
                            _ => {
                                Reader::error(format!(
                                    "Unexpected Data Tag Preceding LUAD: {}::{}",
                                    this.tag_str(),
                                    tag.to_str_lossy()
                                ))?;
                            }
                        }
                    }
                }
                _ => {
                    Reader::error(format!("Unexpected Tag: {}::{}", this.tag_str(), tag.to_str_lossy()))?;
                }
            }

            last_tag = Some(tag);
        }

        if script_config != ScriptConfig::default() {
            this.scripts.push(std::mem::take(&mut script_config))
        }

        Ok(this)
    }
}

impl Save for ScriptConfigList {
    fn save(&self, stream: &mut Writer) -> io::Result<()> {
        stream.save(&self.flags)?;
        for script_config in &self.scripts {
            stream.save(b"LUAS")?;
            stream.save_string_without_null_terminator(&script_config.path)?;

            stream.save(b"LUAF")?;
            let length = 4 + (4 * script_config.types.len());
            stream.save(&(length as u32))?;
            stream.save(&script_config.flags)?;

            for attach_type in &script_config.types {
                stream.save_vec(attach_type.as_bytes().into())?;
            }

            if script_config.init_data.len() > 0 {
                stream.save(b"LUAD")?;
                stream.save(&script_config.init_data)?;
            }

            for record in &script_config.records {
                stream.save(b"LUAR")?;
                let length = record.id.len() as u32;
                stream.save(&(length + 1))?;
                stream.save_as::<u8>(record.attach)?;
                stream.save_bytes(&record.id[..].as_bytes())?;

                if record.data.len() > 0 {
                    stream.save(b"LUAD")?;
                    stream.save(&record.data)?;
                }
            }

            for instance in &script_config.instances {
                stream.save(b"LUAI")?;

                stream.save(&9)?;
                stream.save_as::<u8>(instance.attach)?;

                stream.save(&instance.ref_idx)?;
                stream.save(&instance.mast_idx)?;

                if instance.data.len() > 0 {
                    stream.save(b"LUAD")?;
                    stream.save(&instance.data)?;
                }
            }
        }

        Ok(())
    }
}
