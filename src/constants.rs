use std::fmt;

pub type Comment = ShortString<[u8; 40]>;
pub type Magic = ShortString<[u8; 8]>;

pub const MAGIC: Magic = ShortString { size: 8, string: *b"MONOTONE" };
pub const NUM_ORDERS: usize = 256;
pub const FILE_HEADER_SIZE: usize = 0x15F;

#[derive(Copy)]
#[repr(C)]
pub struct Order(pub [u8; NUM_ORDERS]);

impl Clone for Order {
    fn clone(&self) -> Self {
        *self
    }
}

impl fmt::Debug for Order {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "Order({:?})", &self.0[..])
    }
}

#[derive(Copy)]
#[repr(C)]
pub struct ShortString<T = [u8; 255]> {
    pub size: u8,
    pub string: T,
}

impl PartialEq for Magic {
    fn eq(&self, rhs: &Self) -> bool {
        self.size == rhs.size && &self.string[..] == &rhs.string[..]
    }
}

impl PartialEq for Comment {
    fn eq(&self, rhs: &Self) -> bool {
        self.size == rhs.size && &self.string[..] == &rhs.string[..]
    }
}

impl PartialEq for ShortString {
    fn eq(&self, rhs: &Self) -> bool {
        self.size == rhs.size && &self.string[..] == &rhs.string[..]
    }
}

impl fmt::Debug for Magic {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "ShortString {{ size: {:?}, string: {:?} }}", self.size, &self.string[..])
    }
}

impl fmt::Debug for Comment {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "ShortString {{ size: {:?}, string: {:?} }}", self.size, &self.string[..])
    }
}

impl fmt::Debug for ShortString {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(formatter, "ShortString {{ size: {:?}, string: {:?} }}", self.size, &self.string[..])
    }
}

impl<T: Copy> Clone for ShortString<T> {
    fn clone(&self) -> Self {
        *self
    }
}
