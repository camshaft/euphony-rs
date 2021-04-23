use serde::Serialize;
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Default, Serialize)]
pub struct Manifest {
    pub tracks: BTreeMap<String, Track>,
}

#[derive(Debug, Serialize)]
pub struct Track {
    pub path: PathBuf,
}
