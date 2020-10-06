use super::{Header, Id};
use std::fs::{File};
use std::path::PathBuf;
use std::mem::size_of;
use std::io;
use std::io::{Read, Seek, SeekFrom};
use crate::archives::packages::patches::index::{IndirectIndex, EntriesMeta, BlocksMeta};
use crate::archives::packages::patches::index::IndexStrategy::{DIRECT, INDIRECT};
use crate::archives::packages::patches::{Count, Entry, Block};
use std::alloc::Layout;

fn vec_to_string(vector: Vec<String>) -> io::Result<String> {
    let mut result = String::new();
    for i in 0..vector.len() {
        result.push_str(&(vector[i]));
        if i < vector.len() - 1 {
            result.push_str("_");
        }
    }
    Ok(result)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct FileInfo {
    pub filename: String,
    pub platform: String,
    pub description: String,
    pub package_id: Id,
    pub patch_id: Id,
    pub language: String,
    pub header: Header,
    pub indirect_index: IndirectIndex,
    path: PathBuf,
}

impl FileInfo {
    pub fn from_path(path: PathBuf) -> FileInfo {
        let filename = String::from(
            path.file_stem().expect(
                "Could not get file stem"
            ).to_str().expect(
                "Could not convert file stem to string!"
            )
        );

        let mut chunks: Vec<String> = filename.split('_').map(
            |value| value.to_string()
        ).collect();
        let mut file: File = File::open(&path).expect("File open failed during construction!");

        let platform = chunks.remove(0);
        let patch_id = Id::from_str_radix(
            &chunks.pop().expect("Could not pop Id from filename vector!"),
            16).expect("Id in filename is not valid hex!");
        let package_id: Id;
        let language: String;

        let before_patch_id = chunks.pop().expect("Could not pop value prior to patch_id!");
        if before_patch_id.len() == 2 {
            language = before_patch_id;
        } else {
            language = String::from("NA");
        }
        let header_result = Header::unpack_from_file(file);
        let header = header_result.0;
        package_id = header.package_id;
        file = header_result.1;
        let indirect_index: IndirectIndex;
        if header.index.indirect_index_offset > 0 {
            file.seek(
                SeekFrom::Start(header.index.indirect_index_offset as u64)
            ).expect("Unable to seek to indirect index");
            let indirect_index_result = IndirectIndex::unpack_from_file(file);
            indirect_index = indirect_index_result.0;
        } else {
            indirect_index = IndirectIndex::default();
        }
        FileInfo {
            filename,
            platform,
            description: vec_to_string(chunks).expect("Could not prepare description!"),
            package_id,
            patch_id,
            language,
            header,
            indirect_index,
            path,
        }
    }

    pub fn get_entry_count(&self) -> Count {
        if self.header.format == DIRECT {
            self.header.index.entry_count
        } else if self.header.format == INDIRECT {
            self.indirect_index.entry_count
        } else {
            0
        }
    }

    pub fn get_entries_offset(&self) -> u64 {
        if self.header.format == DIRECT {
            self.header.index.entries_offset as u64
        } else if self.header.format == INDIRECT {
            self.header.index.indirect_index_offset as u64 +
                self.indirect_index.entries_meta_offset as u64 +
                size_of::<EntriesMeta>() as u64
        } else {
            0 as u64
        }
    }

    pub fn get_block_count(&self) -> Count {
        if self.header.format == DIRECT {
            self.header.index.block_count
        } else if self.header.format == INDIRECT {
            self.indirect_index.block_count
        } else {
            0
        }
    }

    pub fn get_blocks_offset(&self) -> u64 {
        if self.header.format == DIRECT {
            self.header.index.blocks_offset as u64
        } else if self.header.format == INDIRECT {
            self.header.index.indirect_index_offset as u64 +
                self.indirect_index.blocks_meta_offset as u64 +
                size_of::<BlocksMeta>() as u64
        } else {
            0 as u64
        }
    }

    pub fn get_entries(&self) -> Vec<Entry> {
        let mut file = self.get_file();
        file.seek(SeekFrom::Start(self.get_entries_offset() as u64)).expect("Seek failed!");
        let entry_count = self.get_entry_count();
        Entry::unpack_slice_from_file(file, entry_count as usize).0
    }

    pub fn get_blocks(&self) -> Vec<Block> {
        let mut file = self.get_file();
        file.seek(SeekFrom::Start(self.get_blocks_offset() as u64)).expect("Seek failed!");
        Block::unpack_slice_from_file(file, self.get_block_count() as usize).0
    }

    pub fn get_file(&self) -> File {
        File::open(self.path.as_path()).expect("File open failed!")
    }
}

pub trait PackedStruct<T> {
    fn unpack_from_file(file: File) -> (T, File);
}

pub trait PackedStructs<T> where T: Clone {
    fn unpack_slice_from_file(file: File, count: usize) -> (Vec<T>, File);
}

impl<T> PackedStruct<T> for T {
    fn unpack_from_file(mut file: File) -> (T, File) {
        let layout = Layout::new::<T>();
        unsafe {
            let pointer = std::alloc::alloc(layout) as *mut T;
            let target = std::slice::from_raw_parts_mut(
                pointer as *mut u8,
                size_of::<T>()
            );
            file.read_exact(target).expect("Single struct read failed!");
            (pointer.read(), file)
        }
    }
}

impl<T> PackedStructs<T> for T where T: Clone {
    fn unpack_slice_from_file(mut file: File, count: usize) -> (Vec<T>, File) where T: Clone {
        let layout = Layout::array::<u8>(
            count * size_of::<T>()
        ).expect("Could not create layout!");

        unsafe {
            let pointer = std::alloc::alloc(layout) as *mut u8;
            let target = std::slice::from_raw_parts_mut(
                pointer,
                count * size_of::<T>()
            );
            file.read_exact(target).expect("Slice read failed!");
            let result = &*(target as *mut [u8] as *const [T]);
            (result[0..count].to_vec(), file)
        }
    }
}



