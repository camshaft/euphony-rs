use super::*;

ugen!(
    /// Sum an array of channels.
    ///
    /// Mix will mix an array of channels down to a single channel or an
    /// array of arrays of channels down to a single array of channels.
    #[rates = [ar, kr, xr]]
    #[new(signals: impl Into<ValueVec>)]
    #[compile = compile_mix]
    #[output = Value]
    struct Mix {
        /// The array of channels or arrays.
        signals: ValueVec,
    }
);

fn compile_mix(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let channels = inputs.iter().next().expect("only one param");
    for signals in channels.iter() {
        match signals.len() {
            0 => continue,
            1 => compiler.push_outputs(signals.clone()),
            _ => {
                let rate = spec.rate.unwrap_or(CalculationRate::Audio); // TODO get from the signals
                let first = signals[0];
                let out = signals[1..].iter().fold(first, |acc, signal| {
                    ops::compile_add(acc.input, signal.input, rate, compiler)
                });
                compiler.push_output(out);
            }
        }
    }
}
