## Playing prepared audio {#prepared-playback}

Stable passages are prepared as full-mix segments, and stems are kept at useful dependency
boundaries, so that an edit of one track plays the stems it doesn't affect while its affected island
and the buses that depend on it are prepared again. How the player streams prepared results, and how
it switches between prepared and live sources, belong to RFC 0005.

### EU-0006-040 {#EU-0006-040}

Each playback segment MUST come from a single render profile.

### EU-0006-041 {#EU-0006-041}

A completed export MUST check that every segment belongs to its selected revision and render profile.

### EU-0006-044 {#EU-0006-044}

Stable passages SHOULD be prepared as full-mix segments.

### EU-0006-045 {#EU-0006-045}

The render cache SHOULD keep stems at the dependency boundaries of the mix.
