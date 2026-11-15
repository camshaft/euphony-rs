## Canonical working audio and its identity {#canonical-audio}

Canonical working audio is a versioned header, which declares the sample format, the sample rate,
the channel order and the frame count, followed by the samples interleaved by frame, the bits of
each sample in little-endian byte order. Its semantic digest covers the header and the samples, and
names the audio whatever encoding stores it; an encoded object has a digest of its encoded bytes
beside it. A processor can hold samples in planar buffers inside, but every interface that
exchanges working audio exchanges the canonical form. Encoding, decoding, a change of byte order
and a change of layout keep each sample's bits, signed zero and subnormal values included. The
store keeps one canonical content once, whatever number of projects and listeners read it.

### EU-0007-019 {#EU-0007-019}

Canonical working audio MUST begin with a versioned header that declares the sample format, the sample rate, the channel order and the frame count.

### EU-0007-020 {#EU-0007-020}

After its header, canonical working audio MUST hold its samples interleaved by frame, each in little-endian byte order.

### EU-0007-021 {#EU-0007-021}

The semantic digest of working audio MUST cover its canonical header and its samples.

### EU-0007-022 {#EU-0007-022}

An encoded object MUST have a digest of its encoded bytes that is separate from the semantic digest of the audio it holds.

### EU-0007-125 {#EU-0007-125}

A reader MUST verify an encoded object's encoded digest before it decodes the object.

### EU-0007-023 {#EU-0007-023}

Every interface that exchanges working audio MUST exchange it in the canonical form.

### EU-0007-024 {#EU-0007-024}

Encoding, decoding, byte-order conversion and layout conversion MUST keep the bits of each sample, signed zero and subnormal values included.

### EU-0007-025 {#EU-0007-025}

Storage, caching and sync MUST NOT change the value of any sample of working audio.

### EU-0007-026 {#EU-0007-026}

The store MUST deduplicate working audio by its semantic digest.

### EU-0007-027 {#EU-0007-027}

The store MUST keep an additional encoded representation of the same canonical content only while an active reader needs it.

### EU-0007-028 {#EU-0007-028}

The encoding of a stored result MUST NOT change the identity of the render calculation that made it.
