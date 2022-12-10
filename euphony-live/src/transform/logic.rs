use super::*;

#[derive(Clone, Debug)]
pub enum Logic {
    All(Vec<Local>),
    Any(Vec<Local>),
    Not(Vec<Local>),
    Equal(Vec<Local>),
}

impl Logic {
    pub fn apply(&self, locals: &[Event]) -> Event {
        use Logic as T;
        match self {
            T::All(idxs) => idxs.iter().all(|idx| locals[idx.id].is_truthy()).into(),
            T::Any(idxs) => idxs
                .iter()
                .find_map(|idx| {
                    if locals[idx.id].is_truthy() {
                        Some(locals[idx.id].clone())
                    } else {
                        None
                    }
                })
                .unwrap_or_default(),
            T::Not(idxs) => (!idxs.iter().any(|idx| locals[idx.id].is_truthy())).into(),
            T::Equal(idxs) => {
                let mut iter = idxs.iter().map(|idx| &locals[idx.id]);
                let first = if let Some(event) = iter.next() {
                    event
                } else {
                    return Event::undefined();
                };

                for item in iter {
                    if item != first {
                        return false.into();
                    }
                }

                first.clone()
            }
        }
    }
}
