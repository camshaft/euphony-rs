use crate::{
    fun::{self, an, AudioNode},
    prelude::*,
};

#[derive(Node)]
#[node(id = 300, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
/// Butterworth lowpass filter (2nd order).
pub struct Butterpass {
    inner: fun::ButterLowpass<Sample, Sample, fun::U2>,
}

impl Default for Butterpass {
    fn default() -> Self {
        Self {
            inner: an(fun::butterpass()),
        }
    }
}

impl Butterpass {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, output: &mut [Sample]) {
        for (signal, cutoff, output) in (signal, cutoff, output.iter_mut()).zip() {
            let input = [signal, cutoff];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 301, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
/// One-pole lowpass filter (1st order).
pub struct Lowpole {
    inner: fun::Lowpole<Sample, Sample, fun::U2>,
}

impl Default for Lowpole {
    fn default() -> Self {
        Self {
            inner: an(fun::lowpole()),
        }
    }
}

impl Lowpole {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, output: &mut [Sample]) {
        for (signal, cutoff, output) in (signal, cutoff, output.iter_mut()).zip() {
            let input = [signal, cutoff];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 302, module = "filter")]
#[input(signal, default = 0.0)]
#[input(delay, default = 1.0)]
/// Allpass filter with adjustable delay (delay > 0) in samples at DC.
pub struct Allpole {
    inner: fun::Allpole<Sample, Sample, fun::U2>,
}

impl Default for Allpole {
    fn default() -> Self {
        Self {
            inner: an(fun::allpole()),
        }
    }
}

impl Allpole {
    #[inline]
    pub fn render(&mut self, signal: Input, delay: Input, output: &mut [Sample]) {
        for (signal, delay, output) in (signal, delay, output.iter_mut()).zip() {
            let input = [signal, delay];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 303, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
/// One-pole, one-zero highpass filter (1st order).
pub struct Highpole {
    inner: fun::Highpole<Sample, Sample, fun::U2>,
}

impl Default for Highpole {
    fn default() -> Self {
        Self {
            inner: an(fun::highpole()),
        }
    }
}

impl Highpole {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, output: &mut [Sample]) {
        for (signal, cutoff, output) in (signal, cutoff, output.iter_mut()).zip() {
            let input = [signal, cutoff];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 304, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
#[input(bandwidth, default = 110.0)]
/// Constant-gain bandpass resonator.
pub struct Resonator {
    inner: fun::Resonator<Sample, Sample, fun::U3>,
}

impl Default for Resonator {
    fn default() -> Self {
        Self {
            inner: an(fun::resonator()),
        }
    }
}

impl Resonator {
    #[inline]
    pub fn render(
        &mut self,
        signal: Input,
        cutoff: Input,
        bandwidth: Input,
        output: &mut [Sample],
    ) {
        for (signal, cutoff, bandwidth, output) in
            (signal, cutoff, bandwidth, output.iter_mut()).zip()
        {
            let input = [signal, cutoff, bandwidth];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 305, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 1000.0)]
#[input(q, default = 0.1)]
/// Moog resonant lowpass filter.
pub struct Moog {
    inner: fun::Moog<Sample, Sample, fun::U3>,
}

impl Default for Moog {
    fn default() -> Self {
        Self {
            inner: an(fun::moog()),
        }
    }
}

impl Moog {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, q: Input, output: &mut [Sample]) {
        for (signal, cutoff, q, output) in (signal, cutoff, q, output.iter_mut()).zip() {
            let input = [signal, cutoff, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 306, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
#[input(morph, default = 0.0)]
/// Morphing filter that morphs between lowpass, peak and highpass modes.
pub struct Morph {
    inner: fundsp::prelude::Morph<Sample, Sample>,
}

impl Default for Morph {
    fn default() -> Self {
        Self {
            inner: an(fun::morph()),
        }
    }
}

impl Morph {
    #[inline]
    pub fn render(
        &mut self,
        signal: Input,
        cutoff: Input,
        q: Input,
        morph: Input,
        output: &mut [Sample],
    ) {
        for (signal, cutoff, q, morph, output) in
            (signal, cutoff, q, morph, output.iter_mut()).zip()
        {
            let input = [signal, cutoff, q, morph];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 307, module = "filter")]
#[input(signal, default = 0.0)]
/// Pinking filter.
pub struct Pinkpass {
    inner: fundsp::prelude::Pinkpass<Sample, Sample>,
}

impl Default for Pinkpass {
    fn default() -> Self {
        Self {
            inner: an(fun::pinkpass()),
        }
    }
}

impl Pinkpass {
    #[inline]
    pub fn render(&mut self, signal: Input, output: &mut [Sample]) {
        for (signal, output) in (signal, output.iter_mut()).zip() {
            let input = [signal];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 308, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
#[input(q, default = 0.1)]
/// Lowpass filter.
pub struct Lowpass {
    inner: fun::Svf<Sample, Sample, fun::LowpassMode<Sample>>,
}

impl Default for Lowpass {
    fn default() -> Self {
        Self {
            inner: an(fun::lowpass()),
        }
    }
}

impl Lowpass {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, q: Input, output: &mut [Sample]) {
        for (signal, cutoff, q, output) in (signal, cutoff, q, output.iter_mut()).zip() {
            let input = [signal, cutoff, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 309, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
#[input(q, default = 0.1)]
/// Highpass filter.
pub struct Highpass {
    inner: fun::Svf<Sample, Sample, fun::HighpassMode<Sample>>,
}

impl Default for Highpass {
    fn default() -> Self {
        Self {
            inner: an(fun::highpass()),
        }
    }
}

impl Highpass {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, q: Input, output: &mut [Sample]) {
        for (signal, cutoff, q, output) in (signal, cutoff, q, output.iter_mut()).zip() {
            let input = [signal, cutoff, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 310, module = "filter")]
#[input(signal, default = 0.0)]
#[input(cutoff, default = 440.0)]
#[input(q, default = 0.1)]
/// Bandpass filter.
pub struct Bandpass {
    inner: fun::Svf<Sample, Sample, fun::BandpassMode<Sample>>,
}

impl Default for Bandpass {
    fn default() -> Self {
        Self {
            inner: an(fun::bandpass()),
        }
    }
}

impl Bandpass {
    #[inline]
    pub fn render(&mut self, signal: Input, cutoff: Input, q: Input, output: &mut [Sample]) {
        for (signal, cutoff, q, output) in (signal, cutoff, q, output.iter_mut()).zip() {
            let input = [signal, cutoff, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 311, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
/// Notch filter.
pub struct Notch {
    inner: fun::Svf<Sample, Sample, fun::NotchMode<Sample>>,
}

impl Default for Notch {
    fn default() -> Self {
        Self {
            inner: an(fun::notch()),
        }
    }
}

impl Notch {
    #[inline]
    pub fn render(&mut self, signal: Input, center: Input, q: Input, output: &mut [Sample]) {
        for (signal, center, q, output) in (signal, center, q, output.iter_mut()).zip() {
            let input = [signal, center, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 312, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
/// Peak filter.
pub struct Peak {
    inner: fun::Svf<Sample, Sample, fun::PeakMode<Sample>>,
}

impl Default for Peak {
    fn default() -> Self {
        Self {
            inner: an(fun::peak()),
        }
    }
}

impl Peak {
    #[inline]
    pub fn render(&mut self, signal: Input, center: Input, q: Input, output: &mut [Sample]) {
        for (signal, center, q, output) in (signal, center, q, output.iter_mut()).zip() {
            let input = [signal, center, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 313, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
/// Allpass filter.
pub struct Allpass {
    inner: fun::Svf<Sample, Sample, fun::AllpassMode<Sample>>,
}

impl Default for Allpass {
    fn default() -> Self {
        Self {
            inner: an(fun::allpass()),
        }
    }
}

impl Allpass {
    #[inline]
    pub fn render(&mut self, signal: Input, center: Input, q: Input, output: &mut [Sample]) {
        for (signal, center, q, output) in (signal, center, q, output.iter_mut()).zip() {
            let input = [signal, center, q];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 314, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
#[input(gain, default = 1.0)]
/// Bell filter.
pub struct Bell {
    inner: fun::Svf<Sample, Sample, fun::BellMode<Sample>>,
}

impl Default for Bell {
    fn default() -> Self {
        Self {
            inner: an(fun::bell()),
        }
    }
}

impl Bell {
    #[inline]
    pub fn render(
        &mut self,
        signal: Input,
        center: Input,
        q: Input,
        gain: Input,
        output: &mut [Sample],
    ) {
        for (signal, center, q, gain, output) in (signal, center, q, gain, output.iter_mut()).zip()
        {
            let input = [signal, center, q, gain];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 315, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
#[input(gain, default = 1.0)]
/// Lowshelf filter.
pub struct Lowshelf {
    inner: fun::Svf<Sample, Sample, fun::LowshelfMode<Sample>>,
}

impl Default for Lowshelf {
    fn default() -> Self {
        Self {
            inner: an(fun::lowshelf()),
        }
    }
}

impl Lowshelf {
    #[inline]
    pub fn render(
        &mut self,
        signal: Input,
        center: Input,
        q: Input,
        gain: Input,
        output: &mut [Sample],
    ) {
        for (signal, center, q, gain, output) in (signal, center, q, gain, output.iter_mut()).zip()
        {
            let input = [signal, center, q, gain];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

#[derive(Node)]
#[node(id = 316, module = "filter")]
#[input(signal, default = 0.0)]
#[input(center, default = 440.0)]
#[input(q, default = 0.1)]
#[input(gain, default = 1.0)]
/// Highshelf filter.
pub struct Highshelf {
    inner: fun::Svf<Sample, Sample, fun::HighshelfMode<Sample>>,
}

impl Default for Highshelf {
    fn default() -> Self {
        Self {
            inner: an(fun::highshelf()),
        }
    }
}

impl Highshelf {
    #[inline]
    pub fn render(
        &mut self,
        signal: Input,
        center: Input,
        q: Input,
        gain: Input,
        output: &mut [Sample],
    ) {
        for (signal, center, q, gain, output) in (signal, center, q, gain, output.iter_mut()).zip()
        {
            let input = [signal, center, q, gain];
            *output = self.inner.tick(&input.into())[0];
        }
    }
}

// TODO FIR filter
