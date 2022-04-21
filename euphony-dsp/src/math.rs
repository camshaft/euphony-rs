use crate::sample::Default as Sample;
use euphony_node::{Input, Node};

macro_rules! binary {
    ($(#[doc = $doc:literal])* $id:literal, $name:ident, | $a:ident, $b:ident | $value:expr) => {
        #[derive(Debug, Clone, Copy, Default, Node)]
        #[node(id = $id)]
        #[input(a)]
        #[input(b)]
        $(#[doc = $doc])*
        pub struct $name;

        impl $name {
            fn render(&mut self, a: Input, b: Input, output: &mut [Sample]) {
                match (a, b) {
                    (Input::Constant($a), Input::Constant($b)) => {
                        let v = $value;
                        for sample in output.iter_mut() {
                            *sample = v;
                        }
                    }
                    (Input::Constant($a), Input::Buffer(b))
                    | (Input::Buffer(b), Input::Constant($a)) => {
                        for (sample, $b) in output.iter_mut().zip(b.iter()) {
                            *sample = $value;
                        }
                    }
                    (Input::Buffer(a), Input::Buffer(b)) => {
                        for (sample, ($a, $b)) in output.iter_mut().zip(a.iter().zip(b.iter())) {
                            *sample = $value;
                        }
                    }
                }
            }
        }
    };
}

binary!(
    /// Adds two signals together
    200,
    Add,
    |a, b| a + b
);
binary!(
    /// Multiplies two signals together
    201,
    Mul,
    |a, b| a * b
);
