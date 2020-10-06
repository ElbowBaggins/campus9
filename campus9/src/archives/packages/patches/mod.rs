mod platform;
mod version;

pub use block::Block;
pub use entry::Entry;
pub use header::Header;
pub use patch::Patch;
pub use platform::Platform;
pub use region::Region;
pub use version::Version;

pub(crate) mod block;
mod entry;
pub mod files;
mod header;
pub mod index;
mod patch;
mod region;
pub mod decrypt;
pub mod decompress;

pub const HASH_LENGTH: usize = 0x00_14;
pub const TAG_LENGTH: usize = 0x00_10;
pub const MAGIC_NUMBER: u32 = 0x28_11_41_FD;
pub const BUILD_STRING_LENGTH: usize = 0x20;
pub const RSA_SIGNATURE_LENGTH: usize = 0x100;

pub type BuildString = [u32; BUILD_STRING_LENGTH];
pub type RsaSignature = [u8; RSA_SIGNATURE_LENGTH];
pub type Id = u16;
pub type Epoch = u32;
pub type AuthenticationTag = [u8; TAG_LENGTH];
pub type Hash = [u8; HASH_LENGTH];
pub type EntryTable = [Entry; 1];
pub type BlockTable = [Block; 1];
pub type MagicNumber = u32;
pub type Offset = u32;
pub type Count = u32;
