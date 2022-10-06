use crate::{ratio::Ratio, time::beat::Beat};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct UnquantizedBeat(pub u64, pub Beat);

impl UnquantizedBeat {
    pub fn quantize(self, min: Beat) -> Beat {
        let count = self.1.as_ratio() / min.as_ratio();
        let count = count.whole() + u64::from(count.fraction() > Ratio(1, 2));
        Beat(self.0, 1) + min * count
    }
}
