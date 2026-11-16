## Render descriptors and result identity {#identity}

A render descriptor is the canonical description of one requested calculation. It holds the schema
version, the digest of each source, the transforms of each region, the tempo map, the automation,
the routing, the descriptor of every upstream calculation, the version of each processor and of its
parameters, the sample rate, the channel layout, the precision of each internal stage, each random
seed, the render mode, the identity of the checkpoint the calculation starts from, and the requested
range with its preroll and its tails. A result is an item of the content store, and its identity is
the digest RFC 0008 computes for that item. A render
profile names one executable and numerical behavior, since different processors, vector
instructions and browser builds don't yield the same floating-point bytes; workers share a profile
only where they yield the same bytes for the same descriptor. The repeatability suite of a render
profile is a fixed set of render descriptors, each with the digest of the result that the profile
yields for it. A worker passes the suite where, for every descriptor of the suite, it renders a
result whose digest equals the digest the suite records. The manifest of a revision maps each
descriptor the revision needs to at most one validated result and pins that result. A digest
establishes a result's identity, not trust in it.

### EU-0006-011 {#EU-0006-011}

Each render request MUST be identified by its render descriptor.

### EU-0006-012 {#EU-0006-012}

A render descriptor MUST hold every input that can change the bytes of its result.

### EU-0006-013 {#EU-0006-013}

A render descriptor MUST NOT hold the identity of a worker or a wall-clock time.

### EU-0006-015 {#EU-0006-015}

A request whose descriptor and render profile equal those of a held validated result MUST be served from that result without rendering again.

### EU-0006-016 {#EU-0006-016}

Each random source of a calculation MUST be named in its render descriptor with its seed and its version.

### EU-0006-017 {#EU-0006-017}

A result MUST NOT hold processing that its render descriptor doesn't record.

### EU-0006-018 {#EU-0006-018}

A worker MUST share a render profile with other workers only once, for every descriptor of that profile's repeatability suite, it renders a result whose digest equals the digest the suite records.

### EU-0006-019 {#EU-0006-019}

A result of one render profile MUST NOT be served for a request of another render profile.

### EU-0006-020 {#EU-0006-020}

An export MUST use exactly one render profile.

### EU-0006-021 {#EU-0006-021}

The manifest of a revision MUST pin at most one validated result for each descriptor the revision needs.

### EU-0006-083 {#EU-0006-083}

Once a descriptor that a revision needs has a validated result of the project's render profile, the manifest of that revision MUST pin a validated result of that profile for that descriptor.

### EU-0006-022 {#EU-0006-022}

Where no present worker can render a project's pinned render profile, euphony MUST play the project's pinned results.

### EU-0006-023 {#EU-0006-023}

Where a project needs a new render profile, euphony MUST show the musician that it does.

### EU-0006-024 {#EU-0006-024}

A result MUST be accepted only from a worker that is one of the owner's paired computers.

### EU-0006-025 {#EU-0006-025}

A result MUST be accepted only once its descriptor, format, length and digest pass their checks.
