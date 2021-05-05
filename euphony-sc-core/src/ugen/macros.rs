macro_rules! ugen {
    (
        $(#[doc = $doc:tt])*
        #[rates = [$($rate:ident),*]]
        $(#[new($($required:ident: $required_ty:ty),*)])?
        $(#[compile = $compile:expr])?
        $(#[meta = $meta:expr])?
        #[output = $output:ty]
        $(#[output_len = $output_len:expr])?
        struct $name:ident {
            $(
                $(#[doc = $field_doc:tt])*
                $(#[default = $default:expr])?
                $field:ident: $ty:ty
            ),*
            $(,)?
        }
    ) => {
        $(#[doc = $doc])*
        #[derive(Clone, Debug)]
        pub struct $name {
            $(
                $(#[doc = $field_doc])*
                pub $field: $ty,
            )*
        }

        impl $name {
            #[allow(clippy::new_without_default)]
            pub fn new($($($required: $required_ty),*)?) -> Self {
                $(
                    $(
                        let $field = $default;
                    )?
                )*

                Self {
                    $(
                        $field: $field.into(),
                    )*
                }
            }

            $(
                $(#[doc = $field_doc])*
                pub fn $field(mut self, $field: impl Into<$ty>) -> Self {
                    self.$field = $field.into();
                    self
                }
            )*

            $(
                #[doc = "Creates a the UGen with the specified rate"]
                pub fn $rate(self) -> $output {
                    self.build(CalculationRate::$rate())
                }
            )*

            fn build(self, rate: Option<CalculationRate>) -> $output {
                #![allow(unused_mut)]

                let Self { $($field,)* } = self;

                let mut spec = UgenSpec {
                    name: stringify!($name),
                    rate,
                    ..Default::default()
                };

                $(
                    spec.compile = $compile;
                )?

                $(
                    spec.meta = $meta;
                )?

                $(
                    spec.outputs = $output_len;
                )?

                ugen(spec, |mut ugen| {
                    $(
                        ugen.input($field);
                    )*
                    ugen.finish()
                })
            }
        }
    };
}
