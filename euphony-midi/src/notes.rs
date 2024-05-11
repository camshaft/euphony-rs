use core::{fmt, ops::Deref};

const LEN: usize = 128;

#[derive(Clone)]
pub struct Notes {
    state: [u8; LEN],
    len: u8,
}

impl fmt::Debug for Notes {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_map().entries(self.entries()).finish()
    }
}

impl Default for Notes {
    #[inline]
    fn default() -> Self {
        Self {
            state: [0; LEN],
            len: 0,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Note(u8);

impl fmt::Debug for Note {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl Note {
    #[inline]
    pub fn new(value: u8) -> Option<Self> {
        if value < 128 {
            Some(Self(value))
        } else {
            None
        }
    }

    #[inline]
    pub fn transpose(&self, amount: i8) -> Option<Self> {
        if amount > 0 {
            Self::new(self.0 + amount as u8)
        } else {
            let v = self.0.checked_sub((-amount) as u8)?;
            Some(Self(v))
        }
    }
}

impl Deref for Note {
    type Target = u8;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Event {
    On { note: Note },
    Off { note: Note },
}

impl Notes {
    #[inline]
    pub fn insert(&mut self, event: Event) -> Option<Event> {
        match event {
            Event::On { note } => self.on(note).map(|note| Event::On { note }),
            Event::Off { note } => self.off(note).map(|note| Event::Off { note }),
        }
    }

    #[inline]
    pub fn on(&mut self, note: Note) -> Option<Note> {
        let count = self.get(note);
        self.set(note, count.saturating_add(1));

        if count == 0 {
            self.len += 1;
            Some(note)
        } else {
            None
        }
    }

    #[inline]
    pub fn off(&mut self, note: Note) -> Option<Note> {
        let count = self.get(note);
        self.set(note, count.saturating_sub(1));

        if count == 1 {
            self.len -= 1;
            Some(note)
        } else {
            None
        }
    }

    #[inline]
    pub fn map<F: FnMut(Note) -> Option<Note>>(&self, mut f: F) -> Self {
        self.entries()
            .filter_map(|(note, count)| {
                let note = f(note)?;
                Some((note, count))
            })
            .collect()
    }

    #[inline]
    pub fn transpose(&self, amount: i8) -> Self {
        self.map(|note| note.transpose(amount))
    }

    #[inline]
    pub fn diff<'a>(&'a self, next: &'a Self) -> impl Iterator<Item = Event> + 'a {
        let len = self.len.max(next.len) as usize;
        self.state
            .iter()
            .zip(&next.state)
            .enumerate()
            .take(len)
            .filter_map(|(note, (&prev_count, &next_count))| {
                match (prev_count > 0, next_count > 0) {
                    (true, true) | (false, false) => None,
                    (false, true) => Some(Event::On {
                        note: Note(note as _),
                    }),
                    (true, false) => Some(Event::Off {
                        note: Note(note as _),
                    }),
                }
            })
    }

    #[inline]
    pub fn active(&self) -> impl Iterator<Item = Note> + '_ {
        self.entries().map(|(note, _)| note)
    }

    #[inline]
    pub fn entries(&self) -> Iter {
        self.clone().into_iter()
    }

    #[inline]
    pub fn len(&self) -> u8 {
        self.len
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    #[inline]
    pub fn get(&self, note: Note) -> u8 {
        self.state[(note.0 & 0x7f) as usize]
    }

    #[inline]
    pub fn set(&mut self, note: Note, count: u8) -> u8 {
        let idx = (note.0 & 0x7f) as usize;
        let v = &mut self.state[idx];
        core::mem::replace(v, count)
    }

    #[inline]
    pub fn clear(&mut self) -> u8 {
        let len = self.len();
        *self = Self::default();
        len
    }
}

impl IntoIterator for Notes {
    type Item = (Note, u8);
    type IntoIter = Iter;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            idx: 0,
            state: self.state,
            len: self.len as _,
        }
    }
}

pub struct Iter {
    idx: usize,
    state: [u8; LEN],
    len: usize,
}

impl Iterator for Iter {
    type Item = (Note, u8);

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        while self.len > 0 {
            let idx = self.idx;
            let count = *self.state.get(idx)?;
            self.idx += 1;

            if count == 0 {
                continue;
            }

            self.len -= 1;

            return Some((Note(idx as _), count));
        }

        None
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl FromIterator<(Note, u8)> for Notes {
    #[inline]
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = (Note, u8)>,
    {
        let mut notes = Self::default();
        notes.extend(iter);
        notes
    }
}

impl Extend<(Note, u8)> for Notes {
    #[inline]
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (Note, u8)>,
    {
        for (note, count) in iter {
            let prev = self.get(note);
            self.set(note, prev.saturating_add(count));
            if prev == 0 && count > 0 {
                self.len += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        use Event::*;

        let mut notes = Notes::default();

        let events = [
            On { note: Note(0) },
            On { note: Note(0) },
            Off { note: Note(0) },
        ];

        for event in events {
            notes.insert(event);
            dbg!(&notes);
            dbg!(notes.transpose(-1));
            dbg!(notes.transpose(1));
        }
    }
}
