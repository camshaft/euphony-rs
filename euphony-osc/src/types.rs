use codec::{buffer,
            encode::{Encoder, EncoderBuffer, TypeEncoder}};
use core::fmt;

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Packet<Address, Arguments> {
    pub address: Address,
    pub arguments: Arguments,
}

impl<Adr, Args, B> TypeEncoder<B> for Packet<Adr, Args>
where
    Adr: TypeEncoder<B>,
    Args: Arguments<B> + TypeEncoder<B>,
    B: EncoderBuffer,
{
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        let (_, buffer) = self.address.encode_type(buffer)?;
        let types = ArgTags(&self.arguments);
        let (_, buffer) = Padding.encode_into(types, buffer)?;
        let (_, buffer) = self.arguments.encode_type(buffer)?;

        Ok(((), buffer))
    }
}

struct ArgTags<'a, T>(&'a T);

impl<'a, T, B> TypeEncoder<B> for ArgTags<'a, T>
where
    T: Arguments<B>,
    B: EncoderBuffer,
{
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        let (_, buffer) = buffer.encode(b',')?;
        let (_, buffer) = self.0.encode_tags(buffer)?;
        Ok(((), buffer))
    }
}

#[test]
fn arg_tags_encoding_test() {
    let mut buf = vec![0u8; 32];
    let (len, _) = buf.encode_with(ArgTags(&&[u32::tag(), String::tag()][..]), Padding)
        .unwrap();
    assert_eq!(&buf[..len], b",is\0");
}

pub trait Arguments<B: EncoderBuffer> {
    fn encode_tags(&self, buffer: B) -> buffer::Result<(), B>;
}

impl<T: Tagged, B: EncoderBuffer> Arguments<B> for &[T] {
    fn encode_tags(&self, mut buffer: B) -> buffer::Result<(), B> {
        for v in self.iter() {
            let (_, next) = v.tag_of().encode_type(buffer)?;
            buffer = next;
        }
        Ok(((), buffer))
    }
}

impl<B: EncoderBuffer> Arguments<B> for &[Tag] {
    fn encode_tags(&self, mut buffer: B) -> buffer::Result<(), B> {
        for t in self.iter() {
            let (_, next) = t.encode_type(buffer)?;
            buffer = next;
        }
        Ok(((), buffer))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Address<'a>(&'a str);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct AddressError;

impl fmt::Display for AddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "OSC Addresses must start with a '/' (forward slash)")
    }
}

impl<'a> Address<'a> {
    pub fn new(value: &'a str) -> Result<Self, AddressError> {
        //# OSC Address Patterns
        //# An OSC Address Pattern is an OSC-string beginning with the character '/' (forward slash).

        if !value.starts_with('/') {
            return Err(AddressError);
        }

        Ok(Self(value))
    }

    pub unsafe fn new_unchecked(value: &'a str) -> Self {
        Self(value)
    }
}

impl<'a, B: EncoderBuffer> TypeEncoder<B> for Address<'a> {
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        Padding.encode_into(self.0.as_bytes(), buffer)
    }
}

#[test]
fn address_encoding_test() {
    let mut buf = vec![0u8; 32];
    let tests = [
        ("/h", &b"/h\0\0"[..]),
        ("/ho", &b"/ho\0"[..]),
        ("/hor", &b"/hor\0\0\0\0"[..]),
        ("/hors", &b"/hors\0\0\0"[..]),
        ("/horse", &b"/horse\0\0"[..]),
    ];
    for (addr, expected) in tests.iter().copied() {
        let (len, _) = buf.encode(Address(addr)).unwrap();
        assert_eq!(&buf[..len], expected);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
pub enum Tag {
    Int32 = 'i' as u8,
    Float32 = 'f' as u8,
    String = 's' as u8,
    Blob = 'b' as u8,
    Int64 = 'h' as u8,
    Timetag = 't' as u8,
    Double = 'd' as u8,
    Symbol = 'S' as u8,
    Char = 'c' as u8,
    RGBA = 'r' as u8,
    MIDI = 'm' as u8,
    True = 'T' as u8,
    False = 'F' as u8,
    Nil = 'N' as u8,
    Infinitum = 'I' as u8,
    ArrayOpen = '[' as u8,
    ArrayClose = ']' as u8,
}

impl<B: EncoderBuffer> TypeEncoder<B> for Tag {
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        (self as u8).encode_type(buffer)
    }
}

#[test]
fn tag_encoding_test() {
    let mut buf = vec![0u8; 32];

    macro_rules! check {
        ($ty:ty, $expected:expr) => {
            let (len, _) = buf.encode(<$ty as Tagged>::tag()).unwrap();
            assert_eq!(&buf[..len], $expected);
        };
    }

    check!(u8, b"i");
    check!(u16, b"i");
    check!(u32, b"i");
}

pub trait Tagged {
    fn tag() -> Tag;

    fn tag_of(&self) -> Tag {
        Self::tag()
    }
}

macro_rules! tag {
    ($($ty:ty = $tag:ident),* $(,)?) => {
        $(
            impl Tagged for $ty {
                fn tag() -> Tag {
                    Tag::$tag
                }
            }
        )*
    }
}

tag!(
    u8 = Int32,
    i8 = Int32,
    u16 = Int32,
    i16 = Int32,
    u32 = Int32,
    i32 = Int32,
    u64 = Int64,
    i64 = Int64,
    f32 = Float32,
    f64 = Double,
    char = Char,
    () = Nil,
    Timetag = Timetag,
    &str = String,
    String = String,
    &String = String,
    &[u8] = Blob,
    Vec<u8> = Blob,
    &Vec<u8> = Blob,
);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Timetag(u64);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Padding;

impl<T: TypeEncoder<B>, B: EncoderBuffer> Encoder<T, B> for Padding {
    fn encode_into(self, value: T, buffer: B) -> buffer::Result<(), B> {
        let (len, buffer) = buffer.encode(value)?;
        let padding = 4 - (len % 4);
        let (_, buffer) = buffer.encode_repeated(0u8, padding)?;
        Ok(((), buffer))
    }
}
