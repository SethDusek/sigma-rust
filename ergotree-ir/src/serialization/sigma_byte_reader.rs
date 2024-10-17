//! Sigma byte stream writer
use super::constant_store::ConstantStore;
use super::val_def_type_store::ValDefTypeStore;
use sigma_ser::vlq_encode::ReadSigmaVlqExt;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;

/// Implementation of SigmaByteRead
pub struct SigmaByteReader<R> {
    inner: R,
    constant_store: ConstantStore,
    substitute_placeholders: bool,
    val_def_type_store: ValDefTypeStore,
    was_deserialize: bool,
}

impl<R: Read> SigmaByteReader<R> {
    /// Create new reader from PeekableReader
    pub fn new(pr: R, constant_store: ConstantStore) -> SigmaByteReader<R> {
        SigmaByteReader {
            inner: pr,
            constant_store,
            substitute_placeholders: false,
            val_def_type_store: ValDefTypeStore::new(),
            was_deserialize: false,
        }
    }

    /// Make a new reader with underlying PeekableReader and constant_store to resolve constant
    /// placeholders
    pub fn new_with_substitute_placeholders(
        pr: R,
        constant_store: ConstantStore,
    ) -> SigmaByteReader<R> {
        SigmaByteReader {
            inner: pr,
            constant_store,
            substitute_placeholders: true,
            val_def_type_store: ValDefTypeStore::new(),
            was_deserialize: false,
        }
    }
}

/// Create SigmaByteReader from a byte array (with empty constant store)
pub fn from_bytes<T: AsRef<[u8]>>(bytes: T) -> SigmaByteReader<Cursor<T>> {
    SigmaByteReader {
        inner: Cursor::new(bytes),
        constant_store: ConstantStore::empty(),
        substitute_placeholders: false,
        val_def_type_store: ValDefTypeStore::new(),
        was_deserialize: false,
    }
}

/// Sigma byte reader trait with a constant store to resolve segregated constants
pub trait SigmaByteRead: ReadSigmaVlqExt {
    /// Constant store with constants to resolve constant placeholder types
    fn constant_store(&mut self) -> &mut ConstantStore;

    /// Option to substitute ConstantPlaceholder with Constant from the store
    fn substitute_placeholders(&self) -> bool;

    /// Set new constant store
    fn set_constant_store(&mut self, constant_store: ConstantStore);

    /// ValDef types store (resolves tpe on ValUse parsing)
    fn val_def_type_store(&mut self) -> &mut ValDefTypeStore;

    /// Returns if value that was deserialized has deserialize nodes, such as DeserializeContext and DeserializeRegister
    fn was_deserialize(&self) -> bool;

    /// Set that deserialization node was read
    fn set_deserialize(&mut self, has_deserialize: bool);
}

impl<R: Read> Read for SigmaByteReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.inner.read(buf)
    }
}

impl<R: Seek> Seek for SigmaByteReader<R> {
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64> {
        self.inner.seek(pos)
    }

    fn rewind(&mut self) -> std::io::Result<()> {
        self.inner.rewind()
    }

    fn stream_position(&mut self) -> std::io::Result<u64> {
        self.inner.stream_position()
    }
}

impl<R: ReadSigmaVlqExt> SigmaByteRead for SigmaByteReader<R> {
    fn constant_store(&mut self) -> &mut ConstantStore {
        &mut self.constant_store
    }

    fn substitute_placeholders(&self) -> bool {
        self.substitute_placeholders
    }

    fn set_constant_store(&mut self, constant_store: ConstantStore) {
        self.constant_store = constant_store;
    }

    fn val_def_type_store(&mut self) -> &mut ValDefTypeStore {
        &mut self.val_def_type_store
    }

    fn was_deserialize(&self) -> bool {
        self.was_deserialize
    }

    fn set_deserialize(&mut self, has_deserialize: bool) {
        self.was_deserialize = has_deserialize
    }
}
