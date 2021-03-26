use codec::{
    buffer,
    encode::{Encoder, EncoderBuffer, TypeEncoder},
};
use core::{fmt, marker::PhantomData, time::Duration};

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

/*
#[test]
fn arg_tags_encoding_test() {
    let mut buf = vec![0u8; 32];
    let (len, _) = buf
        .encode_with(ArgTags(&[Tag::Int32, Tag::String][..]), Padding)
        .unwrap();
    assert_eq!(&buf[..len], b",is\0");
}
*/

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
        let (len, _) = buf.encode(Address(Str(addr))).unwrap();
        assert_eq!(&buf[..len], expected);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(u8)]
pub enum Tag {
    Int32 = b'i',
    Float32 = b'f',
    String = b's',
    Blob = b'b',
    Int64 = b'h',
    Timetag = b't',
    Double = b'd',
    Symbol = b'S',
    Char = b'c',
    RGBA = b'r',
    MIDI = b'm',
    True = b'T',
    False = b'F',
    Nil = b'N',
    Infinitum = b'I',
    ArrayOpen = b'[',
    ArrayClose = b']',
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
            let v: $ty = Default::default();
            v.encode_tag(&mut buf[..]).unwrap();
            assert_eq!(&buf[..$expected.len()], $expected);
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

#[derive(Clone, Copy, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Timetag([u8; 8]);

impl fmt::Debug for Timetag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.as_duration().fmt(f)
    }
}

impl Timetag {
    const INV_MAX: f64 = 1.0 / Self::MAX;
    const MAX: f64 = (u32::MAX as f64) + 1.0;
    const NANOS_PER_SEC: u32 = Duration::from_secs(1).as_nanos() as u32;
    const SECS_PER_NANO: f64 = 1.0 / (Self::NANOS_PER_SEC as f64);

    pub fn new(timestamp: Duration) -> Self {
        let secs = timestamp.as_secs() as u32;
        let nanos = timestamp.subsec_nanos() as f64;
        let frac = (nanos * Self::SECS_PER_NANO * Self::MAX).round() as u32;

        let mut out = [0u8; 8];

        let secs = secs.to_be_bytes();
        out[0] = secs[0];
        out[1] = secs[1];
        out[2] = secs[2];
        out[3] = secs[3];

        let frac = frac.to_be_bytes();
        out[4] = frac[0];
        out[5] = frac[1];
        out[6] = frac[2];
        out[7] = frac[3];

        Self(out)
    }

    pub fn as_duration(self) -> Duration {
        let mut secs = [0u8; 4];
        secs[0] = self.0[0];
        secs[1] = self.0[1];
        secs[2] = self.0[2];
        secs[3] = self.0[3];
        let secs = u32::from_be_bytes(secs) as u64;

        let mut frac = [0u8; 4];
        frac[0] = self.0[4];
        frac[1] = self.0[5];
        frac[2] = self.0[6];
        frac[3] = self.0[7];
        let frac = u32::from_be_bytes(frac);
        let nanos = (frac as f64) * Self::INV_MAX * Self::NANOS_PER_SEC as f64;
        let nanos = nanos as u64;

        Duration::from_nanos(secs * Self::NANOS_PER_SEC as u64 + nanos)
    }
}

impl From<Duration> for Timetag {
    fn from(timestamp: Duration) -> Self {
        Self::new(timestamp)
    }
}

impl From<Timetag> for Duration {
    fn from(timetag: Timetag) -> Self {
        timetag.as_duration()
    }
}

impl<B: EncoderBuffer> TypeEncoder<B> for Timetag {
    fn encode_type(self, buffer: B) -> buffer::Result<(), B> {
        (self.0[..]).encode_type(buffer)
    }
}

impl AsRef<[u8; 8]> for Timetag {
    fn as_ref(&self) -> &[u8; 8] {
        &self.0
    }
}

impl AsRef<[u8]> for Timetag {
    fn as_ref(&self) -> &[u8] {
        &self.0
    }
}

#[test]
fn timetag_inverse_pair_test() {
    let times = [0, 10, 1000, 123456789];
    for expected in times.iter().copied().map(Duration::from_millis) {
        let actual = Timetag::new(expected).as_duration();
        assert_eq!(expected, actual);
    }
}

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
