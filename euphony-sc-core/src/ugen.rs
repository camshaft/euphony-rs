use crate::synthdef::{
    builder::{ops, ugen, Compiler, Dependency, Inputs, UgenMeta, UgenSpec, Value, ValueVec},
    CalculationRate, UGen,
};

#[macro_use]
mod macros;

pub mod generator;
pub mod io;
pub mod multichannel;

pub mod prelude {
    use super::*;

    pub use super::Splay;
    pub use generator::*;
    pub use io::*;
    pub use multichannel::*;
}

pub struct Pan2 {
    pub signal: ValueVec,
    pub pos: ValueVec,
    pub level: ValueVec,
    pub rate: Option<CalculationRate>,
}

impl Pan2 {
    pub fn new(signal: impl Into<ValueVec>) -> Self {
        Self {
            signal: signal.into(),
            pos: 0.into(),
            level: 1.into(),
            rate: None,
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

    pub fn rate(mut self, rate: CalculationRate) -> Self {
        self.rate = Some(rate);
        self
    }

    pub fn ar(self) -> [Value; 2] {
        self.rate(CalculationRate::Audio).build()
    }

    pub fn kr(self) -> [Value; 2] {
        self.rate(CalculationRate::Control).build()
    }

    pub fn build(self) -> [Value; 2] {
        let Self {
            signal,
            pos,
            level,
            rate,
        } = self;
        ugen(
            UgenSpec {
                name: "Pan2",
                rate,
                outputs: 2,
                ..Default::default()
            },
            |mut ugen| {
                ugen.input(signal);
                ugen.input(pos);
                ugen.input(level);
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
    pub rate: Option<CalculationRate>,
}

impl Splay {
    pub fn new(signal: impl Into<ValueVec>) -> Self {
        Self {
            signal: signal.into(),
            spread: Value::from(1.0),
            level: Value::from(1.0),
            center: Value::from(0.0),
            level_comp: true,
            rate: None,
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

    pub fn rate(mut self, rate: CalculationRate) -> Self {
        self.rate = Some(rate);
        self
    }

    pub fn ar(self) -> Value {
        self.rate(CalculationRate::Audio).build()
    }

    pub fn kr(self) -> Value {
        self.rate(CalculationRate::Control).build()
    }

    pub fn build(self) -> Value {
        let Self {
            signal,
            spread,
            level,
            center,
            level_comp,
            rate,
        } = self;

        use crate::ugen::prelude::*;

        let size = Size::ir(signal.clone());
        //        let size = size.max(2);
        let n1 = size - 1;
        let range = Range::ir(Value::from(0)..size);
        let positions = (range * (2 / n1) - 1) * spread + center;

        let signal = Pan2::new(signal).pos(positions).build();
        let signal = Mix::new(signal).xr();

        if !level_comp {
            return signal * level;
        }

        // TODO get calculation rate from ugen
        let level = if rate == Some(CalculationRate::Audio) {
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
