use crate::synthdef::{
    builder::{ops, ugen, Compiler, Dependency, Inputs, UgenMeta, UgenSpec, Value, ValueVec},
    CalculationRate, UGen,
};

#[derive(Clone, Debug)]
pub struct Out {
    pub buffer: Value,
    pub channels: ValueVec,
}

impl Out {
    pub fn new(buffer: impl Into<Value>, channels: impl Into<ValueVec>) -> Self {
        Self {
            buffer: buffer.into(),
            channels: channels.into(),
        }
    }

    pub fn ar(self) {
        self.build(Some(CalculationRate::Audio))
    }

    pub fn kr(self) {
        self.build(Some(CalculationRate::Control))
    }

    pub fn build(self, rate: Option<CalculationRate>) {
        let Self { buffer, channels } = self;
        ugen(
            UgenSpec {
                name: "Out",
                meta: UgenMeta {
                    is_pure: false,
                    is_deterministic: true,
                },
                rate,
                outputs: 0,
                compile: compile_out,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(buffer);
                ugen.input(channels);
                ugen.finish()
            },
        )
    }
}

fn compile_out(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let mut inputs = inputs.iter();
    let buffer = inputs.next().expect("buffer");
    let channels = inputs.next().expect("channels");

    let mut ins = vec![];
    let mut rate = spec.rate.unwrap_or(CalculationRate::Scalar);
    ins.push(buffer[0][0].input);
    rate = rate.max(buffer[0][0].rate);

    for channel in channels.iter().flatten() {
        ins.push(channel.input);
        rate = rate.max(channel.rate);
    }

    let ugen = UGen {
        name: spec.name.into(),
        special_index: spec.special_index,
        rate,
        ins,
        outs: vec![],
    };

    let outputs = compiler.push_ugen(ugen, spec.meta);
    compiler.push_outputs(outputs);
}

#[derive(Clone, Debug)]
pub struct SinOsc {
    pub freq: ValueVec,
    pub phase: ValueVec,
}

impl Default for SinOsc {
    fn default() -> Self {
        Self {
            freq: (440.0).into(),
            phase: (0.0).into(),
        }
    }
}

impl SinOsc {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn freq(mut self, freq: impl Into<ValueVec>) -> Self {
        self.freq = freq.into();
        self
    }

    pub fn ar(self) -> Value {
        self.build(Some(CalculationRate::Audio))
    }

    pub fn kr(self) -> Value {
        self.build(Some(CalculationRate::Control))
    }

    pub fn build(self, rate: Option<CalculationRate>) -> Value {
        let Self { freq, phase } = self;

        ugen(
            UgenSpec {
                name: "SinOsc",
                rate,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(freq);
                ugen.input(phase);
                ugen.finish()
            },
        )
    }
}

#[derive(Clone, Debug)]
pub struct Mix {
    pub signals: ValueVec,
}

impl Mix {
    pub fn new(signals: impl Into<ValueVec>) -> Self {
        Self {
            signals: signals.into(),
        }
    }

    pub fn ar(self) -> Value {
        self.build(Some(CalculationRate::Audio))
    }

    pub fn kr(self) -> Value {
        self.build(Some(CalculationRate::Control))
    }

    pub fn build(self, rate: Option<CalculationRate>) -> Value {
        let Self { signals } = self;
        ugen(
            UgenSpec {
                rate,
                compile: compile_mix,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(signals);
                ugen.finish()
            },
        )
    }
}

fn compile_mix(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let channels = inputs.iter().next().expect("only one param");
    for signals in channels.iter() {
        match signals.len() {
            0 => continue,
            1 => compiler.push_outputs(signals.clone()),
            _ => {
                let rate = spec.rate.unwrap_or(CalculationRate::Audio); // TODO get from the signals
                let first = signals[0];
                let out = signals[1..].iter().fold(first, |acc, signal| {
                    ops::compile_add(acc.input, signal.input, rate, compiler)
                });
                compiler.push_output(out);
            }
        }
    }
}

