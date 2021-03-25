use core::time::Duration;
use euphony::{
    runtime::{
        graph::{cell::Cell, node::Node, subscription::Readable},
        time::delay,
    },
    time::{beat::Beat, measure::Measure, tempo::Tempo, time_signature::TimeSignature},
};

#[derive(Clone, Debug)]
pub struct Time {
    pub time_signature: Node<Cell<TimeSignature>>,
    pub tempo: Node<Cell<Tempo>>,
}

impl Time {
    pub fn beat_duration(&self, beat: Beat) -> Duration {
        let beat: Beat = (beat / self.time_signature.read().beat()).into();
        let duration = self.tempo.read() * beat;
        duration
    }

    pub fn duration_beats(&self, duration: Duration, resolution: Beat) -> Beat {
        let beats = duration / self.tempo.read();
        let beats = beats.quantize(resolution);
        let beats: Beat = (beats.as_ratio() * self.time_signature.read().beat().as_ratio()).into();
        beats
    }

    pub fn measure_duration(&self, measure: Measure) -> Duration {
        let beats = measure * self.time_signature.read();
        let duration = self.tempo.read() * beats;
        duration
    }

    pub async fn delay_for_beat(&self, beat: Beat) {
        delay::delay_for(self.beat_duration(beat)).await;
    }

    pub async fn delay_for_measure(&self, measure: Measure) {
        delay::delay_for(self.measure_duration(measure)).await;
    }
}
