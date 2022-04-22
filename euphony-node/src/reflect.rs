use heck::{ToPascalCase, ToSnakeCase};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs, io,
    path::Path,
};

pub fn generate_files(manifest_dir: &str) {
    let manifest_dir = Path::new(manifest_dir);
    if let Ok(mut nodes) = load(manifest_dir) {
        nodes.sort_by(|a, b| a.id.cmp(&b.id));

        let mut o = io::Cursor::new(vec![]);
        nodes_file(&mut o, &nodes).unwrap();

        sync_file(&manifest_dir.join("src").join("nodes.rs"), o.into_inner());
    }
}

pub fn generate_api(source_dir: &str, output: &str) {
    let source_dir = Path::new(source_dir);
    if let Ok(nodes) = load(source_dir) {
        let mut o = io::Cursor::new(vec![]);
        generate_api_impl(&mut o, nodes).unwrap();
        sync_file(Path::new(output), o.into_inner());
    }
}

fn generate_api_impl<O: io::Write>(o: &mut O, mut nodes: Vec<Node>) -> io::Result<()> {
    let mut modules = Module::default();
    let mut inputs = BTreeSet::new();
    let mut ext = vec![];

    // sink parameters
    inputs.insert("azimuth".to_string());
    inputs.insert("incline".to_string());
    inputs.insert("radius".to_string());

    for mut node in nodes.drain(..) {
        node.module.reverse();

        if node.module == ["unary"] || node.module == ["binary"] || node.module == ["tertiary"] {
            ext.push((
                node.name.clone(),
                node.docs.clone(),
                node.inputs
                    .iter()
                    .map(|i| i.name.clone())
                    .collect::<Vec<_>>(),
            ));
        }

        for input in node.inputs.iter() {
            inputs.insert(input.name.clone());
        }

        modules.insert(node);
    }

    ext.sort();

    let mut level = 0;
    macro_rules! w {
        () => {{
            writeln!(o)?;
        }};
        ($($t:tt)*) => {{
            for _ in 0..(level * 4) {
                write!(o, " ")?;
            }
            writeln!(o, $($t)*)?;
        }}
    }

    w!("#[rustfmt::skip]");
    w!("pub mod ext {{");
    level += 1;
    w!("use crate::parameter::Parameter;");
    w!("use super::input::*;");
    w!("pub trait ProcessorExt: crate::processor::Processor");
    w!("where");
    w!("    for<'a> &'a Self: Into<Parameter>,");
    w!("{{");
    level += 1;
    for (name, docs, inputs) in ext {
        let lower = name.to_snake_case();
        w!("#[inline]");
        w!("#[doc = {:?}]", docs);
        match inputs.len() {
            1 => w!("fn {lower}(&self) -> crate::processors::unary::{name} {{"),
            2 => {
                let rhs = &inputs[1];
                let rhs_upper = rhs.to_pascal_case();
                w!("fn {lower}<{rhs_upper}>(&self, {rhs}: {rhs_upper}) -> crate::processors::binary::{name}");
                w!("where");
                w!("    {rhs_upper}: Into<Parameter>,");
                w!("{{");
            }
            3 => {
                let b = &inputs[1];
                let b_upper = b.to_pascal_case();
                let c = &inputs[2];
                let c_upper = c.to_pascal_case();
                w!("fn {lower}<{b_upper}, {c_upper}>(&self, {b}: {b_upper}, {c}: {c_upper}) -> crate::processors::tertiary::{name}");
                w!("where");
                w!("    {b_upper}: Into<Parameter>,");
                w!("    {c_upper}: Into<Parameter>,");
                w!("{{");
            }
            _ => unreachable!(),
        };
        level += 1;
        match inputs.len() {
            1 => {
                let input = &inputs[0];
                w!("crate::processors::unary::{lower}().with_{input}(self)")
            }
            2 => {
                let lhs = &inputs[0];
                let rhs = &inputs[1];
                w!("crate::processors::binary::{lower}().with_{lhs}(self).with_{rhs}({rhs})")
            }
            3 => {
                let a = &inputs[0];
                let b = &inputs[1];
                let c = &inputs[2];
                w!("crate::processors::tertiary::{lower}().with_{a}(self).with_{b}({b}).with_{c}({c})")
            }
            _ => unreachable!(),
        };
        level -= 1;
        w!("}}");
    }
    level -= 1;
    w!("}}");

    w!("impl<T> ProcessorExt for T");
    w!("where");
    w!("    Self: crate::processor::Processor,");
    w!("    for<'a> &'a Self: Into<Parameter>,");
    w!("{{}}");

    level -= 1;
    w!("}}");

    w!("pub mod input {{");
    level += 1;
    for input in &inputs {
        w!("#[allow(non_camel_case_types)]");
        w!("pub trait {}<Value> {{", input);
        w!("    fn with_{}(self, value: Value) -> Self;", input);
        w!("    fn set_{}(&self, value: Value) -> &Self;", input);
        w!("}}");
    }
    level -= 1;
    w!("}}");
    w!();

    w!("#[rustfmt::skip]");
    w!("mod api {{");
    modules.write(o, 1)?;
    w!("}}");
    w!("pub use api::*;");

    Ok(())
}

