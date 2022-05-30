use euphony_buffer::{decode, hash, symphonia::core::errors::Result};
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

macro_rules! p {
    ($p:literal) => {
        concat!(env!("CARGO_MANIFEST_DIR"), "/", $p)
    };
}

const SAMPLE_RATE: u32 = 48_000;
const SAMPLE_DIR: &str = p!("etc/euphony-samples");

#[test]
fn generate() {
    if std::env::var("EUPHONY_SAMPLES_GEN").is_err() {
        return;
    }

    fs::create_dir_all(SAMPLE_DIR).unwrap();

    samples().unwrap();
    waveforms().unwrap();
}

fn samples() -> io::Result<()> {
    let mut groups = vec![];

    dirt(&mut groups);
    sonic_pi(&mut groups);

    groups.sort_by(|a, b| a.path.cmp(&b.path));

    let mut root = GroupNode::default();

    for mut g in groups {
        g.path.reverse();
        root.insert(g);
    }

    root.to_file(p!("src/samples.rs"))?;

    Ok(())
}

fn waveforms() -> io::Result<()> {
    let mut groups = vec![];

    akwf(&mut groups);

    groups.sort_by(|a, b| a.path.cmp(&b.path));

    let mut root = GroupNode::default();

    for mut g in groups {
        g.path.reverse();
        root.insert(g);
    }

    root.to_file(p!("src/waveforms.rs"))?;

    Ok(())
}

#[derive(Default)]
struct GroupNode {
    children: BTreeMap<String, GroupNode>,
    samples: Vec<Sample>,
}

impl GroupNode {
    pub fn to_file(&self, out: &str) -> io::Result<()> {
        let s = fs::File::create(out)?;
        let mut s = io::BufWriter::new(s);

        writeln!(s, "#![allow(non_upper_case_globals)]")?;

        self.write("", &mut s)
    }

    pub fn insert(&mut self, mut group: Group) {
        if let Some(mut child) = group.path.pop() {
            if child == "808" {
                child = "tr808".to_owned();
            } else if child == "909" {
                child = "tr909".to_owned();
            } else if child == "loop" {
                child = "loops".to_owned();
            } else if child == "if" {
                child = "iff".to_owned();
            } else if child == "3d_printer" {
                child = "printer".to_owned();
            }
            let c = self.children.entry(child).or_default();
            c.insert(group);
            return;
        }

        self.samples.extend(group.samples);
    }

    fn write<W: Write>(&self, name: &str, w: &mut W) -> io::Result<()> {
        if !self.samples.is_empty() {
            writeln!(w, "g!({},", name)?;
            for sample in &self.samples {
                let hash = sample.path.file_stem().unwrap().to_str().unwrap();
                writeln!(w, "{:?},", hash)?;
            }
            writeln!(w, ");")?
        }

        if self.children.is_empty() {
            return Ok(());
        }

        if !name.is_empty() {
            writeln!(w, "pub mod {} {{", name)?;
        }
        for (name, group) in &self.children {
            group.write(name, w)?;
        }
        if !name.is_empty() {
            writeln!(w, "}}")?;
        }

        Ok(())
    }
}

struct Group {
    path: Vec<String>,
    samples: Vec<Sample>,
}

struct Sample {
    path: PathBuf,
}

impl Sample {
    pub fn new(path: &Path, resample: bool) -> Result<Self> {
        let file = fs::File::open(path)?;
        let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let mut reader = decode::reader(file, ext)?;
        let mut samples = decode::Samples::<f32>::from_reader(&mut *reader)?;

        if samples.channels.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "empty file").into());
        }

        if resample {
            samples
                .resample(SAMPLE_RATE)
                .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
        }

        if samples.channels.len() == 2 {
            // if the 2 channels are the same then just make it mono
            if samples.channels[0] == samples.channels[1] {
                samples.channels.pop();
            }
        }

        let frames = samples.channels[0].len() as u64;
        let path = hash::create(Path::new(SAMPLE_DIR), "wav", |file| {
            let spec = hound::WavSpec {
                channels: samples.channels.len() as _,
                sample_rate: SAMPLE_RATE,
                bits_per_sample: 32,
                sample_format: hound::SampleFormat::Float,
            };
            let mut writer = hound::WavWriter::new(file, spec).unwrap();
            for frame in 0..frames {
                for channel in samples.channels.iter() {
                    let sample = channel[frame as usize];
                    writer.write_sample(sample).unwrap();
                }
            }
            writer.finalize().unwrap();
            Ok(())
        })?;

        Ok(Self { path })
    }
}

fn dirt(groups: &mut Vec<Group>) {
    let entries = p!("etc/Dirt-Samples");
    let dirs = dirs(Path::new(entries));

    let g = dirs.par_iter().filter_map(|dir| {
        let name = dir.file_name().unwrap().to_str().unwrap();
        let files = files(dir);

        let samples = files
            .par_iter()
            .filter_map(|sample| Sample::new(sample, true).ok())
            .collect::<Vec<_>>();

        if samples.is_empty() {
            return None;
        }

        let mut path = vec!["dirt".to_owned()];

        if let Some(name) = name.strip_prefix("808") {
            path.push("808".to_owned());
            if !name.is_empty() {
                path.push(name.to_owned());
            }
        } else {
            path.push(name.to_owned());
        }

        Some(Group { path, samples })
    });

    groups.par_extend(g);
}

fn sonic_pi(groups: &mut Vec<Group>) {
    let entries = p!("etc/sonic-pi/etc/samples");
    let mut samples = vec![];
    for entry in fs::read_dir(entries).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        samples.push(entry.path());
    }

    let samples: Vec<_> = samples
        .par_iter()
        .filter_map(|path| {
            Sample::new(path, true).ok().map(|s| {
                let stem = path.file_stem().unwrap().to_str().unwrap().to_owned();
                (stem, s)
            })
        })
        .collect();

    for (stem, sample) in samples {
        let (prefix, name) = stem.split_once('_').unwrap();
        let path = vec!["sonic_pi".to_owned(), prefix.to_owned(), name.to_owned()];
        groups.push(Group {
            path,
            samples: vec![sample],
        })
    }
}

fn akwf(groups: &mut Vec<Group>) {
    let entries = p!("etc/AKWF-FREE/AKWF");
    let dirs = dirs(Path::new(entries));

    let g = dirs.par_iter().filter_map(|dir| {
        let name = dir.file_name().unwrap().to_str().unwrap();

        let files = files(dir);

        let samples = files
            .par_iter()
            .filter_map(|sample| {
                if sample.extension().and_then(|e| e.to_str()) != Some("wav") {
                    return None;
                }
                Sample::new(sample, false).ok()
            })
            .collect::<Vec<_>>();

        if samples.is_empty() {
            return None;
        }

        let mut path = vec!["akwf".to_owned()];

        if name.starts_with("AKWF_00") {
            // uncategorized - put in the root
        } else if let Some(name) = name.strip_prefix("AKWF_bw_") {
            path.push("bw".to_owned());
            path.push(name.to_owned());
        } else {
            let name = name.strip_prefix("AKWF_").unwrap();
            path.push(name.to_owned());
        }

        Some(Group { path, samples })
    });

    groups.par_extend(g);
}

fn dirs(dir: &Path) -> Vec<PathBuf> {
    let mut dirs = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        dirs.push(entry.path());
    }
    dirs
}

fn files(dir: &Path) -> Vec<PathBuf> {
    let mut files = vec![];
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_file() {
            continue;
        }

        files.push(entry.path());
    }
    files
}
