## Scope of analysis {#scope}

Analysis estimates what a recording holds and measures how it sounds. Its input is any audio a
project holds: an imported sample, a recorded take, a stem, a selected region of any of these, or a
prepared mix. Its results are of two kinds. A measurement computes a defined quantity of the signal,
such as loudness, true peak, loudness range, crest factor, spectral balance, stereo width and phase
correlation. An inference estimates musical content, such as key and mode, tuning reference, tempo
and tempo map, beats and downbeats, time signature, swing and groove, sections and repeats, melody
and pitch contour, note events, vibrato, instruments and timbre, separated stems, the behavior of
dynamics, reverb and other effects, lyrics, and mood and genre tags. A method is one versioned
analyzer that produces results of one or more of these families. Any method can abstain: a result
can be absent, ambiguous or outside what the method supports. Analysis doesn't recover the original
session, its tracks or its arrangement from a finished mix, and it doesn't identify the one preset
or effects chain that made a sound; for a sound, it offers candidates that the musician compares by
ear.

### EU-0020-001 {#EU-0020-001}

Analysis MUST accept as its input any audio a project holds, including a sample, a take, a stem, a selected region and a prepared mix.

### EU-0020-002 {#EU-0020-002}

The system MUST list each available method with its version, the result families it produces and the material it supports.

### EU-0020-003 {#EU-0020-003}

A request for analysis MUST name its source audio, the interval of that audio to analyze and the method.

### EU-0020-004 {#EU-0020-004}

A method MUST be able to answer any request with an abstention that states its reason.

### EU-0020-005 {#EU-0020-005}

Each measurement MUST state the definition it computes and the revision of that definition.

### EU-0020-006 {#EU-0020-006}

A result about effects or sound sources MUST NOT be presented as the identification of the original chain or its settings.
