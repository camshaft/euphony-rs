use super::*;

ugen!(
    /// Band Limited ImPulse generator.
    ///
    /// Synth-O-Matic (1990) had an impulse generator called blip, hence
    /// that name here rather than 'buzz'.
    ///
    /// It is improved from other implementations in that it will crossfade
    /// in a control period when the number of harmonics changes, so that
    /// there are no audible pops. It also eliminates the divide in the
    /// formula by using a 1/sin table (with special precautions taken for
    /// 1/0). The lookup tables are linearly interpolated for better quality.
    #[rates = [ar, kr]]
    #[output = Value]
    struct Blip {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Number of harmonics.
        ///
        /// This may be lowered internally if it would cause aliasing.
        #[default = 0.0]
        numharm: ValueVec,
    }
);

ugen!(
    /// Chorusing wavetable lookup oscillator.
    ///
    /// Produces sum of two signals at `(freq ± (beats / 2))`.
    ///
    /// Due to summing, the peak amplitude is not the same as the wavetable and can be twice of that.
    #[rates = [ar, kr]]
    #[new(bufnum: impl Into<ValueVec>)]
    #[meta = UgenMeta::default().reads_buffer()]
    #[output = Value]
    struct COsc {
        /// The number of a buffer filled in wavetable format
        bufnum: ValueVec,

        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Beat frequency in Hertz.
        #[default = 0.0]
        beats: ValueVec,
    }
);

// TODO DynKlang
// TODO DynKlank

ugen!(
    /// Very fast sine wave generator implemented using a ringing filter
    ///
    /// This generates a much cleaner sine wave than a table lookup oscillator and is a
    /// lot faster. However, the amplitude of the wave will vary with frequency.
    /// Generally the amplitude will go down as you raise the frequency and go
    /// up as you lower the frequency.
    #[rates = [ar, kr]]
    #[output = Value]
    struct FSincOsc {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Initial phase offset
        #[default = 0.0]
        phase: ValueVec,
    }
);

ugen!(
    /// Generates a set of harmonics around a formant frequency at a given fundamental frequency.
    ///
    /// The frequency inputs are read at control rate only, so if you use an audio rate UGen as
    /// an input, it will only be sampled at the start of each audio synthesis block.
    #[rates = [ar, kr]]
    #[output = Value]
    struct Formant {
        /// Fundamental frequency in Hertz.
        #[default = 440.0]
        freq: ValueVec,

        /// Formant frequency in Hertz.
        #[default = 1760.0]
        form_freq: ValueVec,

        /// Pulse width frequency in Hertz.
        ///
        /// Controls the bandwidth of the formant.
        /// Must be greater than or equal to freq.
        #[default = 880.0]
        band_freq: ValueVec,
    }
);

ugen!(
    /// Outputs non-bandlimited single sample impulses.
    #[rates = [ar, kr]]
    #[output = Value]
    struct Impulse {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Phase offset in cycles (0..1)
        #[default = 0.0]
        phase: ValueVec,
    }
);

// TODO Klang
// TODO Klank

ugen!(
    /// A sine like shape made of two cubic pieces. Smoother than LFPar.
    #[rates = [ar, kr]]
    #[output = Value]
    struct LFCub {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Initial phase offset
        ///
        /// For efficiency reasons this is a value ranging from 0 to 2.
        #[default = 0.0]
        iphase: ValueVec,
    }
);

ugen!(
    /// A non-band-limited gaussian function oscillator.
    ///
    /// Output ranges from minval to 1.
    ///
    /// LFGauss implements the formula:
    ///
    /// ```text
    /// f(x) = exp(squared(x - iphase) / (-2.0 * squared(width)))
    /// ```
    ///
    /// where x is to vary in the range -1 to 1 over the period dur. minval is the initial value at -1.
    #[rates = [ar, kr]]
    #[output = Value]
    struct LFGauss {
        /// Duration of one cycle
        #[default = 1]
        duration: ValueVec,

        /// Relative width of the bell
        ///
        /// Best to keep below 0.25 when used as envelope
        #[default = 0.1]
        width: ValueVec,

        /// Initial phase offset
        #[default = 0.0]
        iphase: ValueVec,

        /// If true, The UGen oscillates. Otherwise, it calls doneAction after once cycle.
        #[default = 1.0]
        loops: ValueVec,

        /// Evaluated after cycle completes
        #[default = 0]
        done_action: ValueVec,
    }
);

