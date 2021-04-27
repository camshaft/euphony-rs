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
        pub fn fn_synth(out: f32<0.0>, freq: f32) {
            let signal = SinOsc::new().freq([freq, freq, freq]).ar();
            let signal = Pan2::new(signal).ar();
            let signal = Mix::new(signal).ar();
            Out::new(out, signal).ar()
        }
    );

    synthdef!(
        pub fn splay_synth(out: f32<0.0>, freq: f32) {
            let signal = SinOsc::new().freq([freq * 1.0, freq * 1.5, freq * 2]).ar();
            let signal = Splay::new(signal).ar();
            Out::new(out, signal).ar()
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

    #[test]
    fn splay_test() {
        dbg!(splay_synth().freq(440));
    }
}
