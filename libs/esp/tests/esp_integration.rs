use tempfile::{NamedTempFile, TempDir};

use esp::Plugin;

fn create_temp_file() -> (TempDir, NamedTempFile) {
    let dir = TempDir::new().unwrap();
    let file = NamedTempFile::new_in(&dir).unwrap();
    (dir, file)
}

#[test]
fn load_save() -> std::io::Result<()> {
    let (_dir, file) = create_temp_file();

    let src_path = "tests/assets/all_types.esp";
    let dst_path = file.into_temp_path();

    let mut plugin = Plugin::new();
    plugin.load_path(src_path)?;
    plugin.save_path(&dst_path)?;

    let src_bytes = std::fs::read(src_path)?;
    let dst_bytes = std::fs::read(&dst_path)?;
    assert_eq!(src_bytes, dst_bytes);

    Ok(())
}

#[test]
#[cfg(feature = "serde")]
fn load_save_json() -> std::io::Result<()> {
    let src_path = "tests/assets/all_types.esp";

    let mut plugin1 = Plugin::new();
    plugin1.load_path(src_path)?;

    let json_objects = serde_json::to_string(&plugin1.objects).unwrap();

    let mut plugin2 = Plugin::new();
    plugin2.objects = serde_json::from_str(&json_objects).unwrap();

    let plugin1_bytes = plugin1.save_bytes()?;
    let plugin2_bytes = plugin2.save_bytes()?;
    assert_eq!(plugin1_bytes, plugin2_bytes);

    Ok(())
}

/// A plugin of a Russian release: its strings in Windows-1251, read and written in it with
/// the `_with_encoding` calls; Windows-1252 (the default) cannot write them.
#[test]
fn a_code_page_other_than_windows_1252() -> std::io::Result<()> {
    use esp::{Header, MiscItem, TES3Object};
    let header = Header {
        author: String::from("Сергей").into(),
        ..Default::default()
    };
    let item = MiscItem {
        id: "misc_lamp".into(),
        name: "Лампа".into(),
        ..Default::default()
    };
    let mut plugin = Plugin {
        objects: vec![TES3Object::Header(header), TES3Object::MiscItem(item)],
    };
    let cyrillic = bytes_io::encoding_rs::WINDOWS_1251;
    let bytes = plugin.save_bytes_with_encoding(cyrillic)?;
    // "Лампа" in Windows-1251, as the Russian game has it.
    assert!(bytes.windows(5).any(|w| w == [0xCB, 0xE0, 0xEC, 0xEF, 0xE0]));
    assert!(plugin.save_bytes().is_err(), "Windows-1252 has no Cyrillic");

    let mut back = Plugin::new();
    back.load_bytes_with_encoding(&bytes, cyrillic)?;
    let names: Vec<String> = back.objects_of_type::<MiscItem>().map(|m| m.name.clone()).collect();
    assert_eq!(names, ["Лампа"]);
    assert_eq!(&*back.header().unwrap().author, "Сергей");
    // Read as Windows-1252, the same bytes are other letters, and write back as they were.
    let mut western = Plugin::new();
    western.load_bytes(&bytes)?;
    assert_ne!(western.objects_of_type::<MiscItem>().next().unwrap().name, "Лампа");
    assert_eq!(western.save_bytes()?, bytes);

    let (_dir, file) = create_temp_file();
    let path = file.into_temp_path();
    plugin.save_path_with_encoding(&path, cyrillic)?;
    assert_eq!(
        Plugin::from_path_with_encoding(&path, cyrillic)?
            .objects_of_type::<MiscItem>()
            .next()
            .unwrap()
            .name,
        "Лампа"
    );
    assert_eq!(&*Header::from_path_with_encoding(&path, cyrillic)?.author, "Сергей");
    Ok(())
}
