## Feedback and delay {#feedback}

The cycle check reads the combined graph of audio, notes and controls, including reads of effective
values and the state of conversions. A processor that doesn't declare which of its inputs reach its
outputs without delay counts as passing every input to every output at once. Only a dependency that
an explicit delay of D project samples breaks is removed, where D is a positive integer, and the rest
of the graph is required to be acyclic. A loop renders as one unit: at each step it reads its delayed
values, evaluates the graph within the loop, then commits the new delay state, in steps no longer
than its smallest delay, so a delay of one sample is one sample rather than one device block.
Converter and processor latencies are further declared delays. A loop can be causally valid and
still unstable, so the system warns of growing gain, keeps every target within its domain and
protects the output.

### EU-0012-057 {#EU-0012-057}

A cycle in the graph of audio, notes and controls MUST be refused unless a delay the musician sets breaks it.

### EU-0012-058 {#EU-0012-058}

The refusal of a cycle MUST name the path of the cycle.

### EU-0012-059 {#EU-0012-059}

A feedback delay MUST be a positive integer number of project samples.

### EU-0012-060 {#EU-0012-060}

The cycle check MUST count reads of effective values and the state of conversions as dependencies.

### EU-0012-061 {#EU-0012-061}

The cycle check MUST treat a processor that doesn't declare its feedthrough as passing every input to every output without delay.

### EU-0012-062 {#EU-0012-062}

A feedback loop MUST return a signal after exactly its declared total delay, whatever the device's block size.

### EU-0012-063 {#EU-0012-063}

Each step of a feedback loop MUST read the delayed values, then evaluate the loop, then commit the new delay state.

### EU-0012-064 {#EU-0012-064}

A converter's or processor's latency MUST NOT stand in for a loop's explicit feedback delay.

### EU-0012-065 {#EU-0012-065}

A change of a feedback delay MUST make a new revision of the graph.

### EU-0012-066 {#EU-0012-066}

The buffers of a feedback delay MUST start at zero unless a saved checkpoint supplies them.

### EU-0012-067 {#EU-0012-067}

A loop MUST keep or reset its state at a transport change as its stored policy says, the same in offline and live playback.

### EU-0012-068 {#EU-0012-068}

A change of the project's sample rate MUST restate each feedback delay in samples.

### EU-0012-069 {#EU-0012-069}

The interface MUST warn of a cycle whose gain or range can grow.

### EU-0012-070 {#EU-0012-070}

The output MUST stay within its authorized safe gain.

### EU-0012-071 {#EU-0012-071}

A musician MUST be able to bypass any route at once.

### EU-0012-072 {#EU-0012-072}

An output that isn't finite MUST fault the rendering of the part of the project that produced it and protect the output.

### EU-0012-073 {#EU-0012-073}

An output that isn't finite MUST NOT become cached audio.

### EU-0012-074 {#EU-0012-074}

Any limiting of the output MUST be an explicit processor of the project.

### EU-0012-075 {#EU-0012-075}

A live feedback loop MUST render on one computer.
