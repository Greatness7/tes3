use std::cmp::Ordering;
use std::io;
use std::path::Path;

use bstr::BStr;
use memmap2::Mmap;

use crate::hash::{FileHash, hash_path};
use crate::{HEADER_SIZE, VERSION, invalid_data, invalid_data_error};

/// The byte source an [`Archive`] reads its names and file data from.
enum Storage<'a> {
    Borrowed(&'a [u8]),
    Owned(Box<[u8]>),
    Mapped(Mmap),
}

impl Storage<'_> {
    fn as_bytes(&self) -> &[u8] {
        match self {
            Storage::Borrowed(bytes) => bytes,
            Storage::Owned(bytes) => bytes,
            Storage::Mapped(mmap) => mmap,
        }
    }
}

impl std::fmt::Debug for Storage<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let kind = match self {
            Storage::Borrowed(_) => "Borrowed",
            Storage::Owned(_) => "Owned",
            Storage::Mapped(_) => "Mapped",
        };
        f.debug_tuple(kind).field(&self.as_bytes().len()).finish()
    }
}

/// Validated ranges into the archive's byte source.
///
/// Ranges rather than references, so the archive is not self referential. The parser
/// rejects any source longer than `u32::MAX`, so 32-bit offsets always suffice.
///
/// Hashes are absent: the archive already stores them contiguously and sorted, so
/// lookup searches them in place rather than retaining a copy.
#[derive(Clone, Copy, Debug)]
struct EntryMeta {
    name_start: u32,
    name_end: u32,
    data_start: u32,
    data_end: u32,
}

/// An immutable, memory mapped or borrowed BSA.
///
/// Parsing is a single checked pass; lookups afterwards are allocation free.
pub struct Archive<'a> {
    storage: Storage<'a>,
    entries: Box<[EntryMeta]>,
    hashes: u32,
    has_names: bool,
    anomalies: ArchiveAnomalies,
}

/// Non-fatal defects found while parsing an [`Archive`].
///
/// Both are conditions the engine loads without complaint, leaving the affected files
/// unreachable, so the parser records them rather than rejecting the archive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ArchiveAnomalies {
    /// Entries whose stored name does not hash to their stored hash.
    ///
    /// The engine hashes the name it is handed, so such a file is unreachable under its
    /// own stored name there too. The `OpenMW` writer produces these: its hash routine
    /// does not fold ASCII case.
    pub mismatched_names: u32,

    /// Entries sharing a stored hash with the entry before them.
    ///
    /// Only one member of such a run is reachable; see [`Archive::get_by_hash`].
    pub duplicate_hashes: u32,
}

/// Prints a summary. A derived implementation would print the whole byte source.
impl std::fmt::Debug for Archive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Archive")
            .field("storage", &self.storage)
            .field("len", &self.entries.len())
            .field("has_names", &self.has_names)
            .field("anomalies", &self.anomalies)
            .finish()
    }
}

impl Archive<'static> {
    /// Opens a BSA as a read-only memory map.
    ///
    /// # Mapping hazard
    ///
    /// The backing file must not be modified or truncated while the returned `Archive`,
    /// or any [`Entry`] borrowed from it, is alive. This is undefined behavior rather
    /// than merely a stale read: the mapping aliases the file's pages directly. Installed
    /// game archives are immutable in practice, and every other known BSA library uses the
    /// same contract. Use [`Archive::from_bytes`] for a file that cannot be trusted to
    /// stay put.
    pub fn from_path(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = std::fs::File::open(path)?;
        // SAFETY: sound only while the backing file is undisturbed, which is the caller's
        // documented contract above rather than anything verifiable here.
        let mmap = unsafe { Mmap::map(&file)? };
        Self::build(Storage::Mapped(mmap))
    }

    /// Takes ownership of an in-memory BSA.
    pub fn from_bytes(bytes: Box<[u8]>) -> io::Result<Self> {
        Self::build(Storage::Owned(bytes))
    }
}

impl<'a> Archive<'a> {
    /// Borrows an in-memory BSA.
    pub fn from_slice(bytes: &'a [u8]) -> io::Result<Self> {
        Self::build(Storage::Borrowed(bytes))
    }

    fn build(storage: Storage<'a>) -> io::Result<Self> {
        let parsed = parse(storage.as_bytes())?;
        Ok(Self {
            storage,
            entries: parsed.entries,
            hashes: parsed.hashes,
            has_names: parsed.has_names,
            anomalies: parsed.anomalies,
        })
    }

    /// Returns the number of files in the archive, which is always at least one.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Always `false`; the engine rejects archives with no files, and so does the parser.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Whether the archive stores file names.
    ///
    /// The engine also reads a variant that stores only hashes. Those archives support
    /// lookup by name or hash, but their entries cannot report a name.
    pub const fn has_names(&self) -> bool {
        self.has_names
    }

