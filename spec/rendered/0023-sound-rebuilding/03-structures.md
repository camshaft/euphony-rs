## Proposing structures and fitting them {#structures}

A structure is a typed graph of built-in processors: a voice, such as a subtractive, wavetable,
frequency-modulation or physical-model voice, a sampler of built-in sounds or a noise source,
followed by a chain of built-in effects, such as filters, an equalizer, a compressor, saturation,
chorus, delay and reverb. Each parameter of each processor has the domain and the units the
processor catalog states. The rebuild's planner and the assistants the musician connects propose
structures, starting from the analysis of the passage. A fit tunes the settings of one structure
until its render of the passage's notes measures closest to the original. A run fits each proposed
structure, and ends by its stopping rules or when the musician stops it.

### EU-0023-017 {#EU-0023-017}

Each proposed structure MUST be a typed graph of processors that the built-in processor catalog holds.

### EU-0023-018 {#EU-0023-018}

A proposed structure MUST NOT hold code.

### EU-0023-019 {#EU-0023-019}

A rebuild MUST refuse a proposed structure that holds a processor the catalog doesn't hold.

### EU-0023-020 {#EU-0023-020}

An assistant the musician connects MUST be able to propose structures for a rebuild.

### EU-0023-021 {#EU-0023-021}

A fit MUST render the notes of the passage through its structure and measure the difference of that render from the original.

### EU-0023-022 {#EU-0023-022}

A fit MUST tune the settings of its structure to reduce its measured difference from the original.

### EU-0023-023 {#EU-0023-023}

Each fit MUST record its seed, its starting values, the versions of its processors and the version of its measurements.

### EU-0023-024 {#EU-0023-024}

A run MUST end when its held-back measures stop improving beyond their intervals.

### EU-0023-025 {#EU-0023-025}

A run MUST end when the musician stops it.
