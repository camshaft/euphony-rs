## Combining routes with automation {#combination}

Each source conversion produces a dimensionless value x in its declared unipolar or bipolar domain.
A route turns x into a contribution c, its offset plus its depth times its curve of x, and clips c
to its output range. A curve is a versioned piecewise mapping with a stored interpolation and
defined end points. The contributions to one target are summed into m. The base a of a target is
its evaluated track or region automation, or its manual value where it has none. In Add mode the
normalized value is a plus m; in Scale mode, a times the sum of one and m; in Replace mode, b plus
m, where b is a stored replacement base. The result is clamped once to the target's domain and
then mapped to physical units, so a route to a frequency with a logarithmic mapping moves it in
proportion rather than by a fixed number of hertz. Recording automation writes the base, and a
bake writes the effective value into the base and disables the routes it covers.

### EU-0012-023 {#EU-0012-023}

Each source conversion MUST produce a dimensionless value in its declared unipolar or bipolar domain.

### EU-0012-024 {#EU-0012-024}

A route's contribution MUST be its offset plus its depth times its curve applied to the source's value.

### EU-0012-025 {#EU-0012-025}

A route's output range MUST clip its contribution.

### EU-0012-026 {#EU-0012-026}

A route's curve MUST be a versioned piecewise mapping with a stored interpolation and defined end points.

### EU-0012-027 {#EU-0012-027}

The contributions to one target MUST be summed in the order of their route identifiers, with the accumulator the project's numerical profile declares.

### EU-0012-028 {#EU-0012-028}

Each target MUST apply exactly one combination mode, Add, Scale or Replace, to its summed contribution.

### EU-0012-029 {#EU-0012-029}

In Add mode, the normalized value of a target MUST be its base plus its summed contribution.

### EU-0012-030 {#EU-0012-030}

In Scale mode, the normalized value of a target MUST be its base times the sum of one and its summed contribution.

### EU-0012-031 {#EU-0012-031}

In Replace mode, while at least one of its routes is present and not bypassed, the normalized value of a target MUST be its stored replacement base plus its summed contribution.

### EU-0012-032 {#EU-0012-032}

When a target's mode is set to Replace, its replacement base MUST start at the target's unmodulated value.

### EU-0012-033 {#EU-0012-033}

The combined value MUST be clamped once to the target's domain, before it is mapped to physical units.

### EU-0012-034 {#EU-0012-034}

A target whose routes are all absent or bypassed MUST take its unmodulated base in every mode.

### EU-0012-035 {#EU-0012-035}

A value that isn't finite MUST NOT reach the renderer.

### EU-0012-036 {#EU-0012-036}

The base of a target MUST be its evaluated track or region automation, or its manual value where it has no automation.

### EU-0012-037 {#EU-0012-037}

Recording automation on a target MUST write its base lane, not its modulated value.

### EU-0012-038 {#EU-0012-038}

A bake MUST write a target's effective value into its automation and disable the routes it covers in the same change.

### EU-0012-039 {#EU-0012-039}

A target MUST combine in Add mode until the musician chooses another mode.

### EU-0012-040 {#EU-0012-040}

A newly connected route MUST start at a depth of zero.

### EU-0012-041 {#EU-0012-041}

Each change of a combination mode, an offset, a depth or a curve MUST make a new revision of the project that can be undone.

### EU-0012-042 {#EU-0012-042}

A route MUST NOT replace another route's contribution to the same target.

### EU-0012-043 {#EU-0012-043}

Two collaborative changes of one target's combination mode MUST conflict when they merge.

### EU-0012-163 {#EU-0012-163}

Moving a region MUST change only the automation lanes that the region holds.

### EU-0012-169 {#EU-0012-169}

Editing the tempo map MUST change only the automation lanes that resolve their positions through the tempo map.