    /// Non-fatal defects found while parsing; all zero for a well-formed archive.
    pub const fn anomalies(&self) -> ArchiveAnomalies {
        self.anomalies
    }

    /// Iterates every file in stored hash order.
    pub fn entries(&self) -> impl DoubleEndedIterator<Item = Entry<'_>> + ExactSizeIterator {
        let bytes = self.storage.as_bytes();
        let hashes = self.hashes;
        let has_names = self.has_names;
        self.entries
            .iter()
            .enumerate()
            .map(move |(index, meta)| Entry::new(bytes, meta, hash_at(bytes, hashes, index), has_names))
    }

    /// Looks up a file by path, hashing the bytes as the engine would.
    ///
    /// The path is not normalized. Callers that accept `/` or mixed separators must
    /// convert them before calling.
    pub fn get(&self, path: impl AsRef<[u8]>) -> Option<Entry<'_>> {
        self.get_by_hash(hash_path(path))
    }

    /// Looks up a file by its hash.
    ///
    /// When several entries share a hash the first is returned and the rest are
    /// unreachable. The engine also serves exactly one member of such a run, but whichever
    /// its own midpoint probe lands on, so which file wins is not portable.
    pub fn get_by_hash(&self, hash: FileHash) -> Option<Entry<'_>> {
        let bytes = self.storage.as_bytes();
        debug_assert!(!self.entries.is_empty(), "the parser rejects an empty archive");

        // Searched in place, in the archive's own sorted table. Lower bound rather than
        // `slice::binary_search_by`'s shape, so an equal run settles on its first member.
        let mut base = 0;
        let mut size = self.entries.len();
        while size > 1 {
            let half = size / 2;
            let middle = base + half;
            base = if hash_at(bytes, self.hashes, middle) < hash {
                middle
            } else {
                base
            };
            size -= half;
        }

        // `base` is now the last entry below `hash`, so the candidate is the one after
        // it. Unless nothing was below, in which case `base` is itself the candidate.
        let probe = hash_at(bytes, self.hashes, base);
        let index = if probe < hash {
            let next = base + 1;
            if next == self.entries.len() || hash_at(bytes, self.hashes, next) != hash {
                return None;
            }
            next
        } else {
            if probe != hash {
                return None;
            }
            base
        };

        Some(Entry::new(bytes, &self.entries[index], hash, self.has_names))
    }
}

/// A borrowed view of one file in an [`Archive`].
#[derive(Clone, Copy, Debug)]
pub struct Entry<'a> {
    hash: FileHash,
    name: Option<&'a BStr>,
    data: &'a [u8],
}

impl<'a> Entry<'a> {
    fn new(bytes: &'a [u8], meta: &EntryMeta, hash: FileHash, has_names: bool) -> Self {
        let name = has_names.then(|| BStr::new(&bytes[widen(meta.name_start)..widen(meta.name_end)]));
        Self {
            hash,
            name,
            data: &bytes[widen(meta.data_start)..widen(meta.data_end)],
        }
    }

    /// The file's stored hash.
    pub const fn hash(&self) -> FileHash {
        self.hash
    }

    /// The file's raw stored name, or `None` for an archive without names.
    ///
    /// Names are exposed exactly as stored and are never decoded or transcoded.
    pub const fn name(&self) -> Option<&'a BStr> {
        self.name
    }

    /// The file's contents, borrowed from the archive.
    pub const fn as_bytes(&self) -> &'a [u8] {
        self.data
    }
}

/// The result of one validating pass over the header sections.
struct Parsed {
    entries: Box<[EntryMeta]>,
    /// Offset of the archive's own hash table, which lookup searches in place.
    hashes: u32,
    has_names: bool,
    anomalies: ArchiveAnomalies,
}

