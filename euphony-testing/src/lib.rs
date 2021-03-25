use core::fmt;
pub use hex_literal::hex;
pub use pretty_assertions::*;

#[derive(PartialEq)]
pub struct Hex<T>(pub T);

impl<T: AsRef<[u8]>> fmt::Display for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        pretty_hex::PrettyHex::hex_dump(&self.0).fmt(f)
    }
}

impl<T: AsRef<[u8]>> fmt::Debug for Hex<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        pretty_hex::PrettyHex::hex_dump(&self.0).fmt(f)
    }
}

#[macro_export]
macro_rules! assert_hex_eq {
    ($a:expr, $b:expr $(, $($tt:tt)*)?) => {
        $crate::assert_eq!(
            $crate::Hex(&$a[..]),
            $crate::Hex(&$b[..])
            $(, $($tt)*)?
        );
    };
}
