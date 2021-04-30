#![allow(dead_code)]

use crate::{param::Param, synthdef::CalculationRate};
use core::{fmt, marker::PhantomData};

mod compiler;
mod graph;
pub mod ops;
mod value;

pub use compiler::{
    default_compile, Compile, Compiler, Dependency, InputInfo, InputVec, Inputs, UgenMeta,
};
pub use graph::{OutputSpec, Ugen};
pub use value::{Value, ValueVec};

pub trait Parameters: 'static + Sized {
    type Desc;

    fn new<F>(f: F) -> Self
    where
        F: FnOnce(fn() -> Self::Desc) -> SynthDescRef;
}

#[derive(Clone)]
pub struct Synth {
    id: crate::osc::node::Id,
    track: crate::track::Handle,
    synthdef: &'static str,
}

impl Synth {
    pub fn new(
        id: crate::osc::node::Id,
        track: crate::track::Handle,
        synthdef: &'static str,
    ) -> Self {
        Self {
            id,
            track,
            synthdef,
        }
    }

    pub fn name(&self) -> &'static str {
        self.synthdef
    }

    pub fn id(&self) -> crate::osc::node::Id {
        self.id
    }

    pub fn track_name(&self) -> &str {
        use crate::track::Track;
        self.track.name()
    }
}

impl fmt::Debug for Synth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Synth")
            .field("id", &self.id)
            .field("track", &self.track_name())
            .field("synthdef", &self.synthdef)
            .finish()
    }
}

impl Drop for Synth {
    fn drop(&mut self) {
        use crate::track::Track;
        self.track.free(self.id)
    }
}

pub struct SynthDesc {
    name: String,
    desc: Vec<u8>,
}

impl SynthDesc {
    pub fn as_ref(&'static self) -> SynthDescRef {
        SynthDescRef {
            name: &self.name,
            desc: &self.desc,
        }
    }
}

impl fmt::Debug for SynthDesc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthDesc")
            .field("name", &self.name)
            .finish()
    }
}

#[derive(Clone, Copy)]
pub struct SynthDescRef {
    name: &'static str,
    desc: &'static [u8],
}

impl fmt::Debug for SynthDescRef {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthDesc")
            .field("name", &self.name)
            .finish()
    }
}

pub struct SynthDef<Params> {
    desc: SynthDescRef,
    params: PhantomData<Params>,
}

impl<Params> SynthDef<Params> {
    pub fn new(desc: SynthDescRef) -> Self {
        Self {
            desc,
            params: PhantomData,
        }
    }

    pub fn name(&self) -> &'static str {
        &self.desc.name
    }

    pub fn desc(&self) -> &'static [u8] {
        &self.desc.desc
    }
}

impl<Params> fmt::Debug for SynthDef<Params> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SynthDef")
            .field("name", &self.name())
            .finish()
    }
}

use std::cell::RefCell;

pub struct UgenSpec {
    pub name: &'static str,
    pub special_index: i16,
    pub rate: Option<CalculationRate>,
    pub outputs: usize,
    pub meta: UgenMeta,
    pub compile: Compile,
}

impl fmt::Debug for UgenSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("UgenSpec")
            .field("name", &self.name)
            .field("special_index", &self.special_index)
            .finish()
    }
}

impl Default for UgenSpec {
    fn default() -> Self {
        Self {
            name: "",
            special_index: 0,
            rate: None,
            outputs: 1,
            meta: UgenMeta::default(),
            compile: default_compile,
        }
    }
}

#[derive(Debug, Default)]
struct Context {
    params: Vec<compiler::ParamSpec>,
    graph: graph::Graph,
}

impl Context {
    fn build(&mut self, name: &'static str) -> SynthDesc {
        let this = core::mem::take(self);

        let (consts, ugens) = compiler::compile(&this.graph, &this.params);

        let params = this.params.iter().map(|param| param.default).collect();

        let param_names = this
            .params
            .iter()
            .enumerate()
            .map(|(index, param)| crate::synthdef::ParamName {
                name: param.name.to_string(),
                index: index as _,
            })
            .collect();

        let definition = crate::synthdef::Definition {
            name: name.to_string(),
            consts,
            params,
            param_names,
            ugens,
            variants: vec![],
        };

        if let Ok(out) = std::env::var("SYNTHDEF_OUT_DIR") {
            let dir = std::path::Path::new(&out);
            std::fs::create_dir_all(&dir).unwrap();
            let mut path = dir.join(name);
            path.set_extension("dot");

            let file = std::fs::File::create(path).unwrap();
            let mut file = std::io::BufWriter::new(file);

            definition.dot(&mut file).unwrap();
        }

        let container = crate::synthdef::Container {
            version: crate::synthdef::V2 as _,
            defs: vec![definition],
        };

        let desc = container.encode();

        if cfg!(debug_assertions) {
            use codec::decode::DecoderBuffer;
            let (parsed, _) = desc.decode::<crate::synthdef::Container>().unwrap();
            if parsed != container {
                panic!("expected: {:#?}\n actual: {:#?}", container, parsed);
            }
        }

        SynthDesc {
            name: name.to_owned(),
            desc,
        }
    }

    fn param(&mut self, id: u32, name: &'static str, default: f32) -> Param {
        let idx = id as usize;
        if idx >= self.params.len() {
            self.params.resize(idx + 1, compiler::ParamSpec::default());
        }
        self.params[idx] = compiler::ParamSpec {
            name,
            default,
            ..Default::default()
        };
        param_instance(id)
    }

    fn ugen(&mut self, ugen: UgenSpec) -> graph::Ugen {
        self.graph.insert(ugen)
    }
}

thread_local! {
    static CONTEXT: RefCell<Context> = RefCell::new(Context::default());
}

pub fn synthdef<F: FnOnce()>(name: &'static str, b: F) -> SynthDesc {
    b();
    CONTEXT.with(|c| c.take()).build(name)
}

pub fn external_synthdef(name: &'static str, desc: &'static [u8]) -> SynthDescRef {
    SynthDescRef { name, desc }
}

pub fn param(id: u32, name: &'static str, default: f32) -> Param {
    CONTEXT.with(|c| c.borrow_mut().param(id, name, default))
}

pub fn param_instance(id: u32) -> Param {
    let v: Value = value::Parameter::new(id).into();
    v.into()
}

pub fn ugen<F: FnOnce(Ugen) -> O, O>(ugen: UgenSpec, def: F) -> O {
    CONTEXT.with(|c| def(c.borrow_mut().ugen(ugen)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_size_test() {
        assert_eq!(core::mem::size_of::<Value>(), 8);
    }
}
