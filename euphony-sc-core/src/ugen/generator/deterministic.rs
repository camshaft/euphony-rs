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
