use euphony::{
    prelude::{Interval, Mode},
    units::ratio::Ratio,
};
use midly::live::{LiveEvent, SystemCommon};
use std::{ops::Deref, sync::Arc, time::Duration};

pub use midly::{
    live::{MtcQuarterFrameMessage, SystemRealtime as Realtime},
    num::{u14, u4, u7},
    MidiMessage,
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Bytes(Arc<[u7]>);

impl Deref for Bytes {
    type Target = [u7];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Event {
    Midi { channel: u4, message: MidiMessage },
    Common(Common),
    Realtime(Realtime),
    Internal(Internal),
}

impl Default for Event {
    fn default() -> Self {
        Self::undefined()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Internal {
    Bool(bool),
    Number(Ratio<i64>),
    Mode(Mode),
    Duration(Duration),
    Undefined,
}

impl Event {
    pub fn parse(data: &[u8]) -> Option<Self> {
        let v = LiveEvent::parse(data).ok()?;
        Some(v.into())
    }

    pub fn write(&self, buffer: &mut Vec<u8>) -> Option<()> {
        let event = match self {
            Event::Midi { channel, message } => LiveEvent::Midi {
                channel: *channel,
                message: *message,
            },
            Event::Common(common) => LiveEvent::Common(common.into()),
            Event::Realtime(realtime) => LiveEvent::Realtime(*realtime),
            _ => return None,
        };
        event.write(buffer).ok()?;
        Some(())
    }

    pub fn is_truthy(&self) -> bool {
        !matches!(
            self,
            Self::Internal(Internal::Bool(false)) | Self::Internal(Internal::Undefined)
        )
    }

    pub fn undefined() -> Self {
        Self::Internal(Internal::Undefined)
    }

    pub fn defined(&self) -> Option<&Self> {
        if let Self::Internal(Internal::Undefined) = self {
            None
        } else {
            Some(self)
        }
    }

    pub fn as_u4(&self) -> Option<u4> {
        if let Self::Internal(Internal::Number(v)) = self {
            let v: u8 = (*v).whole().try_into().ok()?;
            v.try_into().ok()
        } else {
            None
        }
    }

    pub fn as_u7(&self) -> Option<u7> {
        if let Self::Internal(Internal::Number(v)) = self {
            let v: u8 = (*v).whole().try_into().ok()?;
            v.try_into().ok()
        } else {
            None
        }
    }

    pub fn as_u14(&self) -> Option<u14> {
        if let Self::Internal(Internal::Number(v)) = self {
            let v: u16 = (*v).whole().try_into().ok()?;
            v.try_into().ok()
        } else {
            None
        }
    }

    pub fn as_number(&self) -> Option<Ratio<i64>> {
        if let Self::Internal(Internal::Number(v)) = self {
            Some(*v)
        } else {
            None
        }
    }

    pub fn as_duration(&self) -> Option<Duration> {
        match self {
            Self::Internal(Internal::Number(v)) => {
                Some(Duration::from_secs_f64(v.0 as f64 / v.1 as f64))
            }
            Self::Internal(Internal::Duration(v)) => Some(*v),
            _ => None,
        }
    }

    pub fn as_mode(&self) -> Option<Mode> {
        if let Self::Internal(Internal::Mode(v)) = self {
            Some(*v)
        } else {
            None
        }
    }
}

impl<'a> From<LiveEvent<'a>> for Event {
    fn from(e: LiveEvent<'a>) -> Self {
        match e {
            LiveEvent::Midi { channel, message } => Self::Midi { channel, message },
            LiveEvent::Common(common) => Self::Common(common.into()),
            LiveEvent::Realtime(realtime) => Self::Realtime(realtime),
        }
    }
}

impl From<bool> for Event {
    fn from(v: bool) -> Self {
        Event::Internal(Internal::Bool(v))
    }
}

impl From<u4> for Event {
    fn from(v: u4) -> Self {
        Event::Internal(Internal::Number(Ratio(v.as_int() as _, 1)))
    }
}

impl From<u7> for Event {
    fn from(v: u7) -> Self {
        Event::Internal(Internal::Number(Ratio(v.as_int() as _, 1)))
    }
}

impl From<u14> for Event {
    fn from(v: u14) -> Self {
        Event::Internal(Internal::Number(Ratio(v.as_int() as _, 1)))
    }
}

impl From<i64> for Event {
    fn from(v: i64) -> Self {
        Event::Internal(Internal::Number(Ratio(v, 1)))
    }
}

impl From<Ratio<i64>> for Event {
    fn from(v: Ratio<i64>) -> Self {
        Event::Internal(Internal::Number(v))
    }
}

impl From<Interval> for Event {
    fn from(v: Interval) -> Self {
        Event::Internal(Internal::Number(v.as_ratio()))
    }
}

impl From<Mode> for Event {
    fn from(v: Mode) -> Self {
        Event::Internal(Internal::Mode(v))
    }
}

impl From<Duration> for Event {
    fn from(v: Duration) -> Self {
        Event::Internal(Internal::Duration(v))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum Common {
    SysEx(Bytes),
    MidiTimeCodeQuarterFrame(MtcQuarterFrameMessage, u4),
    SongPosition(u14),
    SongSelect(u7),
    TuneRequest,
    Undefined(u8, Bytes),
}

impl<'a> From<SystemCommon<'a>> for Common {
    fn from(e: SystemCommon<'a>) -> Self {
        match e {
            SystemCommon::SysEx(data) => Self::SysEx(Bytes(data.to_vec().into())),
            SystemCommon::MidiTimeCodeQuarterFrame(a, b) => Self::MidiTimeCodeQuarterFrame(a, b),
            SystemCommon::SongPosition(a) => Self::SongPosition(a),
            SystemCommon::SongSelect(a) => Self::SongSelect(a),
            SystemCommon::TuneRequest => Self::TuneRequest,
            SystemCommon::Undefined(a, b) => Self::Undefined(a, Bytes(b.to_vec().into())),
        }
    }
}

impl<'a> From<&'a Common> for SystemCommon<'a> {
    fn from(e: &'a Common) -> Self {
        match e {
            Common::SysEx(d) => Self::SysEx(d),
            Common::MidiTimeCodeQuarterFrame(a, b) => Self::MidiTimeCodeQuarterFrame(*a, *b),
            Common::SongPosition(a) => Self::SongPosition(*a),
            Common::SongSelect(a) => Self::SongSelect(*a),
            Common::TuneRequest => Self::TuneRequest,
            Common::Undefined(a, b) => Self::Undefined(*a, b),
        }
    }
}
