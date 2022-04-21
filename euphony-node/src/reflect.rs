use serde::{Deserialize, Serialize};
use std::{fs, io, path::Path};

pub fn generate_files(manifest_dir: &str) {
    let manifest_dir = Path::new(manifest_dir);
    let mut nodes = vec![];

    for file in fs::read_dir(manifest_dir.join("nodes")).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if !path.is_file() {
            continue;
        }
        if path.extension().and_then(|v| v.to_str()) != Some("json") {
            continue;
        }

        let file = fs::File::open(path).unwrap();
        let node: Node = serde_json::from_reader(file).unwrap();
        nodes.push(node);
    }

    nodes.sort_by(|a, b| a.id.cmp(&b.id));

    let mut o = io::Cursor::new(vec![]);
    nodes_file(&mut o, &nodes).unwrap();

    sync_file(&manifest_dir.join("src").join("nodes.rs"), o.into_inner());
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
    pub module: Option<String>,
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
        let module = self
            .module
            .as_ref()
            .map_or("".to_string(), |m| m.replace("::", "__") + "__");
        let path = dir.join(format!("{}{}.{}.json", module, self.name, self.id));
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
