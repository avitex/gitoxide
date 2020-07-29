use crate::{hash, pack};
use git_object::owned;
use quick_error::quick_error;
use std::io;

quick_error! {
    #[derive(Debug)]
    pub enum Error {
        Io(err: io::Error) {
            display("An IO error occurred when reading the pack or creating a temporary file")
            from()
            source(err)
        }
        Unsupported(kind: pack::index::Kind) {
            display("Indices of type {} cannot be written, only {} are supported", *kind as usize, pack::index::Kind::default() as usize)
        }
    }
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Outcome {
    pub index_kind: pack::index::Kind,
    pub index_hash: owned::Id,
    pub num_objects: u32,
}

#[derive(PartialEq, Eq, Debug, Hash, Ord, PartialOrd, Clone)]
#[cfg_attr(feature = "serde1", derive(serde::Serialize, serde::Deserialize))]
pub struct Entry {
    pub header: pack::data::Header,
    /// amount of bytes used to encode the `header`. `pack_offset + header_size` is the beginning of the compressed data in the pack.
    pub header_size: u16,
    pub pack_offset: u64,
    /// header + compressed bytes
    pub bytes: Vec<u8>,
    pub decompressed: Vec<u8>,
}

/// Various ways of writing an index file from pack entries
impl pack::index::File {
    pub fn write_to_stream(
        entries: impl Iterator<Item = Result<Entry, pack::data::iter::Error>>,
        _out: impl io::Write,
        kind: pack::index::Kind,
    ) -> Result<Outcome, Error> {
        let _out = hash::Write::new(_out, kind.hash());
        if kind != pack::index::Kind::default() {
            return Err(Error::Unsupported(kind));
        }
        for _entry in entries {}
        let _index_hash = _out.hash.digest();
        unimplemented!("todo stream");
    }
}
