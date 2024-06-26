use crate::{node::Node, output};
use euphony_units::{pitch::frequency::Frequency, ratio::Ratio};

#[derive(Clone, Debug)]
pub struct Parameter(pub(crate) ParameterValue);

impl Parameter {
    pub(crate) fn set(&self, target_node: u64, target_parameter: u64) {
        match &self.0 {
            ParameterValue::Unset => output::set_parameter(target_node, target_parameter, 0.0),
            ParameterValue::Constant(value) => {
                output::set_parameter(target_node, target_parameter, *value)
            }
            ParameterValue::Node(ref source) => {
                output::pipe_parameter(target_node, target_parameter, source.id())
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trigger(pub(crate) ParameterValue);

#[derive(Clone, Debug)]
pub(crate) enum ParameterValue {
    Unset,
    Constant(f64),
    Node(Node),
}

impl From<Node> for Parameter {
    #[inline]
    fn from(node: Node) -> Self {
        Self(ParameterValue::Node(node))
    }
}

impl From<&Node> for Parameter {
    #[inline]
    fn from(node: &Node) -> Self {
        Self(ParameterValue::Node(node.clone()))
    }
}

impl From<Trigger> for Parameter {
    #[inline]
    fn from(value: Trigger) -> Self {
        Self(value.0)
    }
}

impl From<&Trigger> for Parameter {
    #[inline]
    fn from(value: &Trigger) -> Self {
        Self(value.0.clone())
    }
}

macro_rules! impl_convert {
    ($name:ident) => {
        impl From<f64> for $name {
            #[inline]
            fn from(value: f64) -> Self {
                Self(ParameterValue::Constant(value))
            }
        }

        impl From<u64> for $name {
            #[inline]
            fn from(value: u64) -> Self {
                Self(ParameterValue::Constant(value as _))
            }
        }

        impl From<Frequency> for $name {
            #[inline]
            fn from(value: Frequency) -> Self {
                Self(ParameterValue::Constant(value.into()))
            }
        }

        impl From<crate::prelude::Beat> for $name {
            #[inline]
            fn from(value: crate::prelude::Beat) -> Self {
                (crate::time::tempo() * value).into()
            }
        }

        impl From<core::time::Duration> for $name {
            #[inline]
            fn from(value: core::time::Duration) -> Self {
                value.as_secs_f64().into()
            }
        }

        impl From<crate::prelude::Interval> for $name {
            #[inline]
            fn from(value: crate::prelude::Interval) -> Self {
                use crate::ext::*;
                value.freq().into()
            }
        }

        impl From<Ratio<u64>> for $name {
            #[inline]
            fn from(value: Ratio<u64>) -> Self {
                (value.0 as f64 / value.1 as f64).into()
            }
        }
    };
}

impl_convert!(Parameter);
impl_convert!(Trigger);
