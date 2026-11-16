## Views and assistants {#clients}

The analysis actions are: list the methods and what they produce; request an analysis and cancel
it; inspect the state of a job and read its results a page at a time; compare the results of
methods; propose a correction; preview and apply an accepted interpretation; and remove a result
that no project keeps. A reader can read each result's series, its alternatives, its provenance and
the evaluation reports of its method, and can follow the progress, the completion, the invalidation
and the correction of the results it reads. The views for people are a key and chord lane, a tuning
display, an editable map of beats, tempo and meter, an overlay of sections and repeats, a preview of
melody and notes, an audition of stems, a browser of instruments and tags, a lyric lane, and meters
of loudness, spectrum and stereo. An assistant the owner connects works with the same authority as
the musician, and neither interface overwrites the musician's music.

### EU-0020-044 {#EU-0020-044}

The interface for people and the interface for assistants MUST offer the same analysis actions.

### EU-0020-045 {#EU-0020-045}

An assistant the owner authorized MUST be able to inspect and correct results with the same authority as the musician.

### EU-0020-046 {#EU-0020-046}

The system MUST notify each subscribed reader of the progress, completion, invalidation and correction of the results it follows.

### EU-0020-047 {#EU-0020-047}

A reader that cannot hold a subscription MUST be able to resume from a durable continuation instead of polling.

### EU-0020-048 {#EU-0020-048}

A query of an analysis that has not finished MUST return the part it covers so far and a cursor to continue from.

### EU-0020-049 {#EU-0020-049}

Each view MUST show whether the data it shows is measured, inferred, corrected, stale, incomplete or unsupported.

### EU-0020-050 {#EU-0020-050}

Each result shown to a musician or an assistant MUST show its uncertainty and its source passage.

### EU-0020-051 {#EU-0020-051}

Selecting a passage of a result MUST show its source range, its method, the kind of its confidence and its alternatives.

### EU-0020-052 {#EU-0020-052}

The system MUST NOT treat text in a result, such as lyrics, imported labels or an assistant's prose, as an instruction or a grant.