ugen!(
    /// A sine-like shape made of two parabolas and the integral of a triangular wave.
    ///
    /// It has audible odd harmonics and is non-band-limited. Output ranges from -1 to +1.
    #[rates = [ar, kr]]
    #[output = Value]
    struct LFPar {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Initial phase offset
        ///
        /// For efficiency reasons this is a value ranging from 0 to 4.
        #[default = 0.0]
        iphase: ValueVec,
    }
);

ugen!(
    /// A non-band-limited sawtooth oscillator.
    ///
    /// Output ranges from -1 to +1.
    #[rates = [ar, kr]]
    #[output = Value]
    struct LFSaw {
        /// Frequency in Hertz
        ///
        /// For efficiency reasons, only positive values result in correct behavior.
        #[default = 440.0]
        freq: ValueVec,

        /// Initial phase offset
        ///
        /// For efficiency reasons this is a value ranging from 0 to 2.
        #[default = 0.0]
        iphase: ValueVec,
    }
);

ugen!(
    /// A non-band-limited triangle oscillator.
    ///
    /// Output ranges from -1 to +1.
    #[rates = [ar, kr]]
    #[output = Value]
    struct LFTri {
        /// Frequency in Hertz
        ///
        /// For efficiency reasons, only positive values result in correct behavior.
        #[default = 440.0]
        freq: ValueVec,

        /// Initial phase offset
        ///
        /// For efficiency reasons this is a value ranging from 0 to 4.
        #[default = 0.0]
        iphase: ValueVec,
    }
);

ugen!(
    /// Interpolating wavetable oscillator.
    ///
    /// Linear interpolating wavetable lookup oscillator with frequency and
    /// phase modulation inputs.
    ///
    /// This oscillator requires a buffer to be filled with a wavetable format
    /// signal. This preprocesses the Signal into a form which can be used
    /// efficiently by the Oscillator. The buffer size must be a power of 2.
    #[rates = [ar, kr]]
    #[new(bufnum: impl Into<ValueVec>)]
    #[output = Value]
    struct Osc {
        /// Buffer index
        bufnum: ValueVec,

        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Phase in radians
        ///
        /// Sampled at audio-rate
        ///
        /// phase values should be within the range +-8pi. If your phase
        /// values are larger then simply use .mod(2pi) to wrap them.
        #[default = 0.0]
        phase: ValueVec,
    }
);

ugen!(
    /// Non-interpolating wavetable oscillator.
    ///
    /// Noninterpolating wavetable lookup oscillator with frequency and
    /// phase modulation inputs. It is usually better to use the interpolating
    /// oscillator Osc.
    #[rates = [ar, kr]]
    #[new(bufnum: impl Into<ValueVec>)]
    #[output = Value]
    struct OscN {
        /// Buffer index
        bufnum: ValueVec,

        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Phase in radians
        ///
        /// Sampled at audio-rate
        ///
        /// phase values should be within the range +-8pi. If your phase
        /// values are larger then simply use .mod(2pi) to wrap them.
        #[default = 0.0]
        phase: ValueVec,
    }
);

ugen!(
    /// Phase modulation oscillator pair.
    #[rates = [ar, kr]]
    #[new(carfreq: impl Into<ValueVec>, modfreq: impl Into<ValueVec>)]
    #[output = Value]
    struct PMOsc {
        /// Carrier frequency in Hertz
        carfreq: ValueVec,

        /// Modulator frequency in Hertz
        modfreq: ValueVec,

        /// Modulation index in radians
        #[default = 0.0]
        pmindex: ValueVec,

        /// A modulation input for the modulator's phase in radians.
        #[default = 0.0]
        modphase: ValueVec,
    }
);

