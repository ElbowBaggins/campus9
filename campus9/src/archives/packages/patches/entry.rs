use crate::archives::Id;
use crate::archives::packages::patches::{Region, Count, Offset};

pub type EntryLocation = u64;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[repr(C)]
pub struct Entry {
    pub major_type: u32,
    pub minor_type: u32,
    pub b: EntryLocation,
}

impl Entry {

    // Lower 14 bits
    pub fn get_start_block_index(&self) -> Id {
        (self.b & 0x3FFF) as u16
    }

    // Next 14 bits
    pub fn get_start_block_chunk(&self) -> Offset {
        ((self.b >> 0x0E) & 0x3FFF) as u16 as u32
    }

    pub fn get_precise_start_block_offset(&self) -> Offset {
        self.get_start_block_chunk() as u32 * 0x10
    }

    // Next 30 bits
    pub fn get_size(&self) -> Count {
        ((self.b >> 0x1C) & 0x3FFFFFFF) as u32
    }

    pub fn calc_hash(package_id: Id, index: u32, region: Region) -> u32 {
        (((0x04_04 + (package_id as u32)) * 0x01_00 + index) << 0x00_0D) + ((region as u16) as u32)
    }
}