/// Reads and validates the header sections.
///
/// Zero length files, overlapping payloads, payloads stored out of hash order, and
/// trailing bytes are all accepted because the engine can read them, as are the defects
/// counted in [`ArchiveAnomalies`]. Anything that would leave the archive unsearchable,
/// or that cannot be bounds checked, is rejected.
fn parse(bytes: &[u8]) -> io::Result<Parsed> {
    let Ok(total) = u32::try_from(bytes.len()) else {
        return invalid_data("BSA is too large for the engine's 32-bit file offsets");
    };
    let total = u64::from(total);

    if total < HEADER_SIZE {
        return invalid_data("BSA is smaller than its header");
    }

    // The bound above proves the first twelve bytes are present.
    if read_u32(bytes, 0) != VERSION {
        return invalid_data("BSA has an unsupported version");
    }
    let hash_offset = u64::from(read_u32(bytes, 4));
    let count = read_u32(bytes, 8);

    if count == 0 {
        return invalid_data("BSA contains no files");
    }
    let n = u64::from(count);

    // A named archive stores 4 offset bytes and the name bytes between the records and
    // the hashes; the engine's name-less variant places the hashes directly after them.
    let has_names = if hash_offset == 8 * n {
        false
    } else if hash_offset >= 12 * n {
        true
    } else {
        return invalid_data("BSA hash table offset does not match either layout");
    };

    let records_start = HEADER_SIZE;
    let name_offsets = records_start + 8 * n;
    let table_start = name_offsets + 4 * n;
    let table_len = if has_names { hash_offset - 12 * n } else { 0 };
    let hashes_start = HEADER_SIZE + hash_offset;
    let data_start = hashes_start + 8 * n;

    // Every preceding section ends at or before the payload area, so this one bound
    // covers the records, the name offsets, and the name table as well.
    if data_start > total {
        return invalid_data("BSA header sections extend past the end of the file");
    }
    let table_end = index(table_start + table_len)?;
    // Lookup reads hashes straight out of the source, so their offset has to be kept.
    let hashes = narrow(index(hashes_start)?)?;

    let mut entries: Vec<EntryMeta> = Vec::with_capacity(index(n)?);
    let mut anomalies = ArchiveAnomalies::default();
    let mut previous = None;

    for i in 0..n {
        let record = index(records_start + 8 * i)?;
        let size = u64::from(read_u32(bytes, record));
        let offset = u64::from(read_u32(bytes, record + 4));

        // The engine rebases every record offset onto the start of the payload area.
        let begin = data_start + offset;
        let end = begin + size;
        if end > total {
            return invalid_data("BSA file data extends past the end of the file");
        }

        let hash = hash_at(bytes, hashes, index(i)?);

        // Lookup is a binary search over `(low, high)`. A descending pair would leave
        // files the search cannot reach at all, so it stays fatal; an equal pair only
        // makes the later member unreachable, which the engine tolerates too.
        if let Some(previous) = previous.replace(hash) {
            match hash.cmp(&previous) {
                Ordering::Less => return invalid_data("BSA hashes are not in non-decreasing order"),
                Ordering::Equal => anomalies.duplicate_hashes += 1,
                Ordering::Greater => {}
            }
        }

        let (name_start, name_end) = if has_names {
            let offset = u64::from(read_u32(bytes, index(name_offsets + 4 * i)?));
            if offset >= table_len {
                return invalid_data("BSA file name offset is outside the name table");
            }
            let start = index(table_start + offset)?;
            let Some(len) = bytes[start..table_end].iter().position(|&byte| byte == 0) else {
                return invalid_data("BSA file name is not null terminated");
            };
            let end = start + len;
            // Not fatal: the engine hashes the name it is handed, so a stored name that
            // disagrees with its stored hash is simply never found.
            if hash_path(&bytes[start..end]) != hash {
                anomalies.mismatched_names += 1;
            }
            (start, end)
        } else {
            (0, 0)
        };

        entries.push(EntryMeta {
            name_start: narrow(name_start)?,
            name_end: narrow(name_end)?,
            data_start: narrow(index(begin)?)?,
            data_end: narrow(index(end)?)?,
        });
    }

    Ok(Parsed {
        entries: entries.into_boxed_slice(),
        hashes,
        has_names,
        anomalies,
    })
}

/// Reads the stored hash of entry `index`. The caller must have bounds checked `index`.
fn hash_at(bytes: &[u8], hashes: u32, index: usize) -> FileHash {
    let position = widen(hashes) + index * 8;
    let hash = &bytes[position..position + 8];

    let (&[low, high], []) = hash.as_chunks::<4>() else {
        unreachable!();
    };

    FileHash {
        low: u32::from_le_bytes(low),
        high: u32::from_le_bytes(high),
    }
}

/// Narrows a bounds checked file offset to an index.
#[inline]
fn index(offset: u64) -> io::Result<usize> {
    usize::try_from(offset).map_err(|_| invalid_data_error("BSA offset does not fit in the address space"))
}

/// Narrows a bounds checked index for storage in an [`EntryMeta`].
#[inline]
fn narrow(offset: usize) -> io::Result<u32> {
    u32::try_from(offset).map_err(|_| invalid_data_error("BSA offset does not fit in 32 bits"))
}

/// Widens a stored offset. Lossless, since the parser caps the source at `u32::MAX`.
#[inline]
const fn widen(offset: u32) -> usize {
    offset as usize
}

/// Reads a little endian `u32`. The caller must have bounds checked the offset.
#[inline]
fn read_u32(bytes: &[u8], offset: usize) -> u32 {
    u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap())
}
