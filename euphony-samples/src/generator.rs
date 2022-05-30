use euphony_buffer::{decode, hash, symphonia::core::errors::Result};
use rayon::prelude::*;
use std::{
    collections::BTreeMap,
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

const SAMPLE_RATE: u32 = 48_000;
const SAMPLE_DIR: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/samples");

#[test]
fn generate() {
    if std::env::var("EUPHONY_SAMPLES_GEN").is_err() {
        return;
    }

    fs::create_dir_all(SAMPLE_DIR).unwrap();

    let mut groups = vec![];

    dirt(&mut groups);
    sonic_pi(&mut groups);

    groups.sort_by(|a, b| a.path.cmp(&b.path));

    for g in &groups {
        eprintln!("{:?}", &g.path);
    }
}

#[derive(Default)]
struct GroupNode {
    children: BTreeMap<String, GroupNode>,
    samples: Vec<Sample>,
}

impl GroupNode {
    pub fn insert(&mut self, mut group: Group) {
        if let Some(child) = group.path.pop() {
            let c = self.children.entry(child).or_default();
            c.insert(group);
            return;
        }

        self.samples.extend(group.samples);
    }

    pub fn samples<W: Write>(&self, w: &mut W) -> io::Result<()> {
        writeln!(w, "&[")?;
        for sample in &self.samples {
            let hash = sample.path.file_stem().unwrap().to_str().unwrap();
            writeln!(w, "b!({:?}),", hash)?;
        }
        writeln!(w, "]")
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
    pub fn new(path: &Path) -> Result<Self> {
        let file = fs::File::open(path)?;
        let ext = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");
        let mut reader = decode::reader(file, ext)?;
        let mut samples = decode::Samples::<f32>::from_reader(&mut *reader)?;

        if samples.channels.is_empty() {
            return Err(io::Error::new(io::ErrorKind::Other, "empty file").into());
        }

        samples
            .resample(SAMPLE_RATE)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

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
    let entries = concat!(env!("CARGO_MANIFEST_DIR"), "/etc/dirt");
    let mut dirs = vec![];
    for entry in fs::read_dir(entries).unwrap() {
        let entry = entry.unwrap();
        if !entry.file_type().unwrap().is_dir() {
            continue;
        }

        dirs.push(entry.path());
    }

    let g = dirs.par_iter().filter_map(|dir| {
        let name = dir.file_name().unwrap().to_str().unwrap();
        let mut files = vec![];
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            if !entry.file_type().unwrap().is_file() {
                continue;
            }

            files.push(entry.path());
        }

        let samples = files
            .par_iter()
            .filter_map(|sample| Sample::new(sample).ok())
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
    let entries = concat!(env!("CARGO_MANIFEST_DIR"), "/etc/sonic-pi/etc/samples");
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
            Sample::new(path).ok().map(|s| {
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
