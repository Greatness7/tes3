use std::io;

use bsa::{Archive, Builder, FileHash, hash_path};

const FIXTURE: &str = "tests/assets/test.bsa";

const PROBE: &[u8] = b"meshes\\m\\probe_journeyman_01.nif";
const POISON: &[u8] = b"textures\\vfx_poison03.dds";

const fn is_send_sync<T: Send + Sync>() {}

#[test]
fn archive_is_send_and_sync() {
    is_send_sync::<Archive<'static>>();
}

#[test]
fn loads_fixture_from_path() -> io::Result<()> {
    let archive = Archive::from_path(FIXTURE)?;

    assert_eq!(archive.len(), 2);
    assert!(archive.has_names());

    // Stored in hash order, which is not the order they were written in.
    let entries: Vec<_> = archive.entries().collect();
    assert_eq!(entries[0].name().unwrap(), PROBE);
    assert_eq!(entries[1].name().unwrap(), POISON);

    assert_eq!(
        entries[0].hash(),
        FileHash {
            low: 0x0002_0336,
            high: 0xBB50_0695
        }
    );
    assert_eq!(
        entries[1].hash(),
        FileHash {
            low: 0x7F7B_615D,
            high: 0xFEB7_B384
        }
    );

    assert_eq!(entries[0].as_bytes().len(), 52);
    assert_eq!(entries[1].as_bytes().len(), 136);
    assert!(entries[0].as_bytes().starts_with(b"NetImmerse File Format"));
    assert!(entries[1].as_bytes().starts_with(b"DDS "));

    Ok(())
}

#[test]
fn loads_fixture_from_borrowed_and_owned_bytes() -> io::Result<()> {
    let bytes = std::fs::read(FIXTURE)?;

    let borrowed = Archive::from_slice(&bytes)?;
    let owned = Archive::from_bytes(bytes.clone().into_boxed_slice())?;

    for archive in [&borrowed, &owned] {
        assert_eq!(archive.len(), 2);
        assert_eq!(archive.get(PROBE).unwrap().as_bytes().len(), 52);
    }

    Ok(())
}

#[test]
fn looks_up_by_name_and_hash() -> io::Result<()> {
    let archive = Archive::from_path(FIXTURE)?;

    let entry = archive.get(POISON).expect("entry should be present");
    assert_eq!(entry.name().unwrap(), POISON);

    let by_hash = archive.get_by_hash(hash_path(POISON)).expect("entry should be present");
    assert_eq!(by_hash.as_bytes(), entry.as_bytes());

    // The engine folds ASCII case but nothing else.
    assert!(archive.get(b"TEXTURES\\VFX_POISON03.DDS").is_some());
    assert!(archive.get(b"textures/vfx_poison03.dds").is_none());
    assert!(archive.get(b"textures\\vfx_absent.dds").is_none());

    Ok(())
}

#[test]
fn round_trips_fixture_content() -> io::Result<()> {
    let archive = Archive::from_path(FIXTURE)?;

    let mut builder = Builder::new();
    for entry in archive.entries() {
        builder.insert(entry.name().unwrap().to_vec(), entry.as_bytes())?;
    }

    let bytes = builder.save_bytes()?;
    let rebuilt = Archive::from_slice(&bytes)?;

    assert_eq!(rebuilt.len(), archive.len());
    for (before, after) in archive.entries().zip(rebuilt.entries()) {
        assert_eq!(before.name(), after.name());
        assert_eq!(before.hash(), after.hash());
        assert_eq!(before.as_bytes(), after.as_bytes());
    }

    assert_eq!(bytes, std::fs::read(FIXTURE)?);

    Ok(())
}

#[test]
fn writes_deterministically_regardless_of_insertion_order() -> io::Result<()> {
    let files: [(&[u8], &[u8]); 3] = [
        (b"meshes\\b.nif", b"bbb"),
        (b"textures\\a.dds", b"aaaa"),
        (b"meshes\\c.nif", b""),
    ];

    let mut forward = Builder::new();
    for (name, data) in files {
        forward.insert(name.to_vec(), data)?;
    }

    let mut reverse = Builder::new();
    for (name, data) in files.into_iter().rev() {
        reverse.insert(name.to_vec(), data)?;
    }

    let bytes = forward.save_bytes()?;
    assert_eq!(bytes, reverse.save_bytes()?);

    // Writing an archive that was just written must reproduce it byte for byte.
    let archive = Archive::from_slice(&bytes)?;
    let mut again = Builder::new();
    for entry in archive.entries() {
        again.insert(entry.name().unwrap().to_vec(), entry.as_bytes())?;
    }
    assert_eq!(again.save_bytes()?, bytes);

    // A zero length file is legal and must survive the trip.
    assert_eq!(archive.get(b"meshes\\c.nif").unwrap().as_bytes(), b"");

    Ok(())
}

#[test]
fn save_path_matches_save_bytes() -> io::Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("out.bsa");

    let mut builder = Builder::new();
    builder.insert(b"meshes\\test.nif".to_vec(), b"payload".as_slice())?;
    builder.save_path(&path)?;

    assert_eq!(std::fs::read(&path)?, builder.save_bytes()?);
    assert_eq!(
        Archive::from_path(&path)?.get(b"meshes\\test.nif").unwrap().as_bytes(),
        b"payload"
    );

    Ok(())
}

