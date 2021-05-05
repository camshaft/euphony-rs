use super::*;
use crate::synthdef::{
    builder::{GetWrap as _, InputVec},
    Input,
};

#[derive(Clone, Copy, Debug)]
#[repr(i32)]
pub enum Curve {
    Step = 0,
    Linear = 1,
    Exponential = 2,
    Sine = 3,
    Welch = 4,
    Squared = 6,
    Cubed = 7,
    Hold = 8,
}

impl From<Curve> for Value {
    fn from(value: Curve) -> Self {
        (value as i32).into()
    }
}

impl From<Curve> for ValueVec {
    fn from(value: Curve) -> Self {
        (value as i32).into()
    }
}

impl<const N: usize> From<[Curve; N]> for ValueVec {
    fn from(value: [Curve; N]) -> Self {
        value.iter().copied().map(Value::from).collect()
    }
}

ugen!(
    #[rates = [xr]]
    #[compile = compile_env]
    #[output = Value]
    struct Env {
        #[default = [0, 1, 0]]
        levels: ValueVec,

        #[default = [1, 1]]
        times: ValueVec,

        #[default = Curve::Linear]
        curve: ValueVec,

        #[default = -99]
        sustain: ValueVec,

        #[default = -99]
        looped: ValueVec,
    }
);

fn compile_env(_spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let mut args = inputs.iter();
    let levels = args.next().expect("levels");
    assert!(!levels.is_empty(), "at least one level is needed");

    let times = args.next().expect("times");
    let curve = args.next().expect("curve");
    let sustain = args.next().expect("sustain");
    let looped = args.next().expect("looped");

    // find out how many instances we have
    let instances = levels
        .iter()
        .chain(times.iter())
        .chain(curve.iter())
        .map(|input| input.len())
        .max()
        .unwrap_or(1);

    let sustain_len = sustain.iter().map(|input| input.len()).sum();
    let looped_len = looped.iter().map(|input| input.len()).sum();

    let mut sustain = sustain.iter().flatten().cycle();
    let mut looped = looped.iter().flatten().cycle();

    let instances = instances.max(sustain_len).max(looped_len);

    let size = compiler.push_const((levels.len() - 1) as f32);

    for instance in 0..instances {
        let mut outputs = InputVec::new();

        outputs.push(*levels[0].get_wrap(instance));
        outputs.push(size);
        outputs.push(*sustain.next().unwrap());
        outputs.push(*looped.next().unwrap());

        for (idx, level) in levels.iter().skip(1).enumerate() {
            outputs.push(*level.get_wrap(instance));
            outputs.push(*times.get_wrap(idx).get_wrap(instance));
            // TODO support float curves as well
            outputs.push(*curve.get_wrap(idx).get_wrap(instance));
            outputs.push(compiler.push_const(0.));
        }

        compiler.push_outputs(outputs);
    }
}

ugen!(
    #[rates = [ar, kr]]
    #[new(envelope: impl Into<ValueVec>)]
    #[compile = compile_envgen]
    #[output = Value]
    struct EnvGen {
        /// The envelope that is polled when the EnvGen is triggered.
        ///
        /// The Env inputs can be other UGens.
        envelope: ValueVec,

        /// Triggers the envelope and holds it open while > 0.
        ///
        /// If the Env is fixed-length (e.g. Env.linen, Env.perc),
        /// the gate argument is used as a simple trigger. If it is
        /// an sustaining envelope (e.g. Env.adsr, Env.asr), the envelope
        /// is held open until the gate becomes 0, at which point is released.
        ///
        /// If gate < 0, force release with time -1.0 - gate.
        #[default = 1.0]
        gate: ValueVec,

        #[default = 1.0]
        level_scale: ValueVec,

        #[default = 0.0]
        level_bias: ValueVec,

        #[default = 1.0]
        time_scale: ValueVec,

        #[default = 0]
        done_action: ValueVec,
    }
);

fn compile_envgen(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let mut args = inputs.iter_raw();
    let env = args.next().expect("envelope");
    let env = env
        .iter()
        .map(|input| {
            if let Input::UGen { index, .. } = input {
                inputs.outputs(*index as usize)
            } else {
                panic!("invalid envelope");
            }
        })
        .collect::<Vec<_>>();

    for (instance, expansion) in inputs.expand().iter().enumerate() {
        let rate = spec.rate.unwrap_or(CalculationRate::Control); // TODO ask children?

        let ins = expansion
            .inputs()
            .skip(1)
            .chain(
                env.get_wrap(instance)
                    .iter()
                    .flatten()
                    .map(|input| input.input),
            )
            .collect();

        let ugen = UGen {
            name: spec.name.into(),
            ins,
            rate,
            outs: vec![rate],
            special_index: 0,
        };
        let out = compiler.push_ugen(ugen, spec.meta);
        compiler.push_outputs(out);
    }
}
