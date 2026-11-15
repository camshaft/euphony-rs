## Prepared and live playback {#playback}

Modulation renders together with the audio, notes and automation it depends on. Prepared playback
uses results that other computers rendered from the same control graph, and live playback renders
the parts that a live input or a gesture affects on the musician's computer while the rest of the
prepared audio continues. State that modulation carries over time, such as an oscillator's phase,
an envelope follower's history and a loop's delay buffers, is part of each checkpoint. A graph is
admitted by the measured runtime and memory capacity of the computers that render it.

### EU-0012-076 {#EU-0012-076}

The value a modulated control takes at each moment MUST be the same in prepared playback as in live playback of the same revision.

### EU-0012-077 {#EU-0012-077}

The identity of a cached result MUST include every route, tap, selector, conversion and curve version, mode, base, parameter mapping, rate, filter latency, tempo, seed, checkpoint and upstream input it depends on.

### EU-0012-078 {#EU-0012-078}

An edit of an automation lane MUST invalidate its consumers, their tails and the future ranges that depend on their state.

### EU-0012-079 {#EU-0012-079}

An edit of an automation lane MUST NOT invalidate the results of tracks that don't depend on it.

### EU-0012-080 {#EU-0012-080}

A cached result MUST be reused only under the same complete descriptor and a compatible numerical profile.

### EU-0012-081 {#EU-0012-081}

A worker MUST NOT publish a result for an older graph as the current revision.

### EU-0012-082 {#EU-0012-082}

A checkpoint MUST hold the state of every envelope follower and the buffers of every delayed loop.

### EU-0012-083 {#EU-0012-083}

A remote computer that prepares audio MUST compute the same control graph as the local renderer.

### EU-0012-084 {#EU-0012-084}

Prepared playback MUST NOT send control updates for each sample over the network.

### EU-0012-085 {#EU-0012-085}

During a live input or a gesture on a depth, the affected part and its consumers MUST render on the musician's computer while the unaffected prepared audio continues.

### EU-0012-086 {#EU-0012-086}

Where reproducible playback is asked for, the system MUST store each external control performance with its timestamps.

### EU-0012-087 {#EU-0012-087}

A switch between prepared and live playback MUST carry matching oscillator phase, follower history and delay state at an aligned sample.

### EU-0012-088 {#EU-0012-088}

Where matching state is missing at a switch between prepared and live playback, the system MUST render again from a valid checkpoint rather than crossfade two control histories.

### EU-0012-089 {#EU-0012-089}

A seek MUST restore a verified checkpoint and replay the history it needs.

### EU-0012-090 {#EU-0012-090}

A missed live deadline MUST NOT silently lower the rate of modulation.

### EU-0012-091 {#EU-0012-091}

A missed live deadline MUST keep the previous revision playing or mark the preview unavailable, visibly.

### EU-0012-092 {#EU-0012-092}

An accepted edit MUST stay saved when the graph it makes can't run on the available capacity.

### EU-0012-093 {#EU-0012-093}

Where a graph can't run, playback MUST keep the previous revision and say so.

### EU-0012-094 {#EU-0012-094}

The system MUST NOT limit the number of routes or voices by a fixed count.

### EU-0012-095 {#EU-0012-095}

Validating a graph MUST NOT wait on any project other than its own.