#[test]
fn builder_rejects_bad_input() {
    let mut builder = Builder::new();

    assert!(builder.save_bytes().is_err(), "an empty archive must be rejected");

    assert!(builder.insert(b"has\0null".to_vec(), b"".as_slice()).is_err());

    builder.insert(b"meshes\\test.nif".to_vec(), b"".as_slice()).unwrap();
    // Same hash, since the engine folds ASCII case. Entries are only ordered when the
    // archive is written, so the collision is caught there rather than at insert.
    builder.insert(b"MESHES\\TEST.NIF".to_vec(), b"".as_slice()).unwrap();
    assert_eq!(builder.len(), 2);
    assert!(builder.save_bytes().is_err(), "colliding hashes must be rejected");
}

#[test]
fn builder_resorts_after_a_later_insert() -> io::Result<()> {
    let mut builder = Builder::new();
    builder.insert(b"meshes\\b.nif".to_vec(), b"bbb".as_slice())?;
    let first = builder.save_bytes()?;

    // Writing twice with nothing staged in between must not re-order anything.
    assert_eq!(builder.save_bytes()?, first);

    // A name that sorts before the existing one, to prove the write re-sorts.
    builder.insert(b"meshes\\a.nif".to_vec(), b"aaaa".as_slice())?;
    let bytes = builder.save_bytes()?;
    let archive = Archive::from_slice(&bytes)?;

    assert_eq!(archive.len(), 2);
    assert_eq!(archive.get(b"meshes\\a.nif").unwrap().as_bytes(), b"aaaa");
    assert_eq!(archive.get(b"meshes\\b.nif").unwrap().as_bytes(), b"bbb");

    Ok(())
}

#[test]
fn save_path_leaves_the_destination_alone_when_the_build_fails() -> io::Result<()> {
    let directory = tempfile::tempdir()?;
    let path = directory.path().join("out.bsa");
    std::fs::write(&path, b"existing contents")?;

    // Empty, so the layout is rejected before the destination is opened.
    let mut builder = Builder::new();
    assert!(builder.save_path(&path).is_err());
    assert_eq!(std::fs::read(&path)?, b"existing contents");

    Ok(())
}

#[test]
fn finds_every_entry_at_every_size() -> io::Result<()> {
    // Lookup binary searches the archive's own hash table rather than a std slice, so
    // sweep the sizes where an off-by-one in that search would show up.
    let names: Vec<_> = (0..64).map(|i| format!("meshes\\file_{i:02}.nif")).collect();

    for count in 1..=names.len() {
        let mut builder = Builder::new();
        for name in &names[..count] {
            builder.insert(name.as_bytes().to_vec(), name.as_bytes())?;
        }
        let bytes = builder.save_bytes()?;
        let archive = Archive::from_slice(&bytes)?;

        assert_eq!(archive.len(), count);
        for name in &names[..count] {
            let entry = archive.get(name).unwrap_or_else(|| panic!("{name} missing at {count}"));
            assert_eq!(entry.as_bytes(), name.as_bytes());
            assert_eq!(entry.name().unwrap(), name.as_bytes());
            assert_eq!(entry.hash(), hash_path(name));
        }
        for name in &names[count..] {
            assert!(archive.get(name).is_none(), "{name} present at {count}");
        }
        assert!(archive.get(b"meshes\\absent.nif").is_none());
    }

    Ok(())
}

#[test]
fn reads_the_engines_nameless_variant() -> io::Result<()> {
    let bytes = nameless_archive(&[(hash_path("a"), b"first"), (hash_path("zzz"), b"second")]);
    let archive = Archive::from_slice(&bytes)?;

    assert!(!archive.has_names());
    assert_eq!(archive.len(), 2);

    let entry = archive.get("a").expect("entry should be present");
    assert_eq!(entry.name(), None);
    assert_eq!(entry.as_bytes(), b"first");
    assert_eq!(archive.get_by_hash(hash_path("zzz")).unwrap().as_bytes(), b"second");

    Ok(())
}

#[test]
fn counts_nameless_duplicate_hashes() -> io::Result<()> {
    // No names, so nothing can desync from a stored hash: this isolates the duplicate
    // counter from the mismatch counter.
    let hash = hash_path("a");
    let bytes = nameless_archive(&[(hash, b"first"), (hash, b"second")]);
    let archive = Archive::from_slice(&bytes)?;

    assert_eq!(archive.anomalies().duplicate_hashes, 1);
    assert_eq!(archive.anomalies().mismatched_names, 0);
    assert_eq!(archive.len(), 2, "both entries stay iterable");

    // Lower bound settles on the first member of the run.
    assert_eq!(archive.get_by_hash(hash).unwrap().as_bytes(), b"first");

    Ok(())
}

/// Builds a name-less archive: `hash_offset` is exactly `8 * N` and the name offset
/// and name table sections are absent. Entries must already be in hash order.
fn nameless_archive(entries: &[(FileHash, &[u8])]) -> Vec<u8> {
    let count = u32::try_from(entries.len()).unwrap();
    let mut bytes = Vec::new();

    bytes.extend_from_slice(&0x0000_0100u32.to_le_bytes());
    bytes.extend_from_slice(&(8 * count).to_le_bytes());
    bytes.extend_from_slice(&count.to_le_bytes());

    let mut offset = 0u32;
    for (_, data) in entries {
        let size = u32::try_from(data.len()).unwrap();
        bytes.extend_from_slice(&size.to_le_bytes());
        bytes.extend_from_slice(&offset.to_le_bytes());
        offset += size;
    }
    for (hash, _) in entries {
        bytes.extend_from_slice(&hash.low.to_le_bytes());
        bytes.extend_from_slice(&hash.high.to_le_bytes());
    }
    for (_, data) in entries {
        bytes.extend_from_slice(data);
    }

    bytes
}
