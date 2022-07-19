mod common;

use nif::NiStream;

#[ignore]
#[test]
fn load_save() -> std::io::Result<()> {
    let (_dir, file) = common::create_temp_file();

    let src_path = "tests/assets/nif/test.nif";
    let dst_path = file.into_temp_path();

    let mut stream = NiStream::new();
    stream.load_path(&src_path)?;
    stream.save_path(&dst_path)?;

    let src_size = std::fs::metadata(&src_path)?.len();
    let dst_size = std::fs::metadata(&dst_path)?.len();
    assert_eq!(src_size, dst_size);

    Ok(())
}

#[ignore]
#[test]
fn load_save_dir() {
    use rayon::prelude::*;

    let path = "ignore/nif_dump/files";
    let files = common::collect_files(path, &["nif", "kf"]);

    files.into_par_iter().for_each(|src_path| {
        let (_dir, file) = common::create_temp_file();
        let dst_path = file.into_temp_path();

        let mut stream = NiStream::new();

        if stream.load_path(&src_path).is_err() {
            println!("FAILED TO LOAD: {}", src_path.display());
        }

        if stream.save_path(&dst_path).is_err() {
            println!("FAILED TO SAVE: {}", src_path.display());
        }
    });
}
