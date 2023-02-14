use base64::URL_SAFE_NO_PAD;
use std::{
    io::{self, Read, Seek, Write},
    path::{Path, PathBuf},
};
use tempfile::NamedTempFile;

pub type Hash = [u8; 32];

pub fn join_path(root: &Path, hash: &Hash, ext: &str) -> PathBuf {
    let mut out = [b'A'; 64];
    let len = base64::encode_config_slice(hash, URL_SAFE_NO_PAD, &mut out);
    let out = unsafe { core::str::from_utf8_unchecked_mut(&mut out) };
    let mut path = root.join(&out[..len]);
    if !ext.is_empty() {
        path.set_extension(ext);
    }
    path
}

pub fn reader(r: &mut impl Read) -> Hash {
    let mut hasher = blake3::Hasher::new();
    let mut buf = [0; 4096];
    loop {
        let len = r.read(&mut buf).unwrap();

        if len == 0 {
            return *hasher.finalize().as_bytes();
        }

        hasher.update(&buf[..len]);
    }
}

pub fn create<W: FnOnce(&mut io::BufWriter<NamedTempFile>) -> io::Result<()>>(
    root: &Path,
    ext: &str,
    write: W,
) -> io::Result<PathBuf> {
    let tmp = tempfile::NamedTempFile::new_in("target/euphony/tmp")?;

    let mut buf = io::BufWriter::new(tmp);
    write(&mut buf)?;
    buf.flush()?;

    let mut tmp = buf.into_inner()?;

    tmp.rewind()?;

    let hash = reader(&mut tmp);
    let path = join_path(root, &hash, ext);
    tmp.persist(&path)?;
    Ok(path)
}
