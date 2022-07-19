mod common;

use esp::Plugin;

#[test]
fn load_save() -> std::io::Result<()> {
    let (_dir, file) = common::create_temp_file();

    let src_path = "tests/assets/esp/test.esp";
    let dst_path = file.into_temp_path();

    let mut plugin = Plugin::new();
    plugin.load_path(&src_path)?;
    plugin.save_path(&dst_path)?;

    let src_bytes = std::fs::read(&src_path)?;
    let dst_bytes = std::fs::read(&dst_path)?;
    assert_eq!(src_bytes, dst_bytes);

    Ok(())
}

#[ignore]
#[test]
fn load_save_dir() {
    use rayon::prelude::*;

    let path = "ignore/esp_dump/plugins";
    let files = common::collect_files(path, &["esp", "esm"]);

    files.into_par_iter().for_each(|src_path| {
        let (_dir, file) = common::create_temp_file();
        let dst_path = file.into_temp_path();

        let mut plugin = Plugin::new();

        if plugin.load_path(&src_path).is_err() {
            println!("FAILED TO LOAD: {}", src_path.display());
        }

        if plugin.save_path(&dst_path).is_err() {
            println!("FAILED TO SAVE: {}", src_path.display());
        }
    });
}
