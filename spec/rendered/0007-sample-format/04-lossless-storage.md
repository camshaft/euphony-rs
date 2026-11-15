## Lossless storage and raw exceptions {#lossless-storage}

Saved projects, prepared results in the cache and audio that peers sync are stored in a verified
lossless byte-preserving encoding. A lossless codec for integer audio can't hold arbitrary
floating-point samples, so working audio uses a byte-preserving compressor, without quantization or
normalization. A byte rearrangement before compression, such as grouping the bytes of each
significance, is an encoding of its own, with a version and tests that its inverse restores each
bit. Audio with high-entropy mantissa bits can compress poorly: a chunk is stored raw only where a
measurement shows that compression costs more than it saves, or misses a deadline, and the record
of the object holds that measurement. Readers take the representation of each object from its
record. Chunks are sized from measurement, not from a fixed number, and their boundaries allow
seeking, partial transfer and recovery from a corrupt chunk. Uncompressed samples belong in
processing and device buffers, in live network audio packets and in a capture journal while
encoding can't keep up.

### EU-0007-029 {#EU-0007-029}

Saved, cached and shared working audio MUST decode to exactly the samples that were encoded.

### EU-0007-030 {#EU-0007-030}

Saved, cached and synced working audio MUST be stored in a verified lossless encoding unless a measured raw exception covers it.

### EU-0007-031 {#EU-0007-031}

On the representative projects, saved, cached and shared working audio MUST use less disk space and network transfer than uncompressed audio.

### EU-0007-032 {#EU-0007-032}

The record of a stored object MUST state its representation, its codec and its codec version.

### EU-0007-033 {#EU-0007-033}

A reader MUST take an object's representation from its record.

### EU-0007-034 {#EU-0007-034}

The store MUST NOT publish a raw object whose record lacks the measurement that justified storing it raw.

### EU-0007-035 {#EU-0007-035}

The measurement of a raw object MUST hold the compressed size and the encoding cost on that chunk, or the deadline that encoding missed.

### EU-0007-036 {#EU-0007-036}

Before it publishes an encoded object, the store MUST verify its decoded length and its semantic digest.

### EU-0007-037 {#EU-0007-037}

Working audio MUST be compressed before it is encrypted.

### EU-0007-124 {#EU-0007-124}

Stored, cached and synced objects of working audio MUST move between machines only on authenticated, encrypted transfers.

### EU-0007-038 {#EU-0007-038}

A decoder MUST bound the memory it allocates by the sizes a verified manifest declares.

### EU-0007-129 {#EU-0007-129}

The manifest of an encoded object MUST hold the position and the encoded digest of each of its chunks.

### EU-0007-130 {#EU-0007-130}

The encoded digest of an encoded object MUST cover its manifest.

### EU-0007-131 {#EU-0007-131}

A reader MUST verify an encoded object's manifest against the object's encoded digest before it trusts any chunk digest the manifest holds.

### EU-0007-039 {#EU-0007-039}

Chunk boundaries MUST allow seeking, partial transfer and recovery from a corrupt chunk.

### EU-0007-040 {#EU-0007-040}

Chunk and read-ahead sizes MUST be chosen from decoder behavior, device deadlines, the measured workload and the actual resource limits.

### EU-0007-041 {#EU-0007-041}

A byte rearrangement applied before compression MUST be recorded as a versioned encoding.

### EU-0007-128 {#EU-0007-128}

The inverse of a byte rearrangement applied before compression MUST restore each bit of every chunk the rearrangement encodes.

### EU-0007-042 {#EU-0007-042}

Playback MUST stream decoded audio into reusable buffers instead of loading a project's audio into memory whole.

### EU-0007-043 {#EU-0007-043}

Readers of immutable audio MUST share its buffers instead of copying them.

### EU-0007-044 {#EU-0007-044}

The data a cache keeps resident MUST follow observed memory pressure and playback deadlines.

### EU-0007-045 {#EU-0007-045}

Compression MUST NOT run in the real-time audio callback.

### EU-0007-046 {#EU-0007-046}

Where compression or prefetch would miss a live deadline, the system MUST fall back to a path that meets the deadline.

### EU-0007-126 {#EU-0007-126}

Accepted encoding, capture and migration work MUST stay durable while the resources to run it are unavailable.

### EU-0007-047 {#EU-0007-047}

Uncompressed working audio MUST NOT be kept outside processing and device buffers, live network audio packets, export deliveries, capture journals and objects with a measured raw exception.
