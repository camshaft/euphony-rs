use core::{fmt, ops::ControlFlow};
use std::io;

pub mod api;
mod codec;
mod handler;

pub use codec::{decode, decode_one, Codec};
pub use handler::Handler;

use codec::{ReadExt, WriteExt};

#[cfg(test)]
use bolero::generator::*;

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
        write!(f, "ADVANCE ticks = {:?}", self.ticks)
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
pub struct SetTiming {
    pub nanos_per_tick: u64,
    pub ticks_per_beat: u64,
}

impl SetTiming {
    const TAG: u8 = b'T';
}

impl fmt::Display for SetTiming {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  SET_TIMING nanos_per_tick = {}, ticks_per_beat = {}",
            self.nanos_per_tick, self.ticks_per_beat,
        )
    }
}

impl Codec for SetTiming {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.nanos_per_tick)?;
        output.write_u64(self.ticks_per_beat)?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let nanos_per_tick = input.read_u64()?;
        let ticks_per_beat = input.read_u64()?;
        Ok(Self {
            nanos_per_tick,
            ticks_per_beat,
        })
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
        write!(f, "  GROUP id = {}, name = {:?}", self.id, self.name)
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
        let name = input.read_string(len as usize)?;
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
        write!(
            f,
            "  SPAWN id = {}, processor = {}",
            self.id, self.processor
        )?;
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
pub struct ForkNode {
    pub source: u64,
    pub target: u64,
}

impl ForkNode {
    const TAG: u8 = b'k';
}

impl fmt::Display for ForkNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  FORK source = {}, target = {}",
            self.source, self.target
        )?;
        Ok(())
    }
}

impl Codec for ForkNode {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.source)?;
        output.write_u64(self.target)?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(_tag: u8, input: &mut R) -> io::Result<Self> {
        let source = input.read_u64()?;
        let target = input.read_u64()?;
        Ok(Self { source, target })
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct EmitMidi {
    pub data: [u8; 3],
    pub group: Option<u64>,
}

impl EmitMidi {
    const TAG_NO_GROUP: u8 = b'm';
    const TAG_WITH_GROUP: u8 = b'M';
}

impl fmt::Display for EmitMidi {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  MIDI data = {:?}", self.data)?;
        Ok(())
    }
}

impl Codec for EmitMidi {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        if self.group.is_some() {
            output.write_u8(Self::TAG_WITH_GROUP)?;
        } else {
            output.write_u8(Self::TAG_NO_GROUP)?;
        }
        output.write_all(&self.data)?;
        if let Some(group) = self.group {
            output.write_u64(group)?;
        }
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        let mut data = [0; 3];
        input.read_exact(&mut data)?;
        let group = if tag == Self::TAG_WITH_GROUP {
            Some(input.read_u64()?)
        } else {
            None
        };
        Ok(Self { data, group })
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
            "  SET node = {}, param = {}, value = {}",
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
            "  PIPE node = {}, param = {}, source = {}",
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
        write!(f, "  FIN node = {}", self.node)
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct InitBuffer {
    #[cfg_attr(test, generator(gen::<String>().with().len(0usize..64)))]
    pub source: String,
    #[cfg_attr(test, generator(gen::<String>().with().len(0usize..64)))]
    pub meta: String,
}

impl InitBuffer {
    const TAG: u8 = b'I';
}

impl fmt::Display for InitBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "  INIT_BUF path = {:?}", self.source)
    }
}

impl Codec for InitBuffer {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u32(self.source.len() as _)?;
        output.write_all(self.source.as_bytes())?;
        output.write_u32(self.meta.len() as _)?;
        output.write_all(self.meta.as_bytes())?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let len = input.read_u32()?;
        let source = input.read_string(len as usize)?;
        let len = input.read_u32()?;
        let meta = input.read_string(len as usize)?;
        Ok(Self { source, meta })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct LoadBuffer {
    pub id: u64,
    #[cfg_attr(test, generator(gen::<String>().with().len(0usize..64)))]
    pub path: String,
    #[cfg_attr(test, generator(gen::<String>().with().len(0usize..64)))]
    pub ext: String,
}

impl LoadBuffer {
    const TAG: u8 = b'B';
}

impl fmt::Display for LoadBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  LOAD_BUF id = {}, path = {:?}, ext = {:?}",
            self.id, self.path, self.ext
        )
    }
}

impl Codec for LoadBuffer {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.id)?;
        output.write_u32(self.path.len() as _)?;
        output.write_all(self.path.as_bytes())?;
        if !self.ext.is_empty() {
            output.write_u8(self.ext.len() as _)?;
            output.write_all(self.ext.as_bytes())?;
        } else {
            output.write_u8(0)?;
        }
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self> {
        debug_assert_eq!(Self::TAG, tag);
        let id = input.read_u64()?;
        let len = input.read_u32()?;
        let path = input.read_string(len as usize)?;
        let ext_len = input.read_u8()?;
        let ext = if ext_len > 0 {
            input.read_string(ext_len as _)?
        } else {
            String::new()
        };
        Ok(Self { id, path, ext })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(test, derive(TypeGenerator))]
pub struct SetBuffer {
    pub target_node: u64,
    pub target_parameter: u64,
    pub buffer: u64,
    pub buffer_channel: u64,
}

impl SetBuffer {
    const TAG: u8 = b'u';
}

impl fmt::Display for SetBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "  SET_BUFFER node = {}, param = {}, buffer = {}, channel = {}",
            self.target_node, self.target_parameter, self.buffer, self.buffer_channel
        )
    }
}

impl Codec for SetBuffer {
    #[inline]
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write_u8(Self::TAG)?;
        output.write_u64(self.target_node)?;
        output.write_u64(self.target_parameter)?;
        output.write_u64(self.buffer)?;
        output.write_u64(self.buffer_channel)?;
        Ok(())
    }

    #[inline]
    fn decode<R: io::Read>(_tag: u8, input: &mut R) -> io::Result<Self> {
        let target_node = input.read_u64()?;
        let target_parameter = input.read_u64()?;
        let buffer = input.read_u64()?;
        let buffer_channel = input.read_u64()?;
        Ok(Self {
            target_node,
            target_parameter,
            buffer,
            buffer_channel,
        })
    }
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
    fn set_timing() {
        check!().with_type::<SetTiming>().for_each(round_trip);
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

    #[test]
    fn load_buffer() {
        check!().with_type::<LoadBuffer>().for_each(round_trip);
    }

    #[test]
    fn set_buffer() {
        check!().with_type::<SetBuffer>().for_each(round_trip);
    }
}