ugen!(
    /// fixed frequency sine oscillator
    ///
    /// This unit generator uses a very fast algorithm for generating a sine
    /// wave at a fixed frequency.
    #[rates = [ar, kr]]
    #[output = Value]
    struct PSinGrain {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Grain duration
        #[default = 0.2]
        duration: ValueVec,

        /// Amplitude of grain
        #[default = 0.1]
        amp: ValueVec,
    }
);

ugen!(
    /// Band limited pulse wave.
    ///
    /// Band limited pulse wave generator with pulse width modulation.
    #[rates = [ar, kr]]
    #[output = Value]
    struct Pulse {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Pulse width ratio from 0..1
        ///
        /// `0.5` makes a square wave
        #[default = 0.5]
        width: ValueVec,
    }
);

ugen!(
    /// Band limited sawtooth
    ///
    /// Band limited sawtooth wave generator.
    #[rates = [ar, kr]]
    #[output = Value]
    struct Saw {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,
    }
);

ugen!(
    /// Generates a sine wave.
    ///
    /// Uses a wavetable lookup oscillator with linear interpolation.
    ///
    /// Frequency and phase modulation are provided for audio-rate modulation.
    ///
    /// Technically, SinOsc uses the same implementation as Osc except that
    /// its table is fixed to be a sine wave made of 8192 samples.
    #[rates = [ar, kr]]
    #[output = Value]
    struct SinOsc {
        /// Frequency in Hertz
        ///
        /// Sampled at audio-rate
        #[default = 440.0]
        freq: ValueVec,

        /// Phase in radians
        ///
        /// Sampled at audio-rate
        ///
        /// phase values should be within the range +-8pi. If your phase
        /// values are larger then simply use .mod(2pi) to wrap them.
        #[default = 0.0]
        phase: ValueVec,
    }
);

ugen!(
    /// SinOscFB is a sine oscillator that has phase modulation feedback.
    ///
    /// Its output plugs back into the phase input. Basically this allows a
    /// modulation between a sine wave and a sawtooth like wave. Overmodulation
    /// causes chaotic oscillation. It may be useful if you want to simulate
    /// feedback FM synths.
    #[rates = [ar, kr]]
    #[output = Value]
    struct SinOscFB {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Amplitude of the phase feedback in radians
        #[default = 0.0]
        feedback: ValueVec,
    }
);

ugen!(
    /// A sawtooth wave that is hard synched to a fundamental pitch.
    ///
    /// This produces an effect similar to moving formants or pulse width
    /// modulation. The sawtooth oscillator has its phase reset when the
    /// sync oscillator completes a cycle. This is not a band limited waveform,
    /// so it may alias.
    #[rates = [ar, kr]]
    #[output = Value]
    struct SyncSaw {
        /// Frequency of the fundamental in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Frequency of the slave synched sawtooth wave.
        ///
        /// Should always be greater than freq.
        #[default = 440.0]
        saw_freq: ValueVec,
    }
);

ugen!(
    /// A wavetable lookup oscillator which can be swept smoothly across wavetables.
    ///
    /// All the wavetables must be allocated to the same size. Fractional values of
    /// table will interpolate between two adjacent tables.
    ///
    /// This oscillator requires at least two buffers to be filled with a wavetable
    /// format signal. This preprocesses the Signal into a form which can be used
    /// efficiently by the Oscillator. The buffer size must be a power of 2.
    #[rates = [ar, kr]]
    #[new(buf: impl Into<ValueVec>)]
    #[output = Value]
    struct VOsc {
        /// Buffer index
        ///
        /// Can be swept continuously among adjacent wavetable buffers of the same size
        buf: ValueVec,

        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Phase in radians
        #[default = 0.0]
        phase: ValueVec,
    }
);

