## Cache retention and recovery {#cache}

A cached result can be rendered again, while an original recording or an accepted command can't.
Original material is kept durable and backed up as RFC 0008 states. Every manifest pins the
results it uses. A full volume holds new fetches and preparation on that
volume, frees only unpinned results that can be rendered again, keeps its verified chunks, and
resumes once space returns, while playback goes on reading the audio already on the computer. A
deprecated revision keeps a small history, as RFC 0013 states for the retention of revisions.

### EU-0006-067 {#EU-0006-067}

The render cache MUST evict only results that no manifest pins.

### EU-0006-068 {#EU-0006-068}

The render cache MUST NOT evict a result to stay under a fixed size.

### EU-0006-069 {#EU-0006-069}

Storage pressure MUST NOT cause the deletion of an original recording or an accepted command.

### EU-0006-070 {#EU-0006-070}

Storage pressure MUST NOT cause an accepted render job to be dropped.

### EU-0006-071 {#EU-0006-071}

When a volume is full, new fetches and preparation on that volume MUST hold until space returns.

### EU-0006-072 {#EU-0006-072}

A hold for storage pressure MUST be shown to the musician.

### EU-0006-073 {#EU-0006-073}

Verified chunks MUST be kept through a hold for storage pressure.

### EU-0006-074 {#EU-0006-074}

Playback MUST go on reading the prepared audio already on a computer while that computer's storage is full.

### EU-0006-075 {#EU-0006-075}

Each accepted render job MUST be recorded in the host's journal before it is acknowledged.

### EU-0006-076 {#EU-0006-076}

A restarted host MUST rebuild its queued render work from its journal and its manifests.

### EU-0006-077 {#EU-0006-077}

Pruning a unique original MUST wait for the owner's explicit approval of its exact effect.

### EU-0006-078 {#EU-0006-078}

Every local pin MUST be checked again before the results of a deprecated revision are collected.
