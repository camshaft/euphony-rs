## Source originals, import and capture {#sources}

A source original is audio as it was imported or captured. Euphony keeps it apart from working
audio, in its original bytes or in a verified lossless representation of them, such as a lossless
integer codec, so that every later conversion starts from it and never from an earlier
conversion. A signed integer source of up to 24 valid bits converts to working audio exactly,
divided by 2 to the power of one less than its valid bits. A wider integer source or a 64-bit
floating-point source doesn't convert exactly in general, so its import reports the precision the
working copy loses and waits for the musician to accept it. A capture that can't encode within its
deadline first writes an uncompressed journal, which stays recoverable source data until its
encoded object is verified and published.

### EU-0007-055 {#EU-0007-055}

Imported and captured source originals MUST be kept apart from working audio.

### EU-0007-056 {#EU-0007-056}

A source original MUST be kept as its original bytes or as a verified lossless representation of them.

### EU-0007-057 {#EU-0007-057}

A signed integer source of up to 24 valid bits MUST convert to working audio by division by 2 to the power of one less than its valid bits.

### EU-0007-058 {#EU-0007-058}

Import MUST read a packed or padded integer source by its valid bits, not by its container width.

### EU-0007-059 {#EU-0007-059}

Import MUST keep a source's bit depth, alignment and channel metadata.

### EU-0007-060 {#EU-0007-060}

Where a source can't convert to working audio exactly, its import or capture profile MUST report the precision the working copy loses.

### EU-0007-061 {#EU-0007-061}

A working copy with reduced precision MUST NOT be used before the musician accepts the reduction.

### EU-0007-062 {#EU-0007-062}

An import whose conversion isn't supported MUST leave the source original intact.

### EU-0007-063 {#EU-0007-063}

An imported original that holds a non-finite sample MUST be kept for diagnosis.

### EU-0007-064 {#EU-0007-064}

An imported original that holds a non-finite sample MUST NOT be admitted to playback.

### EU-0007-065 {#EU-0007-065}

Capture MAY write an uncompressed journal first where encoding can't meet its deadline.

### EU-0007-066 {#EU-0007-066}

Before it publishes a capture journal's encoded object, capture MUST verify that the object decodes to exactly the journal's samples.

### EU-0007-067 {#EU-0007-067}

A capture journal MUST be reclaimed only after its encoded object is published atomically.

### EU-0007-068 {#EU-0007-068}

A source original MUST NOT be deleted because an approximate working copy of it exists.
