## The strip and its stages {#the-strip}

Every audio track and every instrument track of a project has one channel strip. The strip is a fixed,
ordered set of stages, the same on every such track, so that a musician who learns one strip can use
every strip. Each stage is a built-in processor of euphony, as RFC 0010 specifies built-in processors,
or a model of gear the musician captured. The stages are, in order: input, with trim, polarity and a
short alignment delay; filters, a low cut and a high cut; a gate that quiets noise and spill from
other instruments between notes; a four-band equalizer; a compressor that reduces the difference in
loudness between loud and quiet passages; a de-esser that softens harsh s sounds; color, which adds
saturation for warmth; free slots for further effects in any order; sends to effect returns that
several tracks share; and pan with the fader. An auxiliary channel and the output channel have a
shorter strip. The stages and their order come from the strip layout, a versioned record of the project.

### EU-0011-001 {#EU-0011-001}

Every audio track and every instrument track MUST have exactly one channel strip.

### EU-0011-002 {#EU-0011-002}

The channel strip of an audio or instrument track MUST hold these stages in this order: input, filters, gate, equalizer, compressor, de-esser, color, free slots, sends, and pan with the fader.

### EU-0011-003 {#EU-0011-003}

The stages of a strip and their order MUST come from the project's strip layout, a versioned record of the project.

### EU-0011-004 {#EU-0011-004}

Each stage MUST be a built-in processor of euphony or a model of gear the musician captured.

### EU-0011-005 {#EU-0011-005}

The input stage MUST offer a trim, a polarity inversion and a short delay for aligning microphones.

### EU-0011-006 {#EU-0011-006}

The filters stage MUST offer a low cut and a high cut.

### EU-0011-007 {#EU-0011-007}

The gate stage MUST reduce the track's level by its range while the level its detector reads is below its threshold.

### EU-0011-008 {#EU-0011-008}

The equalizer stage MUST offer four bands: a low shelf, a high shelf and two peaking bands.

### EU-0011-009 {#EU-0011-009}

The compressor stage MUST reduce the track's level above its threshold by its ratio, with an attack and a release.

### EU-0011-010 {#EU-0011-010}

The de-esser stage MUST reduce short loud events in a band of the high frequencies without reducing the rest of the track.

### EU-0011-011 {#EU-0011-011}

The color stage MUST add saturation, from the strip's character or from a model of gear the musician captured.

### EU-0011-012 {#EU-0011-012}

The free slots MUST hold any number of built-in effects and captured models, in the order the musician chooses.

### EU-0011-013 {#EU-0011-013}

Each send MUST pass a share of the track that the musician sets to a shared effect return of the project.

### EU-0011-014 {#EU-0011-014}

The musician MUST be able to place each send before or after the fader.

### EU-0011-015 {#EU-0011-015}

The last stage of a track's strip MUST offer pan or stereo width, and the fader.

### EU-0011-016 {#EU-0011-016}

Each strip MUST offer one switch that swaps the order of the equalizer and the compressor.

### EU-0011-017 {#EU-0011-017}

The musician MAY set the filters stage to filter only the signal the gate's detector reads.

### EU-0011-018 {#EU-0011-018}

The strip of an auxiliary channel MUST hold an equalizer, a compressor and a color stage.

### EU-0011-019 {#EU-0011-019}

The strip of the output channel MUST hold an equalizer, a compressor, a color stage and a limiter.

### EU-0011-020 {#EU-0011-020}

The musician MUST be able to switch each stage on or off with one action.

### EU-0011-021 {#EU-0011-021}

Each stage of a new track MUST start switched off.
