use crate::{node::Node, parameter::Parameter, processor::Definition, processors::input};

static SINK: Definition = Definition {
    id: 0,
    inputs: 4,
    buffers: 0,
};

#[derive(Clone, Debug)]
pub struct Sink(Node);

impl Default for Sink {
    fn default() -> Self {
        let group = crate::group::current();
        Self(Node::new(&SINK, Some(group.as_u64())))
    }
}

impl Sink {
    #[inline]
    pub fn with<T: Into<Parameter>>(self, out: T) -> Self {
        self.0.set(0, out);
        self
    }

    #[inline]
    pub fn set<T: Into<Parameter>>(&self, out: T) -> &Self {
        self.0.set(0, out);
        self
    }

    #[inline]
    pub fn fin(self) {
        drop(self)
    }
}

impl<V: Into<Parameter>> input::AzimuthInput<V> for Sink {
    #[inline]
    fn with_azimuth(self, value: V) -> Self {
        self.0.set(1, value);
        self
    }

    #[inline]
    fn set_azimuth(&self, value: V) -> &Self {
        self.0.set(1, value);
        self
    }
}

impl<V: Into<Parameter>> input::InclineInput<V> for Sink {
    #[inline]
    fn with_incline(self, value: V) -> Self {
        self.0.set(2, value);
        self
    }

    #[inline]
    fn set_incline(&self, value: V) -> &Self {
        self.0.set(2, value);
        self
    }
}

impl<V: Into<Parameter>> input::RadiusInput<V> for Sink {
    #[inline]
    fn with_radius(self, value: V) -> Self {
        self.0.set(3, value);
        self
    }

    #[inline]
    fn set_radius(&self, value: V) -> &Self {
        self.0.set(3, value);
        self
    }
}
