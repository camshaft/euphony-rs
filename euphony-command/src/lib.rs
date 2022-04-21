use core::{fmt, ops::ControlFlow};
use std::io;

#[cfg(test)]
use bolero::generator::*;

pub trait Handler {
    fn advance_time(&mut self, msg: AdvanceTime) -> io::Result<()>;
    fn set_nanos_per_tick(&mut self, msg: SetNanosPerTick) -> io::Result<()>;
    fn create_group(&mut self, msg: CreateGroup) -> io::Result<()>;
    fn spawn_node(&mut self, msg: SpawnNode) -> io::Result<()>;
    fn set_parameter(&mut self, msg: SetParameter) -> io::Result<()>;
    fn pipe_parameter(&mut self, msg: PipeParameter) -> io::Result<()>;
    fn finish_node(&mut self, msg: FinishNode) -> io::Result<()>;
}

fn push_msg<T: fmt::Display>(output: &mut String, v: T) -> io::Result<()> {
    output.push_str(&format!("{}\n", v));
    Ok(())
}

impl Handler for String {
    fn advance_time(&mut self, msg: AdvanceTime) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn set_nanos_per_tick(&mut self, msg: SetNanosPerTick) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn create_group(&mut self, msg: CreateGroup) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn spawn_node(&mut self, msg: SpawnNode) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn set_parameter(&mut self, msg: SetParameter) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn pipe_parameter(&mut self, msg: PipeParameter) -> io::Result<()> {
        push_msg(self, msg)
    }

    fn finish_node(&mut self, msg: FinishNode) -> io::Result<()> {
        push_msg(self, msg)
    }
}

pub fn decode<R: io::Read, H: Handler>(input: &mut R, handler: &mut H) -> io::Result<()> {
    while decode_one(input, handler)?.is_continue() {}
    Ok(())
}

#[deny(unreachable_patterns)]
pub fn decode_one<R: io::Read, H: Handler>(
    input: &mut R,
    handler: &mut H,
) -> io::Result<ControlFlow<()>> {
    let tag = match input.read_u8() {
        Ok(tag) => tag,
        Err(err) if err.kind() == io::ErrorKind::UnexpectedEof => {
            return Ok(ControlFlow::Break(()));
        }
        Err(err) => return Err(err),
    };

    match tag {
        AdvanceTime::TAG => {
            let msg = AdvanceTime::decode(tag, input)?;
            handler.advance_time(msg)?;
        }
        SetNanosPerTick::TAG => {
            let msg = SetNanosPerTick::decode(tag, input)?;
            handler.set_nanos_per_tick(msg)?;
        }
        CreateGroup::TAG => {
            let msg = CreateGroup::decode(tag, input)?;
            handler.create_group(msg)?;
        }
        SpawnNode::TAG_NO_GROUP | SpawnNode::TAG_WITH_GROUP => {
            let msg = SpawnNode::decode(tag, input)?;
            handler.spawn_node(msg)?;
        }
        SetParameter::TAG_PARAM | SetParameter::TAG_NONE => {
            let msg = SetParameter::decode(tag, input)?;
            handler.set_parameter(msg)?;
        }
        PipeParameter::TAG_PARAM | PipeParameter::TAG_NONE => {
            let msg = PipeParameter::decode(tag, input)?;
            handler.pipe_parameter(msg)?;
        }
        FinishNode::TAG => {
            let msg = FinishNode::decode(tag, input)?;
            handler.finish_node(msg)?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid tag: 0x{:x}", tag),
            ))
        }
    }

    Ok(ControlFlow::Continue(()))
}

pub trait Codec: Sized {
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()>;
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self>;
}

trait WriteExt {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    fn write_u64(&mut self, value: u64) -> io::Result<()>;
}

impl<W: io::Write> WriteExt for W {
    #[inline]
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])?;
        Ok(())
    }

    #[inline]
    fn write_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_all(&value.to_be_bytes())?;
        Ok(())
    }
}

trait ReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    fn read_u64(&mut self) -> io::Result<u64>;
}

impl<R: io::Read> ReadExt for R {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut value = [0u8; 1];
        self.read_exact(&mut value)?;
        Ok(value[0])
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut value = [0u8; 8];
        self.read_exact(&mut value)?;
        let value = u64::from_be_bytes(value);
        Ok(value)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct AdvanceTime {
    pub ticks: u64,
}

impl AdvanceTime {
    const TAG: u8 = b't';
}

impl fmt::Display for AdvanceTime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ADV {:?}", self.ticks)
    }
}

impl Codec for AdvanceTime {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.ticks)?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let ticks = input.read_u64()?;
        Ok(Self { ticks })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct SetNanosPerTick {
    pub nanos: u64,
}

impl SetNanosPerTick {
    const TAG: u8 = b'T';
}

impl fmt::Display for SetNanosPerTick {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  NPT {}", self.nanos)
    }
}

impl Codec for SetNanosPerTick {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.nanos)?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let nanos = input.read_u64()?;
        Ok(Self { nanos })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct CreateGroup {
    pub id: u64,
    #[cfg_attr(test, generator(gen::<String>().with().len(0usize..64)))]
    pub name: String,
}

impl CreateGroup {
    const TAG: u8 = b'g';
}

impl fmt::Display for CreateGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  GRP {},{:?}", self.id, self.name)
    }
}

