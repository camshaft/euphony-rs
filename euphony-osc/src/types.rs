use codec::{
    buffer,
    encode::{Encoder, EncoderBuffer, TypeEncoder},
};
use core::{fmt, marker::PhantomData};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Packet<Address, Arguments> {
    pub address: Address,
    pub arguments: Arguments,
}

impl<Adr, Args, B> TypeEncoder<B> for Packet<Adr, Args>
where
    Adr: TypeEncoder<B>,
    Args: Tagged<B> + TypeEncoder<B>,
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
    T: Tagged<B>,
    B: EncoderBuffer,
{
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        let (_, buffer) = buffer.encode(b',')?;
        let (_, buffer) = self.0.encode_tag(buffer)?;
        Ok(((), buffer))
    }
}

#[test]
fn arg_tags_encoding_test() {
    let mut buf = vec![0u8; 32];
    let (len, _) = buf
        .encode_with(ArgTags(&&[u32::tag(), String::tag()][..]), Padding)
        .unwrap();
    assert_eq!(&buf[..len], b",is\0");
}

macro_rules! tuple_args {
    ([$($acc:ident($a_value:tt),)*]) => {
        impl<Buf: EncoderBuffer> Tagged<Buf> for () {
            fn encode_tag(&self, buffer: Buf) -> buffer::Result<(), Buf> {
                Ok(((), buffer))
            }
        }

        // done
    };
    ($head:ident($h_value:tt), $($tail:ident($t_value:tt), )* [$($acc:ident($a_value:tt),)*]) => {
        impl<$head: Tagged<Buf> $(, $acc: Tagged<Buf>)*, Buf: EncoderBuffer> Tagged<Buf> for ($($acc, )* $head ,) {
            fn encode_tag(&self, buffer: Buf) -> buffer::Result<(), Buf> {
                $(
                    let (_, buffer) = self.$a_value.encode_tag(buffer)?;
                )*
                self.$h_value.encode_tag(buffer)
            }
        }

        tuple_args!($($tail($t_value),)* [$($acc($a_value),)* $head($h_value),]);
    }
}

tuple_args!(
    A(0),
    B(1),
    C(2),
    D(3),
    E(4),
    F(5),
    G(6),
    H(7),
    I(8),
    J(9),
    K(10),
    L(11),
    M(12),
    N(13),
    O(14),
    P(15),
    Q(16),
    R(17),
    S(18),
    T(19),
    U(20),
    V(21),
    W(22),
    X(23),
    Y(24),
    Z(25),
    AA(26),
    AB(27),
    AC(28),
    AD(29),
    AE(30),
    AF(31),
    AG(32),
    []
);

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Address<'a>(Str<'a>);

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

        Ok(Self(Str(value)))
    }

    pub unsafe fn new_unchecked(value: &'a str) -> Self {
        Self(Str(value))
    }
}

impl<'a, B: EncoderBuffer> TypeEncoder<B> for Address<'a> {
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        self.0.encode_type(buffer)
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

pub trait Tagged<Buf: EncoderBuffer> {
    fn encode_tag(&self, buffer: Buf) -> buffer::Result<(), Buf>;
}

macro_rules! tag {
    ($($ty:ty = $tag:ident),* $(,)?) => {
        $(
            impl<Buf: EncoderBuffer> Tagged<Buf> for $ty {
                fn encode_tag(&self, buffer: Buf) -> buffer::Result<(), Buf> {
                    (Tag::$tag).encode_type(buffer)
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
    Timetag = Timetag,
    &str = String,
    String = String,
    &String = String,
    &[u8] = Blob,
    Vec<u8> = Blob,
    &Vec<u8> = Blob,
);

impl<B: EncoderBuffer, T: Tagged<B>> Tagged<B> for Option<T> {
    fn encode_tag(&self, buffer: B) -> buffer::Result<(), B> {
        if let Some(value) = self.as_ref() {
            value.encode_tag(buffer)
        } else {
            Ok(((), buffer))
        }
    }
}

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

macro_rules! helper {
    ($name:ident, $inner:ty $(, $prefix:expr)?) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
        pub struct $name<'a>(pub &'a $inner);

        impl<'a, B: EncoderBuffer> Tagged<B> for $name<'a> {
            fn encode_tag(&self, buffer: B) -> buffer::Result<(), B> {
                self.0.encode_tag(buffer)
            }
        }

        impl<'a, B: EncoderBuffer> TypeEncoder<B> for $name<'a> {
            fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
                let r: &[u8] = self.0.as_ref();
                $(
                    let (_, buffer) = ($prefix)(r, buffer)?;
                )?
                Padding.encode_into(r, buffer)
            }
        }
    };
}

helper!(Str, str);
helper!(Blob, [u8], |r: &[u8], buffer| (r.len() as i32)
    .encode_type(buffer));

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Flatten<'a, T>(pub &'a [T]);

impl<'a, B: EncoderBuffer, T: Tagged<B>> Tagged<B> for Flatten<'a, T> {
    fn encode_tag(&self, mut buffer: B) -> buffer::Result<(), B> {
        for value in self.0.iter() {
            let (_, next) = value.encode_tag(buffer)?;
            buffer = next;
        }
        Ok(((), buffer))
    }
}

impl<'a, B: EncoderBuffer, T: TypeEncoder<B> + Copy> TypeEncoder<B> for Flatten<'a, T> {
    fn encode_type(self, mut buffer: B) -> buffer::Result<(), B> {
        for value in self.0.iter().copied() {
            let (_, next) = value.encode_type(buffer)?;
            buffer = next;
        }
        Ok(((), buffer))
    }
}

impl<'a, T> LenOf for Flatten<'a, T> {
    fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct LenPrefix<L, T>(PhantomData<L>, T);

impl<L, T> LenPrefix<L, T> {
    pub const fn new(value: T) -> Self {
        Self(PhantomData, value)
    }
}

pub trait LenOf {
    fn len(&self) -> usize;
}

impl<T> LenOf for &[T] {
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

impl<B, L, T> Tagged<B> for LenPrefix<L, T>
where
    B: EncoderBuffer,
    L: Tagged<B> + core::convert::TryFrom<usize>,
    T: Tagged<B> + LenOf,
{
    fn encode_tag(&self, buffer: B) -> buffer::Result<(), B> {
        let len = self.1.len();
        let len = L::try_from(len).unwrap_or_else(|_| panic!("list too big"));
        let (_, buffer) = len.encode_tag(buffer)?;
        self.1.encode_tag(buffer)
    }
}

impl<B, L, T> TypeEncoder<B> for LenPrefix<L, T>
where
    B: EncoderBuffer,
    L: TypeEncoder<B> + core::convert::TryFrom<usize>,
    T: TypeEncoder<B> + LenOf,
{
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        let len = self.1.len();
        let len = L::try_from(len).unwrap_or_else(|_| panic!("list too big"));
        let (_, buffer) = len.encode_type(buffer)?;
        self.1.encode_type(buffer)
    }
}
