use super::*;
use euphony::units::ratio::Ratio;

#[derive(Clone, Debug)]
pub enum Math {
    Ascending(Vec<Local>),
    Descending(Vec<Local>),
    Min(Vec<Local>),
    Max(Vec<Local>),
    Add(Vec<Local>),
    Sub(Vec<Local>),
    Mul(Vec<Local>),
    Div(Vec<Local>),
    Mod(Vec<Local>),
}

impl Math {
    pub fn apply(&self, locals: &[Event]) -> Event {
        use Math as T;
        match self {
            T::Ascending(idxs) => Self::is_ordered(idxs, locals, true).into(),
            T::Descending(idxs) => Self::is_ordered(idxs, locals, false).into(),
            T::Min(idxs) => idxs
                .iter()
                .filter_map(|idx| locals[idx.id].as_number())
                .min()
                .map(Event::from)
                .unwrap_or_default(),
            T::Max(idxs) => idxs
                .iter()
                .filter_map(|idx| locals[idx.id].as_number())
                .max()
                .map(Event::from)
                .unwrap_or_default(),
            T::Add(idxs) => Self::bin_op(idxs, locals, |a, b| {
                let a = a.as_number()?;
                let b = b.as_number()?;
                let v = a.checked_add(b)?;
                Some(v.into())
            })
            .unwrap_or_default(),
            T::Sub(idxs) => Self::bin_op(idxs, locals, |a, b| {
                let a = a.as_number()?;
                let b = b.as_number()?;
                let v = a.checked_sub(b)?;
                Some(v.into())
            })
            .unwrap_or_default(),
            T::Mul(idxs) => Self::bin_op(idxs, locals, |a, b| {
                let a = a.as_number()?;
                let b = b.as_number()?;
                let v = a.checked_mul(b)?;
                Some(v.into())
            })
            .unwrap_or_default(),
            T::Div(idxs) => Self::bin_op(idxs, locals, |a, b| {
                let a = a.as_number()?;
                let b = b.as_number()?;
                let v = a.checked_div(b)?;
                Some(v.into())
            })
            .unwrap_or_default(),
            T::Mod(idxs) => Self::bin_op(idxs, locals, |a, b| {
                let a = a.as_number()?;
                let b = b.as_number()?;
                let v = a.checked_rem(b)?;
                Some(v.into())
            })
            .unwrap_or_default(),
        }
    }

    fn bin_op(
        idxs: &[Local],
        locals: &[Event],
        op: impl Fn(&Event, &Event) -> Option<Event>,
    ) -> Option<Event> {
        let mut iter = idxs.iter().map(|v| &locals[v.id]);
        let mut acc = iter.next()?.clone();

        for item in iter {
            acc = op(&acc, item)?;
        }

        Some(acc)
    }

    fn is_ordered(idxs: &[Local], locals: &[Event], ascending: bool) -> bool {
        let mut prev: Ratio<i64> = if ascending { i64::MIN } else { i64::MAX }.into();
        for idx in idxs {
            let mut res = false;

            if let Some(v) = locals[idx.id].as_number() {
                res = if ascending { v >= prev } else { v <= prev };
                prev = v;
            }

            if !res {
                return false;
            }
        }

        true
    }
}
