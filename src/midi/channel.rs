use core::ops::Deref;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Channel(u8);

impl Channel {
    pub const C1: Self = Channel(0);
    pub const C10: Self = Channel(9);
    pub const C11: Self = Channel(10);
    pub const C12: Self = Channel(11);
    pub const C13: Self = Channel(12);
    pub const C14: Self = Channel(13);
    pub const C15: Self = Channel(14);
    pub const C16: Self = Channel(15);
    pub const C2: Self = Channel(1);
    pub const C3: Self = Channel(2);
    pub const C4: Self = Channel(3);
    pub const C5: Self = Channel(4);
    pub const C6: Self = Channel(5);
    pub const C7: Self = Channel(6);
    pub const C8: Self = Channel(7);
    pub const C9: Self = Channel(8);

    pub fn new(value: u8) -> Option<Self> {
        if value < 16 {
            Some(Self(value))
        } else {
            None
        }
    }

    pub fn from_status_byte(byte: u8) -> Self {
        Self(byte & 0b1111)
    }
}

impl Deref for Channel {
    type Target = u8;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
