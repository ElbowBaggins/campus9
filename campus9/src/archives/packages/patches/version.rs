#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[repr(u16)]
pub enum Version {
    /* Derived from header */ DESTINY = 0x18,
    /* Derived from header */ DESTINY2 = 0x26,
}

impl Default for Version {
    fn default() -> Self {
        Version::DESTINY2
    }
}
