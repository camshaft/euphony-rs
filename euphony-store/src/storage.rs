use crate::codec::Output;

pub mod fs;

pub trait Storage {
    type Output: Output;

    fn create(&mut self) -> Self::Output;
}