pub struct Pan2 {
    pub signal: ValueVec,
    pub pos: ValueVec,
    pub level: ValueVec,
}

impl Pan2 {
    pub fn new(signal: impl Into<ValueVec>) -> Self {
        Self {
            signal: signal.into(),
            pos: 0.into(),
            level: 1.into(),
        }
    }

    pub fn pos(mut self, pos: impl Into<ValueVec>) -> Self {
        self.pos = pos.into();
        self
    }

    pub fn level(mut self, level: impl Into<ValueVec>) -> Self {
        self.level = level.into();
        self
    }

    pub fn ar(self) -> [Value; 2] {
        self.build(Some(CalculationRate::Audio))
    }

    pub fn kr(self) -> [Value; 2] {
        self.build(Some(CalculationRate::Control))
    }

    pub fn build(self, rate: Option<CalculationRate>) -> [Value; 2] {
        ugen(
            UgenSpec {
                name: "Pan2",
                rate,
                outputs: 2,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(self.signal);
                ugen.input(self.pos);
                ugen.input(self.level);
                ugen.finish()
            },
        )
    }
}

pub struct Splay {
    pub signal: ValueVec,
    pub spread: Value,
    pub level: Value,
    pub center: Value,
    pub level_comp: bool,
}

impl Splay {
    pub fn new(signal: impl Into<ValueVec>) -> Self {
        Self {
            signal: signal.into(),
            spread: Value::from(1.0),
            level: Value::from(1.0),
            center: Value::from(0.0),
            level_comp: true,
        }
    }

    pub fn spread(mut self, spread: impl Into<Value>) -> Self {
        self.spread = spread.into();
        self
    }

    pub fn level(mut self, level: impl Into<Value>) -> Self {
        self.level = level.into();
        self
    }

    pub fn center(mut self, center: impl Into<Value>) -> Self {
        self.center = center.into();
        self
    }

    pub fn ar(self) -> Value {
        self.build(CalculationRate::Audio)
    }

    pub fn kr(self) -> Value {
        self.build(CalculationRate::Control)
    }

    pub fn build(self, rate: CalculationRate) -> Value {
        let Self {
            signal,
            spread,
            level,
            center,
            level_comp,
        } = self;

        let size = Size::ir(signal.clone());
        //        let size = size.max(2);
        let n1 = size - 1;
        let range = Range::ir(Value::from(0)..size);
        let positions = (range * (2 / n1) - 1) * spread + center;

        let signal = Pan2::new(signal).pos(positions).build(Some(rate));
        let signal = Mix::new(signal).build(Some(rate));

        if !level_comp {
            return signal * level;
        }

        let level = if rate == CalculationRate::Audio {
            // TODO level * n.recip().sqrt()
            level * (1 / size)
        } else {
            level / size
        };

        signal * level
    }
}

pub struct Size;

impl Size {
    pub fn ir(signal: impl Into<ValueVec>) -> Value {
        ugen(
            UgenSpec {
                compile: compile_size,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(signal);
                ugen.finish()
            },
        )
    }
}

fn compile_size(_spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let len = inputs.expand().len();
    let output = compiler.push_consts(&[len as f32]);
    compiler.push_outputs(output);
}

pub struct Range;

impl Range {
    pub fn ir(range: core::ops::Range<impl Into<ValueVec>>) -> Value {
        ugen(
            UgenSpec {
                compile: compile_range,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(range.start);
                ugen.input(range.end);
                ugen.finish()
            },
        )
    }
}

fn compile_range(_spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    for expansion in inputs.expand().iter() {
        let mut inputs = expansion.inputs();
        let lhs = inputs.next().unwrap();
        let rhs = inputs.next().unwrap();

        if let (Dependency::Const(lhs), Dependency::Const(rhs)) =
            (compiler.resolve(lhs), compiler.resolve(rhs))
        {
            let lhs = lhs as i32;
            let rhs = rhs as i32;
            for idx in lhs..rhs {
                let output = compiler.push_const(idx as f32);
                compiler.push_output(output);
            }
        } else {
            panic!("invalid range spec");
        }
    }
}
