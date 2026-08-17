use std::borrow::Cow;
use std::io::{self, BufWriter, Write};
use std::path::Path;

use bstr::BString;

use crate::hash::{FileHash, hash_path};
use crate::{HEADER_SIZE, VERSION, invalid_data, invalid_data_error};

#[derive(Clone, Debug)]
struct BuilderEntry<'a> {
    hash: FileHash,
    name: BString,
    data: Cow<'a, [u8]>,
}

/// Precomputed section sizes for one write.
#[derive(Clone, Copy, Debug)]
struct Layout {
    count: u32,
    hash_offset: u32,
    total: u32,
}

/// Builds a BSA.
#[derive(Clone, Debug, Default)]
pub struct Builder<'a> {
    /// Files staged for writing.
    entries: Vec<BuilderEntry<'a>>,
    /// Whether `entries` is already in hash order. Cleared by every [`Builder::insert`].
    sorted: bool,
}

impl<'a> Builder<'a> {
    /// Creates an empty builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the number of files staged for writing.
    pub const fn len(&self) -> usize {
        self.entries.len()
    }

    /// Whether no files have been staged. Writing in this state is an error.
    pub const fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Stages a file under the given name, borrowing or taking ownership of its data.
    ///
    /// The name is hashed and stored verbatim; separators and case are not normalized.
    /// Fails on a name containing a null and on data too large to address. A name whose
    /// hash collides with one already staged is rejected when the archive is written,
    /// not here, since entries are only ordered at that point.
    pub fn insert(&mut self, name: impl Into<BString>, data: impl Into<Cow<'a, [u8]>>) -> io::Result<()> {
        let name = name.into();
        if name.contains(&0) {
            return invalid_data("BSA file names cannot contain a null");
        }
        if u32::try_from(name.len()).is_err() {
            return invalid_data("BSA file name is too long");
        }

        let data = data.into();
        if u32::try_from(data.len()).is_err() {
            return invalid_data("BSA file data is too large");
        }

        let hash = hash_path(&name);
        self.entries.push(BuilderEntry { hash, name, data });
        self.sorted = false;
        Ok(())
    }

    /// Writes the archive to `path`.
    ///
    /// The destination is only created once the archive is known to be writable, so a
    /// rejected build leaves an existing file untouched.
    pub fn save_path(&mut self, path: impl AsRef<Path>) -> io::Result<()> {
        let layout = self.prepare()?;
        let file = std::fs::File::create(path)?;
        let mut output = BufWriter::new(file);
        self.write_layout(&mut output, layout)?;
        output.flush()
    }

    /// Writes the archive into a new buffer.
    pub fn save_bytes(&mut self) -> io::Result<Vec<u8>> {
        let layout = self.prepare()?;
        let mut bytes = Vec::with_capacity(layout.total as usize);
        self.write_layout(&mut bytes, layout)?;
        Ok(bytes)
    }

    /// Streams the archive front to back, without seeking.
    pub fn write(&mut self, output: &mut impl Write) -> io::Result<()> {
        let layout = self.prepare()?;
        self.write_layout(output, layout)
    }

    /// Orders the entries, rejects hash collisions, and computes the layout.
    ///
    /// Sorting here rather than in `insert` keeps staging linear; repeated writes with no
    /// intervening insert do not re-sort.
    fn prepare(&mut self) -> io::Result<Layout> {
        if !self.sorted {
            self.entries.sort_unstable_by_key(|entry| entry.hash);
            self.sorted = true;
        }
        // The engine stops at the first hash match, so a second file sharing a hash would
        // be unreachable wherever it is placed. Rejecting equal hashes also leaves the
        // unstable sort above with one deterministic order.
        if self.entries.windows(2).any(|pair| pair[0].hash == pair[1].hash) {
            return invalid_data("BSA already contains a file with this hash");
        }
        self.layout()
    }

    /// Computes every section size up front so the write itself cannot overflow.
    fn layout(&self) -> io::Result<Layout> {
        if self.entries.is_empty() {
            return invalid_data("BSA must contain at least one file");
        }
        let Ok(count) = u32::try_from(self.entries.len()) else {
            return invalid_data("BSA contains too many files");
        };
        let n = u64::from(count);

        let mut names_len = 0;
        let mut payload_len = 0;
        for entry in &self.entries {
            names_len += u64::from(narrow(entry.name.len())?) + 1;
            payload_len += u64::from(narrow(entry.data.len())?);
        }

        let hash_offset = 12 * n + names_len;
        let total = HEADER_SIZE + hash_offset + 8 * n + payload_len;

        let (Ok(hash_offset), Ok(total)) = (u32::try_from(hash_offset), u32::try_from(total)) else {
            return invalid_data("BSA would exceed the engine's 32-bit file offsets");
        };

        Ok(Layout {
            count,
            hash_offset,
            total,
        })
    }

    fn write_layout(&self, output: &mut impl Write, layout: Layout) -> io::Result<()> {
        output.write_all(&VERSION.to_le_bytes())?;
        output.write_all(&layout.hash_offset.to_le_bytes())?;
        output.write_all(&layout.count.to_le_bytes())?;

        // Payloads are laid out sequentially, and `layout` has already proven every
        // running total below stays within a `u32`.
        let mut offset: u32 = 0;
        for entry in &self.entries {
            let size = narrow(entry.data.len())?;
            output.write_all(&size.to_le_bytes())?;
            output.write_all(&offset.to_le_bytes())?;
            offset += size;
        }

        let mut offset: u32 = 0;
        for entry in &self.entries {
            output.write_all(&offset.to_le_bytes())?;
            offset += narrow(entry.name.len())? + 1;
        }

        for entry in &self.entries {
            output.write_all(&entry.name)?;
            output.write_all(&[0])?;
        }

        for entry in &self.entries {
            output.write_all(&entry.hash.low.to_le_bytes())?;
            output.write_all(&entry.hash.high.to_le_bytes())?;
        }

        for entry in &self.entries {
            output.write_all(&entry.data)?;
        }

        Ok(())
    }
}

/// Narrows a length that [`Builder::insert`] has already checked.
fn narrow(len: usize) -> io::Result<u32> {
    u32::try_from(len).map_err(|_| invalid_data_error("BSA section is too large"))
}
