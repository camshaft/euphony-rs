use euphony_sc::{params, synthdef};

mod my_params {
    use super::*;

    params!(
        pub struct Params {
            /// FOO
            pub freq: f32<440.0>,
        }
    );
}

mod my_synth {
    use super::*;

    pub fn closure_synth() -> my_params::SynthDef {
        synthdef!(|params| sine(params, 1.0))
    }

    fn sine(params: my_params::Params, _foo: f32) {
        let _ = params.freq;
    }

    synthdef!(
        pub fn fn_synth(freq: f32) {
            let mut freq = freq * [1, 2];
            freq += 4;
        }
    );

    #[test]
    fn closure_test() {
        dbg!(closure_synth().freq(440));
    }

    #[test]
    fn fn_test() {
        dbg!(fn_synth().freq(440));
    }
}
