## Decoders at peers and browsers {#negotiation}

A peer or a browser becomes ready to play or sync only once it holds a decoder for each encoding it
will read. A peer that lacks a codec negotiates another supported lossless encoding, or the same
sample bits raw where a measured raw exception covers them, and otherwise reports that it lacks a
decoder. Peers exchange audio only in schema and profile versions both support, and work that one
of them can't read yet is held where the musician sees it. No peer asks for integer audio instead
of working audio.

### EU-0007-048 {#EU-0007-048}

A peer or a browser MUST hold a decoder for each encoding it reads before it reports itself ready.

### EU-0007-049 {#EU-0007-049}

A peer or a browser that lacks a codec MUST negotiate another supported lossless encoding of the same samples.

### EU-0007-050 {#EU-0007-050}

A peer or a browser MUST NOT receive a raw representation of working audio unless a measured raw exception covers it.

### EU-0007-051 {#EU-0007-051}

Where no supported lossless encoding is left and no measured raw exception covers a raw representation, readiness MUST report the unsupported decoder.

### EU-0007-052 {#EU-0007-052}

A peer MUST NOT request integer quantization of working audio.

### EU-0007-053 {#EU-0007-053}

Peers MUST exchange working audio only in schema and profile versions that both support.

### EU-0007-054 {#EU-0007-054}

Work that a peer cannot read for want of a supported version MUST be held where the musician sees it.
