use super::{AuthenticationTag, Count, Hash, Id, Offset};

pub type Flags = u16;
pub const MAX_BLOCK_LENGTH: usize = 0x40000;
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[repr(C)]
pub struct Block {
    pub offset: Offset,
    pub size: Count,
    pub patch_id: Id,
    pub flags: Flags,
    pub hash: Hash,
    pub authentication_tag: AuthenticationTag,
}

impl Block {
    pub fn is_compressed(&self) -> bool {
        (self.flags & 0x0001) == 0x0001
    }

    pub fn is_encrypted(&self) -> bool {
        (self.flags & 0x0002) == 0x0002
    }

    pub fn is_using_alternate_key(&self) -> bool {
        (self.flags & 0x0004) == 0x0004
    }
}

