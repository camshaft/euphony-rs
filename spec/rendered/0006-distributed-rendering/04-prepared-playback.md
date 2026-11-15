## Playing prepared audio {#prepared-playback}

Stable passages are prepared as full-mix segments, and stems are kept at useful dependency
boundaries, so that an edit of one track plays the stems it doesn't affect while its affected island
and the buses that depend on it are prepared again. The player streams prepared results from the
render cache, can start partway through a result, and reads and frees memory away from the deadline
of the audio device. How the player switches between prepared and live sources, and what the
musician listens to while an edited passage is prepared, belong to RFC 0005.

### EU-0006-039 {#EU-0006-039}

Each playback segment MUST come from a single revision of the graph.

### EU-0006-040 {#EU-0006-040}

Each playback segment MUST come from a single render profile.

### EU-0006-041 {#EU-0006-041}

A completed export MUST check that every segment belongs to its selected revision and render profile.

### EU-0006-042 {#EU-0006-042}

The player MUST NOT wait on a read of the render cache while it produces audio for the device.

### EU-0006-043 {#EU-0006-043}

Playback MUST be able to start at any sample position inside a prepared result.

### EU-0006-044 {#EU-0006-044}

Stable passages SHOULD be prepared as full-mix segments.

### EU-0006-045 {#EU-0006-045}

The render cache SHOULD keep stems at the dependency boundaries of the mix.
