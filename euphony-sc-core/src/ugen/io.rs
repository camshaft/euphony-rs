use super::*;

ugen!(
    /// Write a signal to a bus.
    ///
    /// When using a Bus with an In or Out UGen there is nothing to stop you
    /// from reading to or writing from a larger range, or from
    /// hardcoding to a bus that has been allocated. You are responsible
    /// for making sure that the number of channels match and that there
    /// are no conflicts.
    #[rates = [ar, kr]]
    #[new(bus: impl Into<Value>, channels: impl Into<ValueVec>)]
    #[compile = compile_out]
    #[meta = UgenMeta::default().writes_bus()]
    #[output = ()]
    #[output_len = 0]
    struct Out {
        /// The index of the bus to write out to.
        ///
        /// The lowest numbers are written to the audio hardware.
        bus: Value,

        /// An Array of channels or single output to write out.
        ///
        /// You cannot change the size of this once a SynthDef has been built.
        channels: ValueVec,
    }
);

fn compile_out(spec: &UgenSpec, inputs: &Inputs, compiler: &mut Compiler) {
    let mut inputs = inputs.iter();
    let bus = inputs.next().expect("bus");
    let channels = inputs.next().expect("channels");

    let mut ins = vec![];
    let mut rate = spec.rate.unwrap_or(CalculationRate::Scalar);
    ins.push(bus[0][0].input);
    rate = rate.max(bus[0][0].rate);

    for channel in channels.iter().flatten() {
        ins.push(channel.input);
        rate = rate.max(channel.rate);
    }

    let ugen = UGen {
        name: spec.name.into(),
        special_index: spec.special_index,
        rate,
        ins,
        outs: vec![],
    };

    let outputs = compiler.push_ugen(ugen, spec.meta);
    compiler.push_outputs(outputs);
}
