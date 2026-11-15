## Dependencies and invalidation {#invalidation}

The project holds rational musical time, explicit tempo and meter maps, sample-positioned audio,
stable identities of tracks and regions, bus routing, automation curves, processor definitions and
references to immutable assets. Its graph is compiled into render islands. A render island is a
subgraph that renders from explicit input streams and an explicit start state. Independent tracks
and islands render in parallel, while sends, sidechains and buses wait on their upstream. A feedback
cycle is collapsed into one island with an explicit delay and state transition, since moving a
stateful loop between computers while it renders changes how it sounds. A checkpoint holds the
running state of each processor it covers, such as filter memory and the contents of a delay line,
which a processor's saved parameters don't hold. Edits create a new revision. Tempo, meter,
automation interpolation, loop boundaries, latency compensation, tails and processor state are
dependencies like any other input.

### EU-0006-026 {#EU-0006-026}

Musical positions MUST be held exactly until they are resolved to sample positions.

### EU-0006-027 {#EU-0006-027}

Musical positions MUST be resolved to sample positions by one stated rounding rule.

### EU-0006-028 {#EU-0006-028}

Each feedback cycle MUST be rendered inside one render island with an explicit delay.

### EU-0006-029 {#EU-0006-029}

A feedback cycle without a delay MUST NOT be rendered.

### EU-0006-030 {#EU-0006-030}

A feedback cycle MUST NOT be divided between workers.

### EU-0006-031 {#EU-0006-031}

A time range MUST be rendered on its own only where its start state and its complete input history are known.

### EU-0006-032 {#EU-0006-032}

A stateful processor MUST start each range from its initial state or from a verified checkpoint.

### EU-0006-033 {#EU-0006-033}

A checkpoint MUST hold the running state of each processor it covers.

### EU-0006-034 {#EU-0006-034}

Independent render islands and independent time ranges MUST be able to render on different workers at the same time.

### EU-0006-035 {#EU-0006-035}

An edit MUST create a new revision without changing any result of an earlier revision.

### EU-0006-036 {#EU-0006-036}

An edit MUST invalidate every result that depends on its changed input through any dependency of the graph.

### EU-0006-037 {#EU-0006-037}

A change of tempo or meter MUST invalidate every result whose timing it changes, whether or not its notes changed.

### EU-0006-038 {#EU-0006-038}

Where a tail is unbounded or unknown, the remainder that depends on it MUST stay invalid until it is rendered again.