impl Codec for CreateGroup {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.id)?;
        let len = self.name.len().min(255);
        output.write_u8(len as u8)?;
        if len > 0 {
            output.write_all(&self.name.as_bytes()[..len])?;
        }
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let id = input.read_u64()?;
        let len = input.read_u8()?;
        let name = if len > 0 {
            let mut name = vec![0; len as usize];
            input.read_exact(&mut name)?;
            match String::from_utf8_lossy(&name) {
                std::borrow::Cow::Owned(v) => v,
                std::borrow::Cow::Borrowed(_) => unsafe {
                    // the lossy will check that this is valid
                    String::from_utf8_unchecked(name)
                },
            }
        } else {
            String::new()
        };
        Ok(Self { id, name })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct SpawnNode {
    pub id: u64,
    pub processor: u64,
    pub group: Option<u64>,
}

impl SpawnNode {
    const TAG_NO_GROUP: u8 = b'n';
    const TAG_WITH_GROUP: u8 = b'N';
}

impl fmt::Display for SpawnNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // TODO map generator to name
        write!(f, "  SPN {},{}", self.id, self.processor)?;
        Ok(())
    }
}

impl Codec for SpawnNode {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        if self.group.is_some() {
            output.write_u8(Self::TAG_WITH_GROUP)?;
        } else {
            output.write_u8(Self::TAG_NO_GROUP)?;
        }
        output.write_u64(self.id)?;
        output.write_u64(self.processor)?;
        if let Some(group) = self.group {
            output.write_u64(group)?;
        }
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        let id = input.read_u64()?;
        let generator = input.read_u64()?;
        let group = if tag == Self::TAG_WITH_GROUP {
            Some(input.read_u64()?)
        } else {
            None
        };
        Ok(Self {
            id,
            processor: generator,
            group,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct SetParameter {
    pub target_node: u64,
    pub target_parameter: u64,
    pub value: u64,
}

impl SetParameter {
    const TAG_PARAM: u8 = b'S';
    const TAG_NONE: u8 = b's';
}

impl fmt::Display for SetParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  SET {},{},{}",
            self.target_node,
            self.target_parameter,
            f64::from_bits(self.value)
        )
    }
}

impl Codec for SetParameter {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        match self.target_parameter {
            0 => {
                output.write_u8(Self::TAG_NONE)?;
                output.write_u64(self.target_node)?;
                output.write_u64(self.value)?;
            }
            param => {
                output.write_u8(Self::TAG_PARAM)?;
                output.write_u64(self.target_node)?;
                output.write_u64(param)?;
                output.write_u64(self.value)?;
            }
        }
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        let target_node = input.read_u64()?;
        let target_parameter = if tag == Self::TAG_PARAM {
            input.read_u64()?
        } else {
            0
        };
        let value = input.read_u64()?;
        Ok(Self {
            target_node,
            target_parameter,
            value,
        })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct PipeParameter {
    pub target_node: u64,
    pub target_parameter: u64,
    pub source_node: u64,
}

impl PipeParameter {
    const TAG_PARAM: u8 = b'P';
    const TAG_NONE: u8 = b'p';
}

impl fmt::Display for PipeParameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  PIP {},{},{}",
            self.target_node, self.target_parameter, self.source_node
        )
    }
}

impl Codec for PipeParameter {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        match self.target_parameter {
            0 => {
                output.write_u8(Self::TAG_NONE)?;
                output.write_u64(self.target_node)?;
                output.write_u64(self.source_node)?;
            }
            param => {
                output.write_u8(Self::TAG_PARAM)?;
                output.write_u64(self.target_node)?;
                output.write_u64(self.source_node)?;
                output.write_u64(param)?;
            }
        }
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        let target_node = input.read_u64()?;
        let source_node = input.read_u64()?;
        let mut v = Self {
            target_node,
            target_parameter: 0,
            source_node,
        };
        match tag {
            Self::TAG_NONE => {}
            Self::TAG_PARAM => {
                v.target_parameter = input.read_u64()?;
            }
            _ => unreachable!(),
        }
        Ok(v)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct FinishNode {
    pub node: u64,
}

impl FinishNode {
    const TAG: u8 = b'f';
}

impl fmt::Display for FinishNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  FIN {}", self.node)
    }
}

impl Codec for FinishNode {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.node)?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let node = input.read_u64()?;
        Ok(Self { node })
    }
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct LoadFile<'a> {
    pub id: u64,
    pub path: &'a str,
}

#[derive(Clone, Copy, Debug)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct LoadBuffer<'a> {
    pub id: u64,
    pub buffer: &'a [u8],
}

#[cfg(test)]
mod tests {
    use super::*;
    use bolero::check;
    use std::io::Cursor;

    fn round_trip<T: Codec + fmt::Debug + PartialEq>(v: &T) {
        let mut buf = Cursor::new(vec![]);
        v.encode(&mut buf).unwrap();
        buf.set_position(0);
        let tag = buf.read_u8().unwrap();
        let actual = T::decode(tag, &mut buf).unwrap();
        assert_eq!(&actual, v);
    }

    #[test]
    fn advance_time() {
        check!().with_type::<AdvanceTime>().for_each(round_trip);
    }

    #[test]
    fn set_nanos_per_tick() {
        check!().with_type::<SetNanosPerTick>().for_each(round_trip);
    }

    #[test]
    fn create_group() {
        check!().with_type::<CreateGroup>().for_each(round_trip);
    }

    #[test]
    fn spawn_node() {
        check!().with_type::<SpawnNode>().for_each(round_trip);
    }

    #[test]
    fn set_parameter() {
        check!().with_type::<SetParameter>().for_each(round_trip);
    }

    #[test]
    fn pipe_parameter() {
        check!().with_type::<PipeParameter>().for_each(round_trip);
    }

    #[test]
    fn finish_node() {
        check!().with_type::<FinishNode>().for_each(round_trip);
    }
}
