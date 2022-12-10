use euphony::prelude::{Beat, Tempo};

use super::*;

#[derive(Clone, Debug)]
pub enum Theory {
    FromMidi,
    ToMidi,
    Collapse {
        mode: Local,
        // TODO configure strategy
    },
    Expand {
        mode: Local,
        // TODO configure strategy
    },
    BeatDuration {
        tempo: Local,
    },
}

impl Theory {
    pub fn apply(&self, event: &Event, locals: &[Event]) -> Option<Event> {
        match self {
            Self::FromMidi => {
                let n = event.as_number()?;
                Some(((n - 69) / 12).into())
            }
            Self::ToMidi => {
                let i = event.as_number()?;
                let i = i * 12 + 69;
                Some(i.into())
            }
            Self::Collapse { mode } => {
                let i = event.as_number()?;
                let m = locals[mode.id].as_mode()?;
                let i = m.checked_collapse(i.into(), Default::default())?;
                Some(i.into())
            }
            Self::Expand { mode } => {
                let i = event.as_number()?;
                let m = locals[mode.id].as_mode()?;
                let i = m.checked_expand(i.into(), Default::default())?;
                Some(i.into())
            }
            Self::BeatDuration { tempo } => {
                let beats = event.as_number()?;
                let beats = Beat::from(beats);
                let tempo = locals[tempo.id].as_number()?;
                let tempo = Tempo::from(tempo);

                let duration = tempo * beats;

                Some(duration.into())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use builder::*;
    use euphony::prelude::{western::*, *};
    use insta::assert_debug_snapshot;

    #[test]
    fn collapse_expand() {
        let mut out = 0;

        let config = build(|| {
            let key = local();
            let source_mode = local();
            let target_mode = local();

            let collapsed = key.theory(Theory::Collapse { mode: source_mode });
            let expand = collapsed.theory(Theory::Expand { mode: target_mode });

            out = expand.id;
        });

        let mut tests = vec![];

        for interval in 0..12 {
            tests.push((Interval(interval, 12).reduce(), MAJOR, MINOR));
        }

        let mut state = config.init_offline();

        let mut outputs = vec![];

        for (key, source_mode, target_mode) in tests {
            state[0] = key.into();
            state[1] = source_mode.into();
            state[2] = target_mode.into();

            state.apply(&[]);

            outputs.push((state[0].clone(), state[out].clone()));
        }

        assert_debug_snapshot!(outputs);
    }
}
