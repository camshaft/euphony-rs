use proc_macro2::Ident;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::{
    fs::File,
    io::{self, BufReader, Seek, Write},
    path::Path,
};

pub fn compile(name: &Ident, url: &str, input: &Path, output: &Path) -> io::Result<()> {
    let mut out_file = vec![];

    if let Some(path) = url.strip_prefix("clean://") {
        let mut file = File::open(input)?;
        let file = BufReader::new(&mut file);
        let manifest: Clean = serde_json::from_reader(file)?;

        let mut parts = path.split('/');
        let mut base = url::Url::parse("https://raw.githubusercontent.com").unwrap();
        let mut segments = base.path_segments_mut().unwrap();
        segments.push(parts.next().unwrap());
        segments.push(parts.next().unwrap());

        segments.push(if let Some(version) = path.split('?').nth(1) {
            version
        } else {
            "main"
        });

        segments.push("");

        drop(segments);

        let out_dir = output.parent().unwrap();

        writeln!(
            out_file,
            "pub static {}: &BufDefGroup = &BufDefGroup {{",
            name
        )?;
        writeln!(out_file, "    buffers: &[")?;
        for sound in &manifest.sounds {
            let url = base.join(&sound.filename).unwrap();
            let sound_path = crate::download(url.as_str(), out_dir)?;
            compile_file(&sound_path, &mut out_file)?;
            writeln!(out_file, ",")?;
        }
        writeln!(out_file, "    ]")?;
        writeln!(out_file, "}};")?;
    } else {
        write!(out_file, "pub static {}: &BufDef = ", name)?;
        compile_file(input, &mut out_file)?;
        writeln!(out_file, ";")?;
    }

    std::fs::write(output, out_file)?;

    Ok(())
}

fn compile_file<W: Write>(input: &Path, o: &mut W) -> io::Result<()> {
    let mut file = File::open(input)?;
    let mut file = BufReader::new(&mut file);

    let reader = hound::WavReader::new(&mut file)
        .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
    let spec = reader.spec();

    let channels = spec.channels;
    let sample_rate = spec.sample_rate;
    let bit_depth = spec.bits_per_sample;
    let frames = reader.duration();

    // TODO convert to PCM wav if needed

    let mut file = reader.into_inner();
    file.seek(io::SeekFrom::Start(0))?;
    let mut hasher = Sha256::default();
    std::io::copy(&mut file, &mut hasher)?;
    let hash = hasher.finalize();

    let hash_str = base64::encode_config(&hash, base64::URL_SAFE_NO_PAD);

    writeln!(o, "&BufDef {{")?;
    writeln!(o, "    channels: {},", channels)?;
    writeln!(o, "    sample_rate: {},", sample_rate)?;
    writeln!(o, "    bit_depth: {},", bit_depth)?;
    writeln!(o, "    frames: {},", frames)?;
    writeln!(o, "    contents: {{")?;
    match std::env::var("PROFILE").as_deref() {
        Ok("release") => {
            writeln!(
                o,
                "        static FILE: InlineFile = InlineFile::new({:?}, &{:?}, include_bytes!({:?}));",
                hash_str,
                hash,
                input.display()
            )?;
        }
        _ => {
            writeln!(
                o,
                "        static FILE: ExternalFile = ExternalFile::new(&{:?}, {:?});",
                hash,
                input.display()
            )?;
        }
    }
    writeln!(o, "        &FILE")?;
    writeln!(o, "    }}")?;
    write!(o, "}}")?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct Clean {
    description: String,
    shortname: String,
    sounds: Vec<Sound>,
}

#[derive(Debug, Deserialize)]
struct Sound {
    filename: String,
    shortname: String,
}