ugen!(
    /// A wavetable lookup oscillator which can be swept smoothly across wavetables.
    ///
    /// All the wavetables must be allocated to the same size. Fractional values of
    /// table will interpolate between two adjacent tables.
    ///
    /// This unit generator contains three oscillators at different frequencies, mixed together.
    ///
    /// This oscillator requires at least two buffers to be filled with a wavetable
    /// format signal. This preprocesses the Signal into a form which can be used
    /// efficiently by the Oscillator. The buffer size must be a power of 2.
    #[rates = [ar, kr]]
    #[new(buf: impl Into<ValueVec>)]
    #[output = Value]
    struct VOsc3 {
        /// Buffer index
        ///
        /// Can be swept continuously among adjacent wavetable buffers of the same size
        buf: ValueVec,

        /// Frequency in Hertz of the 1st oscillator
        #[default = 440.0]
        freq: ValueVec,

        /// Frequency in Hertz of the 2nd oscillator
        #[default = 220.0]
        freq_2: ValueVec,

        /// Frequency in Hertz of the 3rd oscillator
        #[default = 110.0]
        freq_3: ValueVec,
    }
);

ugen!(
    /// Variable duty saw
    ///
    /// Sawtooth-triangle oscillator with variable duty.
    #[rates = [ar, kr]]
    #[output = Value]
    struct VarSaw {
        /// Frequency in Hertz
        #[default = 440.0]
        freq: ValueVec,

        /// Initial phase offset in radians
        #[default = 0.0]
        iphase: ValueVec,

        /// Duty cycle from zero to one
        #[default = 0.5]
        width: ValueVec,
    }
);

ugen!(
    /// The Vibrato oscillator models a slow frequency modulation.
    ///
    /// Vibrato is a slow frequency modulation. Consider the systematic

    /// deviation in pitch of a singer around a fundamental frequency, or a
    /// violinist whose finger wobbles in position on the fingerboard, slightly
    /// tightening and loosening the string to add shimmer to the pitch. There is
    /// often also a delay before vibrato is established on a note. This UGen models
    /// these processes; by setting more extreme settings, you can get back to the
    /// timbres of FM synthesis. You can also add in some noise to the vibrato rate
    /// and vibrato size (modulation depth) to make for a more realistic motor pattern.
    ///
    /// The vibrato output is a waveform based on a squared envelope shape with four
    /// stages marking out 0.0 to 1.0, 1.0 to 0.0, 0.0 to -1.0, and -1.0 back to 0.0.
    /// Vibrato rate determines how quickly you move through these stages.
    #[rates = [ar, kr]]
    #[output = Value]
    struct Vibrato {
        /// Fundamental  frequency in Hertz
        ///
        /// If the Vibrato UGen is running at audio rate, this must not be a constant,
        /// but an actual audio rate UGen
        #[default = 440.0]
        freq: ValueVec,

        /// Vibrato rate, speed of wobble in Hertz.
        ///
        /// Note that if this is set to a low value (and definitely with 0.0),
        /// you may never get vibrato back, since the rate input is only checked at the
        /// end of a cycle.
        #[default = 6]
        rate: ValueVec,

        /// Size of vibrato frequency deviation around the fundamental, as a proportion of the fundamental.
        ///
        /// For example, 0.02 = 2% of the fundamental.
        #[default = 0.02]
        depth: ValueVec,

        /// Delay before vibrato is established in seconds
        ///
        /// For example, a singer tends to attack a note and then stabilise with vibrato.
        #[default = 0.0]
        delay: ValueVec,

        /// Transition time in seconds from no vibrato to full vibrato after the initial delay time.
        #[default = 0.0]
        onset: ValueVec,

        /// Noise on the rate, expressed as a proportion of the rate
        ///
        /// This can change once per cycle of vibrato.
        #[default = 0.04]
        rate_var: ValueVec,

        /// Noise on the depth of modulation, expressed as a proportion of the depth.
        ///
        /// This can change once per cycle of vibrato. The noise affects independently
        /// the up and the down part of vibrato shape within a cycle.
        #[default = 0.1]
        depth_var: ValueVec,

        /// Initial phase of vibrato modulation
        ///
        /// This allows it to start above or below the fundamental rather than on it.
        #[default = 0.0]
        iphase: ValueVec,

        /// Start again if transition from trig <= 0 to trig > 0.
        #[default = 0.0]
        trig: ValueVec,
    }
);
