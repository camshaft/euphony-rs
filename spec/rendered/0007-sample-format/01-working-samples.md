## Working sample format {#working-samples}

Working audio is the audio euphony processes, prepares, saves and shares: the samples at each
processor interface, each prepared result in the render cache, the audio objects of a project and
the audio that peers sync. Each of its samples is an IEEE 754 binary32 value. That format holds
every signed 24-bit integer sample exactly, keeps a value above full scale until a later stage
attenuates it, and doesn't round to an integer at each processing step. Floating-point amplitude is
about sample values alone: sample positions, musical time, note identities, counters and protocol
timestamps keep their exact forms. Lossy copies of audio made for listening, such as compressed
previews for collaborators, are outside this RFC. Audio that was clipped, rounded or reduced in
detail before it reached euphony stays as it arrived, and audio devices and network audio
equipment keep the sample formats they expect.

### EU-0007-001 {#EU-0007-001}

Working audio MUST carry each sample amplitude as an IEEE 754 binary32 value.

### EU-0007-002 {#EU-0007-002}

The floating-point amplitude format MUST NOT change how sample positions, musical time, note identities, counters or protocol timestamps are represented.

### EU-0007-003 {#EU-0007-003}

A processor MUST return binary32 samples at its declared interface, whatever precision it uses inside.

### EU-0007-004 {#EU-0007-004}

A sample above full scale MUST pass from one processor to the next unchanged.

### EU-0007-005 {#EU-0007-005}

A render MUST NOT be published while it holds a non-finite sample.

### EU-0007-006 {#EU-0007-006}

A live processor result that holds a non-finite sample MUST mute the outputs it feeds.

### EU-0007-007 {#EU-0007-007}

A live processor result that holds a non-finite sample MUST be reported as a failure that names the processor.
