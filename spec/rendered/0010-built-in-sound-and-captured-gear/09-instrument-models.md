## Instrument models {#instrument-models}

An instrument model generates audio from timed events, control trajectories, elapsed time and prior
state. A structured instrument model has three parts: euphony's own voice allocator and envelopes,
which hold the voices and their state; a model fitted for each voice, conditioned on the known
settings; and a shared stage after the voices, modeled as an external chain is. A learned
instrument model is one trained generator of the whole instrument. A voice state holds oscillator
phase, envelope stage, filter memory, note identity, pedal state, glide and random-process state;
the instrument state holds voice allocation, global modulation, shared effects and channel and
program state. The manifest of a model states the continuous control regions and the discrete
modes it supports, and the interface shows each setting as measured, validated by interpolation or
unsupported. The model represents the tested unit, firmware, patch families, controls and
operating conditions, not every sound any unit of the product can make.

### EU-0010-168 {#EU-0010-168}

An instrument model MUST generate audio from timed events, control trajectories, elapsed time and prior state.

### EU-0010-169 {#EU-0010-169}

For each synthesizer capture, euphony MUST fit a structured instrument model and a learned instrument model to the same capture.

### EU-0010-170 {#EU-0010-170}

A structured instrument model MUST hold its voices and their state in euphony's own voice allocator and envelopes.

### EU-0010-171 {#EU-0010-171}

The pitch of each voice of a structured instrument model MUST come from the events it plays.

### EU-0010-172 {#EU-0010-172}

A voice state MUST hold oscillator phase, envelope stage, filter memory, note identity, pedal state, glide and random-process state.

### EU-0010-173 {#EU-0010-173}

The instrument state MUST hold voice allocation, global modulation, shared effects and channel and program state.

### EU-0010-174 {#EU-0010-174}

An instrument model MUST hold the polyphony and voice allocation behavior measured on the device.

### EU-0010-175 {#EU-0010-175}

An instrument model MUST NOT steal voices beyond its modeled behavior to hide an overload.

### EU-0010-176 {#EU-0010-176}

Where the runtime can't sustain the modeled voice behavior, euphony MUST mark live use of the model unavailable.

### EU-0010-177 {#EU-0010-177}

An instrument model MUST derive the random stream of each voice from a root seed and stable event and voice identities.

### EU-0010-178 {#EU-0010-178}

A seek or restart of an instrument model MUST restore its voice and random state from a compatible checkpoint, or render the history that leads to it.

### EU-0010-179 {#EU-0010-179}

An instrument model MUST interpolate a control only between validated points within its supported domain.

### EU-0010-180 {#EU-0010-180}

A request for a control value outside the supported domain of an instrument model MUST be refused with that domain and its missing coverage.

### EU-0010-181 {#EU-0010-181}

An instrument model MUST keep patch identities, algorithm choices and voice modes discrete.

### EU-0010-182 {#EU-0010-182}

An experimental preview outside the supported domain of an instrument model MUST NOT acquire a fidelity claim.

### EU-0010-183 {#EU-0010-183}

euphony MUST admit an instrument model for live playback only where its deadline, memory and underrun tests pass at its advertised polyphony and control rates on that computer.

### EU-0010-184 {#EU-0010-184}

Attaching an instrument model to a track MUST name its model digest, runtime profile version, parameter map, program, initial state and seed.

### EU-0010-185 {#EU-0010-185}

A worker MUST NOT approve a model or attach it to a track.

### EU-0010-186 {#EU-0010-186}

The report of an instrument model MUST give its results by pitch, velocity, articulation, duration and release, and by control region, chord, pedal sequence and automation path.

### EU-0010-187 {#EU-0010-187}

A null test of an instrument model MUST be applied only where reset, phase, timing and deterministic state can be matched.

### EU-0010-188 {#EU-0010-188}

The final listening test of an instrument model MUST compare independently captured hardware performances with model renders of identical event and control sequences.

### EU-0010-189 {#EU-0010-189}

The report of an instrument model MUST state note-level clips, phrase-level clips and live playing as separate claims.

### EU-0010-190 {#EU-0010-190}

The acceptance of an instrument model MUST keep its tested coverage and limitations with the model.

### EU-0010-191 {#EU-0010-191}

euphony MUST NOT call an instrument model complete while a promised parameter domain or voice behavior remains untested.
