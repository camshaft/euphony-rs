use crate::{ObjectKind, Store, StoreError};
use euphony_daw_identity::Digest;
use std::{
    fs,
    io::{self, Write},
    path::PathBuf,
    process,
    sync::atomic::{AtomicU64, Ordering},
};

/// Objects in a directory, one file each.
///
/// Publication writes and syncs a staging file, then renames it into place,
/// so an object is never readable in part.
#[derive(Debug)]
pub struct DirStore {
    root: PathBuf,
    staged: AtomicU64,
}

impl DirStore {
    /// Opens the store at `root` to publish into, and removes each staging
    /// file an interrupted publication left, since no object refers to it.
    ///
    /// Only the process that publishes opens a store this way; a process that
    /// only reads opens it with [`DirStore::open_to_read`].
    pub fn open(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let store = Self::open_to_read(root)?;
        let staging = store.root.join("staging");
        fs::create_dir_all(&staging)?;
        for entry in fs::read_dir(&staging)? {
            fs::remove_file(entry?.path())?;
        }
        Ok(store)
    }

    /// Opens the store at `root` to read and publish, leaving staging files
    /// as they are.
    pub fn open_to_read(root: impl Into<PathBuf>) -> Result<Self, StoreError> {
        let root = root.into();
        for kind in ObjectKind::ALL {
            fs::create_dir_all(root.join(kind.dir()))?;
        }
        fs::create_dir_all(root.join("staging"))?;
        Ok(Self {
            root,
            staged: AtomicU64::new(0),
        })
    }

    pub fn root(&self) -> &PathBuf {
        &self.root
    }

    fn path(&self, kind: ObjectKind, digest: &Digest) -> PathBuf {
        self.root.join(kind.dir()).join(digest.to_string())
    }

    /// Writes `bytes` to a staging file and syncs it without publishing it.
    ///
    /// A publication interrupted after this leaves only the staging file,
    /// which no read returns and the next [`DirStore::open`] removes.
    pub fn stage(&self, bytes: &[u8]) -> Result<PathBuf, StoreError> {
        let id = self.staged.fetch_add(1, Ordering::Relaxed);
        let name = format!("{}-{id}", process::id());
        let path = self.root.join("staging").join(name);
        let mut file = fs::File::create(&path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        Ok(path)
    }

    /// The number of staging files present.
    pub fn staged_files(&self) -> Result<usize, StoreError> {
        Ok(fs::read_dir(self.root.join("staging"))?.count())
    }
}

impl Store for DirStore {
    fn publish(&self, kind: ObjectKind, bytes: &[u8]) -> Result<Digest, StoreError> {
        let digest = kind.digest(bytes);
        let path = self.path(kind, &digest);
        if path.exists() {
            return Ok(digest);
        }
        let staged = self.stage(bytes)?;
        fs::rename(&staged, &path)?;
        Ok(digest)
    }

    fn get(&self, kind: ObjectKind, digest: &Digest) -> Result<Vec<u8>, StoreError> {
        let bytes = match fs::read(self.path(kind, digest)) {
            Ok(bytes) => bytes,
            Err(error) if error.kind() == io::ErrorKind::NotFound => {
                return Err(StoreError::Absent(*digest));
            }
            Err(error) => return Err(error.into()),
        };
        if kind.digest(&bytes) != *digest {
            return Err(StoreError::Corrupt(*digest));
        }
        Ok(bytes)
    }

    fn contains(&self, kind: ObjectKind, digest: &Digest) -> bool {
        self.path(kind, digest).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishes_reads_and_reopens() {
        let dir = tempfile::tempdir().unwrap();
        let digest = {
            let store = DirStore::open(dir.path()).unwrap();
            store.publish(ObjectKind::WorkingAudio, b"audio").unwrap()
        };
        let store = DirStore::open(dir.path()).unwrap();
        assert_eq!(store.get(ObjectKind::WorkingAudio, &digest).unwrap(), b"audio");
        assert!(store.contains(ObjectKind::WorkingAudio, &digest));
        assert!(!store.contains(ObjectKind::Manifest, &digest));
    }

    #[test]
    fn an_interrupted_publication_leaves_no_object() {
        let dir = tempfile::tempdir().unwrap();
        let store = DirStore::open(dir.path()).unwrap();
        let digest = ObjectKind::WorkingAudio.digest(b"partial");
        store.stage(b"partial").unwrap();
        assert_eq!(store.staged_files().unwrap(), 1);
        assert!(matches!(
            store.get(ObjectKind::WorkingAudio, &digest),
            Err(StoreError::Absent(_))
        ));
        drop(store);

        let store = DirStore::open(dir.path()).unwrap();
        assert_eq!(store.staged_files().unwrap(), 0);
        assert!(!store.contains(ObjectKind::WorkingAudio, &digest));
    }

    #[test]
    fn detects_damaged_objects() {
        let dir = tempfile::tempdir().unwrap();
        let store = DirStore::open(dir.path()).unwrap();
        let digest = store.publish(ObjectKind::Manifest, b"manifest").unwrap();
        fs::write(store.path(ObjectKind::Manifest, &digest), b"manifesT").unwrap();
        assert!(matches!(
            store.get(ObjectKind::Manifest, &digest),
            Err(StoreError::Corrupt(_))
        ));
    }
}
