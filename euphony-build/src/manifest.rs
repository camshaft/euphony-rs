#![allow(dead_code)]

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    #[serde(default)]
    pub synths: HashMap<String, Synth>,
    #[serde(default)]
    pub samples: HashMap<String, Sample>,
    #[serde(default)]
    pub wavetable: HashMap<String, Wavetable>,
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Synth {
    Url(String),
    Config {
        url: String,
        // TODO other options
    },
}

impl Synth {
    pub fn url(&self) -> &str {
        match self {
            Self::Url(url) => url,
            Self::Config { url, .. } => url,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Sample {
    Url(String),
    Config {
        url: String,
        // TODO other options
    },
}

impl Sample {
    pub fn url(&self) -> &str {
        match self {
            Self::Url(url) => url,
            Self::Config { url, .. } => url,
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum Wavetable {
    Url(String),
    Config {
        url: String,
        // TODO other options
    },
}

impl Wavetable {
    pub fn url(&self) -> &str {
        match self {
            Self::Url(url) => url,
            Self::Config { url, .. } => url,
        }
    }
}
