use crate::{
    buffer::{BufDef, BufDefGroup, BufView, Buffer},
    osc::control::Value,
    synthdef::builder::Value as BuilderValue,
    track::Track,
};

#[derive(Clone, Copy, Debug)]
pub enum Param {
    BufView(BufView),
    Buffer(Buffer),
    Builder(BuilderValue),
}

impl Param {
    pub fn control_value<T: Track>(self, track: &T) -> Option<Value> {
        Some(match self {
            Self::BufView(v) => track.read(v).into(),
            Self::Buffer(v) => v.into(),
            Self::Builder(v) => v.as_osc()?,
        })
    }

    pub fn value(self) -> BuilderValue {
        match self {
            Self::BufView(_) => unreachable!(),
            Self::Buffer(_) => unreachable!(),
            Self::Builder(v) => v,
        }
    }

    pub fn debug_field(&self, name: &str, f: &mut core::fmt::DebugStruct) {
        match self {
            Self::BufView(v) => {
                f.field(name, &v);
            }
            Self::Buffer(v) => {
                f.field(name, v);
            }
            Self::Builder(v) => {
                if let Some(v) = v.as_osc() {
                    f.field(name, &v);
                }
            }
        }
    }
}

impl From<i32> for Param {
    fn from(value: i32) -> Self {
        Self::Builder(value.into())
    }
}

impl From<f32> for Param {
    fn from(value: f32) -> Self {
        Self::Builder(value.into())
    }
}

impl From<&'static BufDef> for Param {
    fn from(value: &'static BufDef) -> Self {
        Self::BufView(value.into())
    }
}

impl From<BufView> for Param {
    fn from(value: BufView) -> Self {
        Self::BufView(value)
    }
}

impl From<Buffer> for Param {
    fn from(value: Buffer) -> Self {
        Self::Buffer(value)
    }
}

impl From<&'static BufDefGroup> for Param {
    fn from(value: &'static BufDefGroup) -> Self {
        value[0].into()
    }
}

impl From<BuilderValue> for Param {
    fn from(value: BuilderValue) -> Self {
        Self::Builder(value)
    }
}
