use super::*;

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
        SetTiming::TAG => {
            let msg = SetTiming::decode(tag, input)?;
            handler.set_timing(msg)?;
        }
        CreateGroup::TAG => {
            let msg = CreateGroup::decode(tag, input)?;
            handler.create_group(msg)?;
        }
        SpawnNode::TAG_NO_GROUP | SpawnNode::TAG_WITH_GROUP => {
            let msg = SpawnNode::decode(tag, input)?;
            handler.spawn_node(msg)?;
        }
        ForkNode::TAG => {
            let msg = ForkNode::decode(tag, input)?;
            handler.fork_node(msg)?;
        }
        EmitMidi::TAG_NO_GROUP | EmitMidi::TAG_WITH_GROUP => {
            let msg = EmitMidi::decode(tag, input)?;
            handler.emit_midi(msg)?;
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
        InitBuffer::TAG => {
            let msg = InitBuffer::decode(tag, input)?;
            handler.init_buffer(msg)?;
        }
        LoadBuffer::TAG => {
            let msg = LoadBuffer::decode(tag, input)?;
            handler.load_buffer(msg)?;
        }
        SetBuffer::TAG => {
            let msg = SetBuffer::decode(tag, input)?;
            handler.set_buffer(msg)?;
        }
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid tag: 0x{tag:x}"),
            ))
        }
    }

    Ok(ControlFlow::Continue(()))
}

pub trait Codec: Sized {
    fn encode<W: io::Write>(&self, output: &mut W) -> io::Result<()>;
    fn decode<R: io::Read>(tag: u8, input: &mut R) -> io::Result<Self>;
}

pub trait WriteExt {
    fn write_u8(&mut self, value: u8) -> io::Result<()>;
    #[allow(dead_code)]
    fn write_u16(&mut self, value: u16) -> io::Result<()>;
    fn write_u32(&mut self, value: u32) -> io::Result<()>;
    fn write_u64(&mut self, value: u64) -> io::Result<()>;
}

impl<W: io::Write> WriteExt for W {
    #[inline]
    fn write_u8(&mut self, value: u8) -> io::Result<()> {
        self.write_all(&[value])?;
        Ok(())
    }

    #[inline]
    fn write_u16(&mut self, value: u16) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_u32(&mut self, value: u32) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;
        Ok(())
    }

    #[inline]
    fn write_u64(&mut self, value: u64) -> io::Result<()> {
        self.write_all(&value.to_le_bytes())?;
        Ok(())
    }
}

pub trait ReadExt {
    fn read_u8(&mut self) -> io::Result<u8>;
    #[allow(dead_code)]
    fn read_u16(&mut self) -> io::Result<u16>;
    fn read_u32(&mut self) -> io::Result<u32>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_string(&mut self, len: usize) -> io::Result<String>;
}

impl<R: io::Read> ReadExt for R {
    #[inline]
    fn read_u8(&mut self) -> io::Result<u8> {
        let mut value = [0u8; 1];
        self.read_exact(&mut value)?;
        Ok(value[0])
    }

    #[inline]
    fn read_u16(&mut self) -> io::Result<u16> {
        let mut value = [0u8; 2];
        self.read_exact(&mut value)?;
        let value = u16::from_le_bytes(value);
        Ok(value)
    }

    #[inline]
    fn read_u32(&mut self) -> io::Result<u32> {
        let mut value = [0u8; 4];
        self.read_exact(&mut value)?;
        let value = u32::from_le_bytes(value);
        Ok(value)
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut value = [0u8; 8];
        self.read_exact(&mut value)?;
        let value = u64::from_le_bytes(value);
        Ok(value)
    }

    #[inline]
    fn read_string(&mut self, len: usize) -> io::Result<String> {
        Ok(if len > 0 {
            let mut name = vec![0; len];
            self.read_exact(&mut name)?;
            match String::from_utf8_lossy(&name) {
                std::borrow::Cow::Owned(v) => v,
                std::borrow::Cow::Borrowed(_) => unsafe {
                    // the lossy will check that this is valid
                    String::from_utf8_unchecked(name)
                },
            }
        } else {
            String::new()
        })
    }
}
