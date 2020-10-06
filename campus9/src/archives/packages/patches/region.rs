#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Region {
    ALL = 0x00,
    ENGLISH = 0x01,
    FRENCH = 0x02,
    ITALIAN = 0x03,
    GERMAN = 0x04,
    SPAIN = 0x05,
    JAPANESE = 0x06,
    PORTUGUESE = 0x07,
    RUSSIAN = 0x08,
    POLISH = 0x09,
    MEXICO = 0x0C,
    KOREAN = 0x0D,
}

impl Default for Region {
    fn default() -> Self {
        Region::ENGLISH
    }
}
