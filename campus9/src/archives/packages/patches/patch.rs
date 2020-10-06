use super::{files::FileInfo};
use crate::archives::packages::patches::{Entry, Block};
use std::io::{SeekFrom, Seek, Read};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[allow(dead_code)]
pub struct Patch {
    pub file_info: FileInfo,
    entries: Vec<Entry>,
    blocks: Vec<Block>,
}


impl Patch {
    pub fn new(file_info: FileInfo) -> Patch {
        Patch {
            file_info,
            entries: Vec::<Entry>::new(),
            blocks: Vec::<Block>::new(),
        }
    }

    pub fn load(&mut self) {
        self.entries = self.file_info.get_entries();
        self.blocks = self.file_info.get_blocks();
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        &self.entries
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    pub fn get_block_slice(&self, block: &Block) -> Vec<u8> {
        let mut file = self.file_info.get_file();
        let mut slice = Vec::<u8>::with_capacity(block.size as usize);
        file.seek(SeekFrom::Start(block.offset as u64)).unwrap();
        match file.read_exact(slice.as_mut_slice()) {
            Ok(()) => slice,
            Err(e) => panic!(e)
        }
    }
}