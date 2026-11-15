## Switching, delay and rendering {#switching}

A stage that is switched off isn't part of the track's processing at all. Switching a stage on or
off is a change of the project that the live render crosses at one sample position, with a short
crossfade. Each switched-on stage declares the delay it adds, and the project's delay compensation,
as RFC 0005 specifies it, uses that declaration. In its default mode a stage doesn't add delay of
its own, apart from the few samples the color stage's oversampling adds. Optional modes add declared
delay: a linear-phase equalizer, and look-ahead for the compressor, the de-esser and the output
limiter. The stages render on other computers as any other processing does, as RFC 0006 specifies.

### EU-0011-022 {#EU-0011-022}

A switched-off stage MUST NOT add processing to its track.

### EU-0011-023 {#EU-0011-023}

A switched-off stage MUST NOT add delay to its track.

### EU-0011-024 {#EU-0011-024}

Switching a stage on or off during playback MUST take effect at the same sample position on every track and output it affects.

### EU-0011-025 {#EU-0011-025}

Switching a stage on or off MUST cross from the earlier output to the new one with a short crossfade, so that the output has no discontinuity.

### EU-0011-026 {#EU-0011-026}

Each switched-on stage MUST declare the delay it adds.

### EU-0011-027 {#EU-0011-027}

The measured delay of a stage MUST equal its declared delay on each supported computer.

### EU-0011-028 {#EU-0011-028}

In its default mode, a stage other than the color stage MUST NOT add delay.

### EU-0011-029 {#EU-0011-029}

The equalizer MAY offer a linear-phase mode, whose delay it declares.

### EU-0011-030 {#EU-0011-030}

The compressor, the de-esser and the output limiter MAY offer a look-ahead mode, whose delay each declares.

### EU-0011-031 {#EU-0011-031}

The project's delay compensation MUST use each stage's declared delay.

### EU-0011-032 {#EU-0011-032}

On a track armed for live monitoring, a stage mode whose delay exceeds the musician's live-latency setting MUST NOT be available.

### EU-0011-033 {#EU-0011-033}

The strip MUST show why a stage mode is unavailable on a track armed for live monitoring.

### EU-0011-034 {#EU-0011-034}

A strip MUST render the same output on every computer of a compatible numerical profile.

### EU-0011-035 {#EU-0011-035}

A change of one stage MUST invalidate only the rendered audio of its track and of what depends on that track.

### EU-0011-036 {#EU-0011-036}

Where the computer's measured capacity can't run a strip live, the track MUST play its prepared audio with the strip's settings kept.
