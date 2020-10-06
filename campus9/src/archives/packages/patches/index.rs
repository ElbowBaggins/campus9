use super::{Count, Hash, Offset};
use crate::archives::packages::patches::index::IndexStrategy::DIRECT;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum IndexStrategy {
    DIRECT,
    INDIRECT,
}

impl Default for IndexStrategy {
    fn default() -> Self {
        DIRECT
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct DirectIndex {
    /* field_0x00_B4 */ pub entry_count: Count,
    /* field_0x00_B8 */ pub entries_offset: Offset,
    /* field_0x00_BC */ pub entries_hash: Hash,
    /* field_0x00_D0 */ pub block_count: Count,
    /* field_0x00_D4 */ pub blocks_offset: Offset,
    /* field_0x00_D8 */ pub blocks_hash: Hash,
    /* field_0x00_DC */ pub extra_count: Count,
    /* field_0x00_F0 */ pub extras_offset: Offset,
    /* field_0x00_F4 */ pub extra_size: u32,
    /* field_0x00_F8 */ pub extras_hash: Hash,
    /* field_0x01_0C */ pub field_0x01_0C: u32,
    /* field_0x01_10 */ pub indirect_index_offset: Offset,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct IndirectIndex {
    /* field_0x00 */ pub field_0x00: u32,
    /* field_0x04 */ pub field_0x04: u32,
    /* field_0x08 */ pub field_0x08: u32,
    /* field_0x0C */ pub field_0x0C: u32,
    /* field_0x10 */ pub entry_count: Count,
    /* field_0x14 */ pub field_0x14: u32,
    /* field_0x18 */ pub entries_meta_offset: Offset,
    /* field_0x1C */ pub field_0x1C: u32,
    /* field_0x20 */ pub block_count: Count,
    /* field_0x24 */ pub field_0x24: u32,
    /* field_0x28 */ pub blocks_meta_offset: Offset,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct EntriesMeta {
    /* field_0x00 */ pub field_0x00: u32,
    /* field_0x04 */ pub field_0x04: u32,
    /* field_0x08 */ pub field_0x08: u32,
    /* field_0x0C */ pub field_0x0C: u32,
    /* field_0x10 */ pub field_0x10: u32,
    /* field_0x14 */ pub field_0x14: u32,
    /* field_0x18 */ pub field_0x18: u32,
    /* field_0x1C */ pub field_0x1C: u32,
    /* field_0x20 */ pub field_0x20: u32,
    /* field_0x24 */ pub field_0x24: u32,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[repr(C)]
pub struct BlocksMeta {
    /* field_0x00 */ pub field_0x00: u32,
    /* field_0x04 */ pub field_0x04: u32,
    /* field_0x08 */ pub field_0x08: u32,
    /* field_0x0C */ pub field_0x0C: u32,
    /* field_0x10 */ pub field_0x10: u32,
    /* field_0x14 */ pub field_0x14: u32,
    /* field_0x18 */ pub field_0x18: u32,
    /* field_0x1C */ pub field_0x1C: u32,
    /* field_0x20 */ pub field_0x20: u32,
    /* field_0x24 */ pub field_0x24: u32,
    /* field_0x28 */ pub field_0x28: u32,
    /* field_0x3C */ pub field_0x3C: u32,
    /* field_0x30 */ pub field_0x30: u32,
    /* field_0x34 */ pub field_0x34: u32,
}
