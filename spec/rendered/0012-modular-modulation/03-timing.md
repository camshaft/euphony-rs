## Rate, resolution and timing {#timing}

Resolution of values and timing of updates are separate properties. A control stream carries
values at the resolution of the project's numerical profile, while note and controller events keep
their original integer values until a conversion maps them. Positions are integer numbers of project
samples, and curves in musical time and oscillators synced to tempo resolve through the project's
tempo map. A processor that accepts updates at any sample receives spans of values or events at
sample offsets, so a source at audio rate can drive a compatible filter or oscillator. A processor
that accepts updates once a block declares that interval in its processor profile, and the route's
conversion to it states how it filters, samples and holds or interpolates the source.

### EU-0012-044 {#EU-0012-044}

A control stream MUST carry its values at the resolution of the project's numerical profile.

### EU-0012-045 {#EU-0012-045}

A note or controller value MUST keep its original integer value until its conversion maps it.

### EU-0012-046 {#EU-0012-046}

Each position of a modulation update MUST be an integer number of project samples.

### EU-0012-047 {#EU-0012-047}

A curve in musical time or an oscillator synced to tempo MUST resolve its positions through the project's tempo map.

### EU-0012-048 {#EU-0012-048}

The project MUST store the seed, phase and reset policy of each oscillator.

### EU-0012-167 {#EU-0012-167}

The project MUST store the state of each envelope.

### EU-0012-168 {#EU-0012-168}

The project MUST store the state of each control's smoothing.

### EU-0012-049 {#EU-0012-049}

A processor that accepts updates at any sample MUST apply each update at the sample it is given.

### EU-0012-050 {#EU-0012-050}

A processor that accepts updates only once a block MUST declare the interval at which it accepts them.

### EU-0012-051 {#EU-0012-051}

The interface MUST show the update interval of a target that accepts updates once a block, with its duration at the project's sample rate.

### EU-0012-052 {#EU-0012-052}

A route to a target that accepts updates once a block MUST state how its conversion filters, samples and holds or interpolates the source.

### EU-0012-053 {#EU-0012-053}

A route that asks for sample accuracy to a target that accepts updates once a block MUST be refused unless the musician explicitly selects a conversion.

### EU-0012-054 {#EU-0012-054}

Reducing the rate of a control MUST filter it against aliasing and record the latency the filter adds.

### EU-0012-055 {#EU-0012-055}

A discrete control MUST apply a change at its declared safe boundary.

### EU-0012-056 {#EU-0012-056}

A change of the device's buffer size MUST NOT change what a stored route does.
