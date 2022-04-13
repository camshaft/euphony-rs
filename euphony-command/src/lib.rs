pub mod api;
pub mod message;
pub mod node;

#[cfg(feature = "reader")]
pub mod reader;

pub fn foo(freq: f32, phase: f32, output: &mut [f32; node::BUF_LEN]) {
    use node::{Constructor, Result};

    struct L;

    impl node::Loader for L {
        fn mul(&mut self) -> Option<usize> {
            None
        }

        fn add(&mut self) -> Option<usize> {
            None
        }

        fn input(&mut self, id: u32) -> Result<usize> {
            todo!()
        }

        fn node(&mut self, id: u32, output: u32) -> Result<usize> {
            todo!()
        }

        fn buffer(&mut self, id: u32) -> Result<usize> {
            todo!()
        }
    }

    api::Sine {
        frequency: freq.into(),
        phase: 0.0.into(),
    }
    .load(&mut L)
    .unwrap()
    .fill(&[][..], &[][..], &mut [output][..])
}

#[test]
fn f() {
    let mut output = [0.0; node::BUF_LEN];
    foo(440.0, 0.0, &mut output);
    panic!("{:?}", output);
}
