## Results, uncertainty and provenance {#results}

A result holds the request it answers, a typed value or a set of candidates, the units and the
vocabulary of that value with their version, the interval of the source it covers, its confidence,
its alternatives, the method and its version, the digest of any model the method used, the
execution profile, the identities of its inputs, its validation status, its warnings and a digest
of the whole result. An interval of the source is a half-open range of whole sample frames at a
stated sample rate; where a result sits in a project, and where it falls in musical time, are
separate mappings from those frames. The confidence of a result is one of four kinds: a calibrated
probability, with the calibration it comes from; an uncalibrated score of the method, with its
scale; a measurement uncertainty, with the method that computes it; or unavailable, with its
reason. Alternatives are other hypotheses the method found, kept apart from the confidence of the
value. The kinds of value are tonal labels, chord intervals, tuning, tempo segments, beat and bar
events, meter candidates, onset offsets, section and repeat links, pitch contours, note events,
instrument and tag distributions, references to stem audio, timed text and measurement series. A
digest identifies a result; it doesn't show that a musical inference is right.

### EU-0020-007 {#EU-0020-007}

Each result MUST state the method and the method version that produced it.

### EU-0020-008 {#EU-0020-008}

Each result MUST state the identity of each input it was computed from.

### EU-0020-009 {#EU-0020-009}

Each result MUST state the interval of its source audio that it covers.

### EU-0020-010 {#EU-0020-010}

Each interval of source audio MUST be a half-open range of whole sample frames at a stated sample rate.

### EU-0020-011 {#EU-0020-011}

Each result MUST carry a confidence of exactly one of the kinds calibrated probability, uncalibrated score, measurement uncertainty and unavailable.

### EU-0020-012 {#EU-0020-012}

A calibrated probability MUST name the calibration record it comes from.

### EU-0020-013 {#EU-0020-013}

An uncalibrated score MUST state its scale.

### EU-0020-014 {#EU-0020-014}

An unavailable confidence MUST state its reason.

### EU-0020-015 {#EU-0020-015}

The system MUST NOT treat an unavailable confidence as zero or as certainty.

### EU-0020-016 {#EU-0020-016}

A measurement MUST NOT carry a probability of being right.

### EU-0020-017 {#EU-0020-017}

Each result MUST keep the alternatives its method found apart from its confidence.

### EU-0020-018 {#EU-0020-018}

Each result MUST state the units and the versioned vocabulary of its value.

### EU-0020-019 {#EU-0020-019}

The system MUST refuse a result that holds a non-finite value, an invalid range, an unknown unit or a schema version that contradicts its own.

### EU-0020-020 {#EU-0020-020}

Each result MUST hold a digest of its whole content.

### EU-0020-021 {#EU-0020-021}

An incomplete result MUST state which part of its requested interval it covers.

### EU-0020-022 {#EU-0020-022}

A failed analysis MUST end in a typed outcome that names its cause, such as a conflict, missing audio, an unsupported method or profile, a cancelled job, invalid output, an unavailable model or denied access.
