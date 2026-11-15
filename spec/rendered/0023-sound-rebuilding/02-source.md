## Choosing the source and reading its analysis {#source}

A rebuild from a passage starts from an item of the content store and a half-open interval of its
samples, or from a stem that the analysis separated from such an item. The rebuild reads the
analysis results for the passage: notes, pitch contour, onsets, loudness, spectral balance,
instrument tags and effect hypotheses, each with its confidence. Notes the musician enters or
corrects come before the estimates. A sound inside a full mix is rebuilt from a separated stem,
which is an estimate of the sound and not the sound alone, and what separation leaves in the stem
limits the comparison.

### EU-0023-010 {#EU-0023-010}

The request of a rebuild from a passage MUST record the digest of the source item and the interval of its samples.

### EU-0023-011 {#EU-0023-011}

A rebuild from a passage MUST read the analysis results for the passage, each with its confidence.

### EU-0023-012 {#EU-0023-012}

Where the musician entered or corrected notes for the passage, the rebuild MUST use them instead of the estimated notes.

### EU-0023-013 {#EU-0023-013}

Where an analysis result that a rebuild needs is missing, the rebuild MUST request it through the ordinary analysis request and show that it waits on it.

### EU-0023-014 {#EU-0023-014}

A rebuild whose target is a separated stem MUST report that its target is an estimate of the sound rather than the sound alone.

### EU-0023-015 {#EU-0023-015}

A rebuild whose target is a separated stem MUST show in its comparison what separation left in the target.

### EU-0023-016 {#EU-0023-016}

A rebuild MUST NOT read audio other than audio the musician holds.
