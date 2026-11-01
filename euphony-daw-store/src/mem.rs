use crate::{ObjectKind, Store, StoreError};
use euphony_daw_identity::Digest;
use std::{collections::HashMap, sync::Mutex};

/// Objects held in memory, for simulations and tests.
#[derive(Debug, Default)]
pub struct MemStore {
    objects: Mutex<HashMap<(ObjectKind, Digest), Vec<u8>>>,
}

impl MemStore {
    pub fn new() -> Self {
        Self::default()
    }

    /// The number of objects held.
    pub fn len(&self) -> usize {
        self.objects.lock().unwrap().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Each object held, with its kind and digest.
    pub fn objects(&self) -> Vec<(ObjectKind, Digest, Vec<u8>)> {
        let objects = self.objects.lock().unwrap();
        let mut out = Vec::with_capacity(objects.len());
        for ((kind, digest), bytes) in objects.iter() {
            out.push((*kind, *digest, bytes.clone()));
        }
        out.sort_by_key(|(kind, digest, _)| (*kind, *digest));
        out
    }

    /// Flips one bit of a held object, as a failing disk would.
    pub fn damage(&self, kind: ObjectKind, digest: &Digest) {
        let mut objects = self.objects.lock().unwrap();
        if let Some(bytes) = objects.get_mut(&(kind, *digest)) {
            if let Some(byte) = bytes.last_mut() {
                *byte ^= 1;
            }
        }
    }
}

impl Store for MemStore {
    fn publish(&self, kind: ObjectKind, bytes: &[u8]) -> Result<Digest, StoreError> {
        let digest = kind.digest(bytes);
        let mut objects = self.objects.lock().unwrap();
        let entry = objects.entry((kind, digest));
        entry.or_insert_with(|| bytes.to_vec());
        Ok(digest)
    }

    fn get(&self, kind: ObjectKind, digest: &Digest) -> Result<Vec<u8>, StoreError> {
        let objects = self.objects.lock().unwrap();
        let bytes = objects.get(&(kind, *digest));
        let bytes = bytes.ok_or(StoreError::Absent(*digest))?;
        if kind.digest(bytes) != *digest {
            return Err(StoreError::Corrupt(*digest));
        }
        Ok(bytes.clone())
    }

    fn contains(&self, kind: ObjectKind, digest: &Digest) -> bool {
        let objects = self.objects.lock().unwrap();
        objects.contains_key(&(kind, *digest))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishes_and_verifies() {
        let store = MemStore::new();
        let digest = store.publish(ObjectKind::Manifest, b"manifest").unwrap();
        let bytes = store.get(ObjectKind::Manifest, &digest).unwrap();
        assert_eq!(bytes, b"manifest");
        let absent = store.get(ObjectKind::WorkingAudio, &digest);
        assert!(matches!(absent, Err(StoreError::Absent(_))));
        store.damage(ObjectKind::Manifest, &digest);
        let corrupt = store.get(ObjectKind::Manifest, &digest);
        assert!(matches!(corrupt, Err(StoreError::Corrupt(_))));
    }

    #[test]
    fn refuses_bytes_with_another_digest() {
        let store = MemStore::new();
        let expected = ObjectKind::Manifest.digest(b"one");
        let result = store.publish_expected(ObjectKind::Manifest, b"two", &expected);
        assert!(matches!(result, Err(StoreError::Mismatch { .. })));
        assert!(store.is_empty());
    }
}
