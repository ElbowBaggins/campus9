#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Platform {
    /* Uncertain */ XBOX = 0x00,
    /* Uncertain */ PLAYSTATION = 0x01,
    WINDOWS = 0x02,
}

impl Default for Platform {
    fn default() -> Self {
        Platform::WINDOWS
    }
}
