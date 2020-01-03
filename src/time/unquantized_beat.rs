use crate::{ratio::Ratio, time::beat::Beat};

#[derive(Clone, Copy, Debug, Default, PartialEq, PartialOrd, Ord, Eq)]
pub struct UnquantizedBeat(pub u64, pub Beat);

impl UnquantizedBeat {
    pub fn quantize(self, min: Beat) -> Beat {
        let count = self.1.as_ratio() / min.as_ratio();
        let count = count.whole() + if count.fraction() > Ratio(1, 2) { 1 } else { 0 };
        Beat(self.0, 1) + min * count
    }
}
