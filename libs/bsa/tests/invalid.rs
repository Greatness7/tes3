use std::io::ErrorKind;

use bsa::{Archive, Builder};

/// A two file archive with a fully known layout:
///
/// ```text
///  0 version        12 record 0      28 name offset 0   36 "a.nif\0"
///  4 hash offset    20 record 1      32 name offset 1   42 "b.nif\0"
///  8 file count                                         48 hashes
///                                                       64 payloads
/// ```
fn valid() -> Vec<u8> {
    let mut builder = Builder::new();
    builder.insert(b"a.nif".to_vec(), b"aaaa".as_slice()).unwrap();
    builder.insert(b"b.nif".to_vec(), b"bbbb".as_slice()).unwrap();

    let bytes = builder.save_bytes().unwrap();
    assert_eq!(bytes.len(), 72, "the offsets below assume this layout");
    bytes
}

fn patch(offset: usize, value: u32) -> Vec<u8> {
    let mut bytes = valid();
    bytes[offset..offset + 4].copy_from_slice(&value.to_le_bytes());
    bytes
}

#[track_caller]
fn assert_rejected(bytes: &[u8]) {
    match Archive::from_slice(bytes) {
        Ok(_) => panic!("archive should have been rejected"),
        Err(error) => assert_eq!(error.kind(), ErrorKind::InvalidData),
    }
}

#[test]
fn accepts_the_unmodified_control() {
    // Every rejection below patches this fixture, so it has to parse cleanly and report
    // nothing, or those tests could be passing for a reason other than the patch.
    let bytes = valid();
    let archive = Archive::from_slice(&bytes).expect("the control must parse");
    assert_eq!(archive.anomalies(), bsa::ArchiveAnomalies::default());
}

#[test]
fn rejects_truncated_header() {
    assert_rejected(&[]);
    assert_rejected(&valid()[..8]);
}

#[test]
fn rejects_unsupported_version() {
    assert_rejected(&patch(0, 0x0000_0101));
    assert_rejected(&patch(0, 0));
}

#[test]
fn rejects_empty_archive() {
    // The engine's loader discards an archive that reports no files.
    assert_rejected(&patch(8, 0));
}

#[test]
fn rejects_hash_offset_between_the_two_layouts() {
    // Neither `8 * N` (name-less) nor at least `12 * N` (named).
    assert_rejected(&patch(4, 20));
    assert_rejected(&patch(4, 23));
}

#[test]
fn rejects_out_of_bounds_sections() {
    assert_rejected(&patch(4, u32::MAX));
    assert_rejected(&patch(4, 1000));
}

#[test]
fn rejects_name_offset_outside_the_table() {
    // The table is 12 bytes, so 12 is one past its end.
    assert_rejected(&patch(28, 12));
    assert_rejected(&patch(32, u32::MAX));
}

#[test]
fn rejects_unterminated_name() {
    let mut bytes = valid();
    bytes[47] = b'x'; // the null ending "b.nif"
    assert_rejected(&bytes);
}

#[test]
fn rejects_out_of_bounds_payload() {
    assert_rejected(&patch(12, u32::MAX)); // size of record 0
    assert_rejected(&patch(16, 1000)); // offset of record 0
}

#[test]
fn rejects_descending_hashes() {
    // No per-entry recovery exists: a binary search over an unsorted table is meaningless.
    let valid = valid();
    let mut swapped = valid.clone();
    swapped[48..56].copy_from_slice(&valid[56..64]);
    swapped[56..64].copy_from_slice(&valid[48..56]);
    assert_rejected(&swapped);
}

#[test]
fn counts_duplicate_hashes_instead_of_rejecting_them() {
    // The engine loads such an archive and serves one member of the run, so this does
    // too. Overwriting a stored hash also desyncs that entry's name, so the fixture
    // trips both counters at once.
    let valid = valid();
    let mut bytes = valid.clone();
    bytes[56..64].copy_from_slice(&valid[48..56]);

    let archive = Archive::from_slice(&bytes).expect("duplicates are readable");
    assert_eq!(archive.anomalies().duplicate_hashes, 1);
    assert_eq!(archive.anomalies().mismatched_names, 1);
    assert_eq!(archive.len(), 2, "both entries stay iterable");

    // Lookup settles on the first member of the run; the second is unreachable.
    let entry = archive.get(b"a.nif").expect("the first of the run is reachable");
    assert_eq!(entry.as_bytes(), b"aaaa");
    assert_eq!(entry.name().unwrap(), b"a.nif".as_slice());
    assert!(archive.get(b"b.nif").is_none(), "b.nif no longer hashes to its own entry");
}

#[test]
fn counts_a_name_that_does_not_match_its_hash() {
    // OpenMW writes these: its hash routine does not fold ASCII case, so the stored name
    // and stored hash disagree and the engine silently never finds the file.
    let mut bytes = valid();
    bytes[36] = b'c'; // "a.nif" -> "c.nif", leaving the stored hash behind

    let archive = Archive::from_slice(&bytes).expect("a mismatched name is readable");
    assert_eq!(archive.anomalies().mismatched_names, 1);
    assert_eq!(archive.anomalies().duplicate_hashes, 0);

    // Iteration still exposes it, so an extraction tool can reach the payload.
    let entries: Vec<_> = archive.entries().collect();
    assert_eq!(entries[0].name().unwrap(), b"c.nif".as_slice());
    assert_eq!(entries[0].as_bytes(), b"aaaa");

    // It is unreachable under its own stored name, exactly as in the engine.
    assert!(archive.get(b"c.nif").is_none());
    assert!(archive.get(b"b.nif").is_some(), "the other entry is unaffected");

    // The stored hash is still a real hash of "a.nif", so that name reaches the entry and
    // gets back a different stored name. The Construction Set reports this as "Hash map
    // collision between two files"; Morrowind's copy of the check is compiled in but
    // disabled, so it serves the payload silently.
    let found = archive.get(b"a.nif").expect("the stored hash is unchanged");
    assert_eq!(found.name().unwrap(), b"c.nif".as_slice());
}

#[test]
fn accepts_what_the_engine_can_still_read() {
    // Trailing bytes.
    let mut trailing = valid();
    trailing.extend_from_slice(b"junk");
    assert!(Archive::from_slice(&trailing).is_ok());

    // Overlapping payloads: point both records at the same offset.
    let overlapping = patch(24, 0);
    let archive = Archive::from_slice(&overlapping).expect("overlap is readable");
    assert_eq!(archive.get(b"b.nif").unwrap().as_bytes(), b"aaaa");

    // A zero length file.
    let empty = patch(12, 0);
    let archive = Archive::from_slice(&empty).expect("zero length is readable");
    assert_eq!(archive.get(b"a.nif").unwrap().as_bytes(), b"");
}
