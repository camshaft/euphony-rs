use crate::{node::Node, sink::Sink, value::Parameter};

pub trait Processor: Sized
where
    Self: Into<Parameter>,
    for<'a> &'a Self: Into<Parameter>,
{
    fn sink(&self) -> Sink;

    #[inline]
    fn fin(self) {
        drop(self)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct Definition {
    pub id: u64,
    pub inputs: u64,
    pub buffers: u64,
}

impl Definition {
    #[inline]
    pub fn spawn(&self) -> Node {
        Node::new(self, None)
    }
}

macro_rules! define_processor_binary_op {
    ($name:ident, $op:ident, $lower:ident) => {
        impl core::ops::$op<$name> for f64 {
            type Output = crate::processors::binary::$op;

            #[inline]
            fn $lower(self, rhs: $name) -> Self::Output {
                use crate::processors::input::*;
                crate::processors::binary::$lower()
                    .with_lhs(self)
                    .with_rhs(rhs)
            }
        }

        impl core::ops::$op<&$name> for f64 {
            type Output = crate::processors::binary::$op;

            #[inline]
            fn $lower(self, rhs: &$name) -> Self::Output {
                use crate::processors::input::*;
                crate::processors::binary::$lower()
                    .with_lhs(self)
                    .with_rhs(rhs)
            }
        }

        impl<Rhs: Into<crate::value::Parameter>> core::ops::$op<Rhs> for $name {
            type Output = crate::processors::binary::$op;

            #[inline]
            fn $lower(self, rhs: Rhs) -> Self::Output {
                use crate::processors::input::*;
                crate::processors::binary::$lower()
                    .with_lhs(self)
                    .with_rhs(rhs)
            }
        }

        impl<Rhs: Into<crate::value::Parameter>> core::ops::$op<Rhs> for &$name {
            type Output = crate::processors::binary::$op;

            #[inline]
            fn $lower(self, rhs: Rhs) -> Self::Output {
                use crate::processors::input::*;
                crate::processors::binary::$lower()
                    .with_lhs(self)
                    .with_rhs(rhs)
            }
        }
    };
}

macro_rules! define_processor_ops {
    ($name:ident) => {
        define_processor_binary_op!($name, Add, add);
        define_processor_binary_op!($name, Div, div);
        define_processor_binary_op!($name, Mul, mul);
        define_processor_binary_op!($name, Rem, rem);
        define_processor_binary_op!($name, Sub, sub);

        impl core::ops::Neg for $name {
            type Output = crate::processors::unary::Neg;

            #[inline]
            fn neg(self) -> Self::Output {
                use crate::processors::input::*;
                crate::processors::unary::neg().with_input(self)
            }
        }

        impl core::ops::Neg for &$name {
            type Output = crate::processors::unary::Neg;

            #[inline]
            fn neg(self) -> Self::Output {
                use crate::processors::input::*;
                crate::processors::unary::neg().with_input(self)
            }
        }
    };
}

macro_rules! define_processor {
    (
        $(#[doc = $doc:literal])?
        #[id = $id:literal]
        #[lower = $lower:ident]
        struct $name:ident {
            $(
                #[buffer]
                #[trait = $trait_buffer_name:ident]
                #[with = $with_buffer:ident]
                #[set = $set_buffer:ident]
                $buffer:ident: Buffer<$buffer_id:literal>,
            )*
            $(
                #[trait = $trait_name:ident]
                #[with = $with:ident]
                #[set = $set:ident]
                $input:ident: $input_ty:ident<$input_id:literal>,
            )*
        }
    ) => {
        $(#[doc = $doc])?
        #[derive(Clone, Debug)]
        #[must_use = "nodes do nothing unless routed to a Sink"]
        pub struct $name(crate::node::Node);

        #[inline]
        $(#[doc = $doc])?
        pub fn $lower() -> $name {
            $name::default()
        }

        mod $lower {
            use super::*;

            #[allow(unused_imports)]
            use crate::value::{Parameter, Trigger};

            impl Default for $name {
                #[inline]
                fn default() -> Self {
                    use crate::processor::Definition;
                    static DEF: Definition = Definition {
                        id: $id,
                        inputs: $({
                            let _ = $input_id;
                            1
                        } +)* 0,
                        buffers: $({
                            let _ = $buffer_id;
                            1
                        } +)* 0,
                    };
                    Self(DEF.spawn())
                }
            }

            impl $name {
                $(
                    pub fn $input(&self) -> crate::parameter::$input_ty {
                        crate::parameter::$input_ty {
                            node: self.0.clone(),
                            index: $input_id,
                        }
                    }
                )*

                $(
                    pub fn $buffer(&self) -> crate::parameter::Buffer {
                        crate::parameter::Buffer {
                            node: self.0.clone(),
                            index: $buffer_id,
                        }
                    }
                )*
            }

            impl crate::processor::Processor for $name {
                #[inline]
                fn sink(&self) -> crate::sink::Sink {
                    self.0.sink()
                }
            }

            $(
                impl<V: euphony_buffer::AsChannel> crate::processors::input::$trait_buffer_name<V> for $name {
                    #[inline]
                    fn $with_buffer(self, value: V) -> Self {
                        self.0.set_buffer($buffer_id, value);
                        self
                    }

                    #[inline]
                    fn $set_buffer(&self, value: V) -> &Self {
                        self.0.set_buffer($buffer_id, value);
                        self
                    }
                }
            )*

            $(
                impl<V: Into<$input_ty>> crate::processors::input::$trait_name<V> for $name {
                    #[inline]
                    fn $with(self, value: V) -> Self {
                        self.0.set($input_id, value.into());
                        self
                    }

                    #[inline]
                    fn $set(&self, value: V) -> &Self {
                        self.0.set($input_id, value.into());
                        self
                    }
                }
            )*

            impl From<$name> for Parameter {
                #[inline]
                fn from(node: $name) -> Self {
                    node.0.into()
                }
            }

            impl From<&$name> for Parameter {
                #[inline]
                fn from(node: &$name) -> Self {
                    (&node.0).into()
                }
            }

            define_processor_ops!($name);
        }
    };
}
