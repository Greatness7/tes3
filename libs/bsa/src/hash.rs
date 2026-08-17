/// The pair of 32-bit hashes that identifies a file within a BSA.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FileHash {
    pub low: u32,
    pub high: u32,
}

/// Hashes a path exactly as Morrowind does.
///
/// The path is used verbatim. Separators are not converted, case is not folded
/// beyond the engine's own ASCII rule, and nothing is trimmed or normalized.
///
/// The engine measures the path with `strlen`, so it would stop at an embedded null.
/// [`Builder`] rejects such names, so this hashes the whole slice instead of paying
/// for a scan on every lookup.
///
/// [`Builder`]: crate::Builder
pub fn hash_path(path: impl AsRef<[u8]>) -> FileHash {
    hash_bytes(path.as_ref())
}

fn hash_bytes(path: &[u8]) -> FileHash {
    let (head, tail) = path.split_at(path.len() / 2);

    let mut low = 0;
    for (i, &byte) in head.iter().enumerate() {
        low ^= extend(byte) << ((i % 4) * 8);
    }

    let mut high = 0;
    for (i, &byte) in tail.iter().enumerate() {
        let temp = extend(byte) << ((i % 4) * 8);
        // The engine derives the rotation from the shifted value, not the byte.
        high = (high ^ temp).rotate_right(temp & 31);
    }

    FileHash { low, high }
}

/// Lowercases ASCII uppercase, then sign extends.
///
/// The engine tests the byte with `isupper` and adds `0x20` when it passes, but loads
/// it with `movsx`, so `0x80..=0xff` become negative 32-bit values before shifting.
const fn extend(byte: u8) -> u32 {
    let byte = byte.to_ascii_lowercase();
    if byte >= 0x80 {
        0xFFFF_FF00 | byte as u32
    } else {
        byte as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_engine_vectors() {
        assert_eq!(
            hash_path("meshes\\m\\probe_journeyman_01.nif"),
            FileHash {
                low: 0x0002_0336,
                high: 0xBB50_0695
            }
        );
        assert_eq!(
            hash_path("textures\\vfx_poison03.dds"),
            FileHash {
                low: 0x7F7B_615D,
                high: 0xFEB7_B384
            }
        );
    }

    #[test]
    fn folds_ascii_case() {
        assert_eq!(hash_path("MESHES\\Test.NIF"), hash_path("meshes\\test.nif"));
    }

    #[test]
    fn does_not_normalize_separators() {
        assert_ne!(hash_path("meshes/test.nif"), hash_path("meshes\\test.nif"));
    }

    #[test]
    fn sign_extends_high_bytes() {
        assert_eq!(
            hash_path([0x80]),
            FileHash {
                low: 0,
                high: 0xFFFF_FF80
            }
        );
        assert_eq!(
            hash_path([0xE9]),
            FileHash {
                low: 0,
                high: 0xF4FF_FFFF
            }
        );
        assert_eq!(
            hash_path([0xFF]),
            FileHash {
                low: 0,
                high: 0xFFFF_FFFF
            }
        );
    }

    #[test]
    fn hashes_empty_path_to_zero() {
        assert_eq!(hash_path(""), FileHash { low: 0, high: 0 });
    }

    /// The field declaration order above is load bearing, not cosmetic: the derived `Ord`
    /// is what both the builder's sort and the parser's non-decreasing check run on, and
    /// it has to match the engine's own low-then-high comparison. Swapping the two fields
    /// leaves those two in agreement with each other, so almost nothing else notices.
    #[test]
    fn orders_by_low_then_high() {
        let a = FileHash { low: 1, high: u32::MAX };
        let b = FileHash { low: 2, high: 0 };
        assert!(a < b);
    }
}
