use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

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

    let mut loader = "
#[deny(unreachable_patterns)]
pub fn load(id: u64) -> Option<::euphony_node::BoxProcessor> {
    match id {
"
    .trim()
    .to_string();

    for node in &nodes {
        let (_, impl_path) = node.impl_path.split_once("::").unwrap();
        loader.push_str(&format!(
            "\n        {} => Some(crate::{}::{}::spawn()),",
            node.id, impl_path, node.name
        ));
    }

    loader.push_str(
        "
        _ => None,
    }
}",
    );

    sync_file(&manifest_dir.join("src").join("loader.rs"), &loader);
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
    pub fn test(&self, manifest_dir: &str) {
        let dir = Path::new(manifest_dir).join("nodes");
        fs::create_dir_all(&dir).unwrap();
        let module = self
            .module
            .as_ref()
            .map_or("".to_string(), |m| m.replace("::", "__") + "__");
        let path = dir.join(format!("{}{}.{}.json", module, self.name, self.id));
        let contents = serde_json::to_string_pretty(self).unwrap();
        sync_file(&path, &contents);
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    pub name: String,
    pub id: u64,
    pub trigger: bool,
    pub default: f64,
}

fn sync_file(path: &Path, contents: &str) {
    let should_write = fs::read_to_string(path)
        .ok()
        .map_or(true, |prev| prev != contents);

    if should_write {
        fs::write(path, contents).unwrap();
    }
}
