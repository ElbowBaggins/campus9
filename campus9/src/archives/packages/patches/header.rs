use super::index::{DirectIndex, IndexStrategy};
use super::{BuildString, Epoch, Id, MagicNumber, Offset, Platform, Region, Version};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Default)]
#[allow(non_snake_case)]
#[allow(dead_code)]
#[repr(C)]
pub struct Header {
    /* field_0x00_00 */ pub version: Version,
    /* field_0x00_02 */ pub platform: Platform,
    /* field_0x00_04 */ pub package_id: Id,

    /* field_0x00_06 */ pub field_0x00_06: u16,
    /* field_0x00_08 */ pub field_0x00_08: u32,
    /* field_0x00_0C */ pub field_0x00_0C: u32,

    /* field_0x00_10 */ pub build_epoch: Epoch,

    /* field_0x00_14 */ pub field_0x14: u32,
    /* field_0x00_18 */ pub field_0x18: u16,

    /* field_0x00_1A */ pub format: IndexStrategy,

    /* field_0x00_1C */ pub field_0x00_1C: u32,

    /* field_0x00_20 */ pub patch_id: Id,
    /* field_0x00_22 */ pub region: Region,
    /* field_0x24-A0 */ pub build_string: BuildString,

    /* field_0x00_A4 */ pub field_0x00_A4: u32,
    /* field_0x00_A8 */ pub field_0x00_A8: u32,
    /* field_0x00_AC */ pub field_0x00_AC: u32,
    /* field_0x00_B0 */ pub rsa_signature_offset: Offset,

    /* 0xB4 - 0x1_10 */ pub index: DirectIndex,

    /* field_0x01_14 */ pub field_0x01_14: u32,
    /* field_0x01_18 */ pub field_0x01_18: u32,
    /* field_0x01_1C */ pub field_0x01_1C: u32,
    /* field_0x01_20 */ pub field_0x01_20: u32,
    /* field_0x01_24 */ pub field_0x01_24: u32,
    /* field_0x01_28 */ pub field_0x01_28: u32,
    /* field_0x01_2C */ pub field_0x01_2C: u32,
    /* field_0x01_30 */ pub field_0x01_30: u32,
    /* field_0x01_34 */ pub field_0x01_34: u32,
    /* field_0x01_38 */ pub field_0x01_38: u32,
    /* field_0x01_3C */ pub field_0x01_3C: u32,
    /* field_0x01_40 */ pub field_0x01_40: u32,
    /* field_0x01_44 */ pub field_0x01_44: u32,
    /* field_0x01_48 */ pub field_0x01_48: u32,
    /* field_0x01_4C */ pub field_0x01_4C: u32,
    /* field_0x01_50 */ pub field_0x01_50: u32,
    /* field_0x01_54 */ pub field_0x01_54: u32,
    /* field_0x01_58 */ pub field_0x01_58: u32,
    /* field_0x01_5C */ pub field_0x01_5C: u32,
    /* field_0x01_60 */ pub dead_beef_offset: Offset,
    /* field_0x01_64 */ pub end_offset: Offset,
    /* field_0x01_68 */
    pub magic_number: MagicNumber, // Seems to indicate end of header
}