fn load(manifest_dir: &Path) -> io::Result<Vec<Node>> {
    let manifest_dir = Path::new(manifest_dir);
    let mut nodes = vec![];

    for file in fs::read_dir(manifest_dir.join("nodes"))? {
        let file = file?;
        let path = file.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|v| v.to_str()) != Some("json") {
            continue;
        }

        let file = fs::File::open(path)?;
        let node: Node = serde_json::from_reader(file)?;
        nodes.push(node);
    }

    Ok(nodes)
}

fn nodes_file<O: io::Write>(o: &mut O, nodes: &[Node]) -> io::Result<()> {
    macro_rules! w {
        ($($t:tt)*) => {
            writeln!(o, $($t)*)?;
        }
    }

    w!("#![deny(unreachable_patterns)]");
    w!();

    w!("#[rustfmt::skip]");
    w!("use euphony_node::{{BoxProcessor, Error, ParameterValue as Value}};");
    w!();

    w!("#[rustfmt::skip]");
    w!("#[inline]");
    w!("pub fn load(processor: u64) -> Option<BoxProcessor> {{");
    w!("    match processor {{");
    for node in nodes {
        w!("        {} => Some({}::spawn()),", node.id, node.path());
    }
    w!("        _ => None,");
    w!("    }}");
    w!("}}");
    w!();

    w!("#[rustfmt::skip]");
    w!("#[inline]");
    w!("pub fn name(processor: u64) -> Option<&'static str> {{");
    w!("    match processor {{");
    for node in nodes {
        w!("        {} => Some({:?}),", node.id, node.name);
    }
    w!("        _ => None,");
    w!("    }}");
    w!("}}");
    w!();

    w!("#[rustfmt::skip]");
    w!("#[inline]");
    w!("pub fn validate_parameter(processor: u64, parameter: u64, value: Value) -> Result<(), Error> {{");
    w!("    match processor {{");
    for node in nodes {
        w!(
            "        {} => {}::validate_parameter(parameter, value),",
            node.id,
            node.path()
        );
    }
    w!("        _ => unreachable!(\"processor ({{}}) param ({{}}) doesn't exist\", processor, parameter)");
    w!("    }}");
    w!("}}");

    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Node {
    pub name: String,
    pub module: Vec<String>,
    pub impl_path: String,
    pub id: u64,
    pub inputs: Vec<Input>,
    pub docs: String,
}

impl Node {
    fn path(&self) -> String {
        let (_, impl_path) = self.impl_path.split_once("::").unwrap();
        format!("crate::{}::{}", impl_path, self.name)
    }

    pub fn test(&self, manifest_dir: &str) {
        assert_ne!(self.id, 0, "processor id 0 is reserved for Sink");

        let dir = Path::new(manifest_dir).join("nodes");
        fs::create_dir_all(&dir).unwrap();
        let mut module = self.module.join("__");

        if !module.is_empty() {
            module += "__";
        }

        let path = dir.join(format!("{}{}.json", module, self.name));
        let contents = serde_json::to_string_pretty(self).unwrap();
        sync_file(&path, contents);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub id: u64,
    pub trigger: bool,
    pub default: f64,
}

fn sync_file<C: AsRef<[u8]>>(path: &Path, contents: C) {
    let should_write = fs::read(path)
        .ok()
        .map_or(true, |prev| prev != contents.as_ref());

    if should_write {
        fs::write(path, contents).unwrap();
    }
}

#[derive(Debug, Default)]
struct Module {
    nodes: Vec<Node>,
    modules: BTreeMap<String, Module>,
}

impl Module {
    pub fn insert(&mut self, mut node: Node) {
        if let Some(m) = node.module.pop() {
            self.modules.entry(m).or_default().insert(node);
        } else {
            self.nodes.push(node);
        }
    }

    pub fn write<O: io::Write>(&mut self, o: &mut O, mut level: usize) -> io::Result<()> {
        macro_rules! w {
            () => {
                writeln!(o)?;
            };
            ($($t:tt)*) => {
                for _ in 0..(level * 4) {
                    write!(o, " ")?;
                }
                writeln!(o, $($t)*)?;
            }
        }

        self.nodes.sort_by(|a, b| a.id.cmp(&b.id));

        for (idx, node) in self.nodes.iter().enumerate() {
            if idx > 0 {
                w!();
            }

            w!("define_processor!(");
            level += 1;

            if !node.docs.is_empty() {
                w!("#[doc = {:?}]", node.docs);
            }

            w!("#[id = {}]", node.id);
            w!("#[lower = {}]", node.name.to_snake_case());
            w!("struct {} {{", node.name);
            level += 1;

            for input in &node.inputs {
                w!("#[with = with_{}]", input.name);
                w!("#[set = set_{}]", input.name);
                if input.trigger {
                    w!("{}: f64<{}>,", input.name, input.id);
                } else {
                    w!("{}: Parameter<{}>,", input.name, input.id);
                }
            }

            level -= 1;
            w!("}}");
            level -= 1;
            w!(");");
        }

        if !self.modules.is_empty() {
            w!();
        }

        for (name, module) in &mut self.modules {
            w!("pub mod {} {{", name);
            level += 1;
            module.write(o, level)?;
            level -= 1;
            w!("}}");
        }

        Ok(())
    }
}
