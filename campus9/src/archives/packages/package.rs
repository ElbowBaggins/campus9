use crate::archives::packages::patches::{block::MAX_BLOCK_LENGTH, Id, Patch, Entry, Block, decrypt::decrypt};
use std::collections::HashMap;
use std::io::{Write, SeekFrom, Seek, Read};
use crate::archives::Oodle;
use std::alloc::Layout;

#[derive(Debug)]
pub struct Package {
    pub id: Id,
    pub patches: HashMap<Id, Patch>,
    pub description: String,
}

impl Package {
    pub fn new(id: Id, description: String) -> Package {
        Package {
            id,
            patches: HashMap::new(),
            description,
        }
    }

    pub fn get_master_patch(&self) -> &Patch {
        self.patches.get(self.patches.keys().max().unwrap()).unwrap()
    }

    pub fn get_entries(&self) -> &Vec<Entry> {
        self.get_master_patch().get_entries()
    }

    pub fn get_blocks(&self) -> &Vec<Block> {
        self.get_master_patch().get_blocks()
    }

    pub fn get_block_bytes(&self, block: Block) -> Vec<u8> {
        let patch = self.patches.get(&block.patch_id).unwrap();
        let size = block.size as usize;

        let mut file = patch.file_info.get_file();
        file.seek(SeekFrom::Start(block.offset as u64)).expect("Seek failed!");

        let layout = Layout::array::<u8>(
            size
        ).expect("Could not create layout!");

        let raw_bytes = unsafe {
            let pointer = std::alloc::alloc(layout) as *mut u8;
            let target = std::slice::from_raw_parts_mut(pointer, size);
            file.read_exact(target).expect("Slice read failed!");
            target.to_vec()
        };

        let decrypted_bytes: Vec<u8>;
        if block.is_encrypted() {
            decrypted_bytes = decrypt(
                self.id,
                patch.file_info.header.version,
                block.is_using_alternate_key(),
                raw_bytes
            );
        } else {
            decrypted_bytes = raw_bytes;
        }

        let mut decompressed_bytes: Vec<u8>;
        if block.is_compressed() {
            decompressed_bytes = Oodle::lz_decompress(decrypted_bytes);
        } else {
            decompressed_bytes = decrypted_bytes;
        }
        decompressed_bytes.truncate(block.size as usize);
        decompressed_bytes
    }

    pub fn extract_entry(&self, entry: Entry) -> Vec<u8> {
        self.get_entry_blocks(entry).1
    }

    pub fn get_entry_blocks(&self, entry: Entry) -> (Vec<Block>, Vec<u8>) {

        let size = entry.get_size() as usize;
        let index = entry.get_start_block_index() as usize;
        let offset = entry.get_precise_start_block_offset() as usize;
        let count = (offset + size + MAX_BLOCK_LENGTH - 1) / MAX_BLOCK_LENGTH;
        let entry_blocks = &self.get_blocks()[index..(index + count)];
        let mut entry_bytes: Vec<u8> = Vec::<u8>::with_capacity(size + offset);

        for block in entry_blocks {
            entry_bytes.write_all(self.get_block_bytes(*block).as_slice()).unwrap();
        }

        (entry_blocks.to_vec(), entry_bytes[offset..].to_vec())
    }
}