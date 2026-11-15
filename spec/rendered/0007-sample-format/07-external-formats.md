## Devices, network audio and export {#external-formats}

Audio devices and network audio endpoints use the sample formats they negotiate, and euphony
doesn't require them to accept floating-point audio. Incoming integer samples convert to working
audio once, at the engine's adapter; outgoing working audio converts after the final gain, any
resampling and output protection, under a declared rounding, saturation and dither policy. Dither
belongs at a lossy reduction to integer precision, where the delivery calls for it, never in cache
compression or peer sync. Live safety stays with the local output protection, and the range of
floating-point samples doesn't allow unsafe output levels. An export is a delivery of a render to
a destination: the native lossless bundle, an uncompressed floating-point file, an integer file or
another verified target, such as an immersive-audio adapter. A delivery to integer audio is
quantized on purpose, and is lossless only relative to its integer samples.

### EU-0007-069 {#EU-0007-069}

An audio device or a network audio endpoint MUST receive samples in the external format it negotiated.

### EU-0007-070 {#EU-0007-070}

Incoming integer samples MUST convert to working audio once, at the engine's device or network adapter.

### EU-0007-071 {#EU-0007-071}

Outgoing working audio MUST convert to an external format only after the final sink gain, any resampling and output protection.

### EU-0007-072 {#EU-0007-072}

A conversion to an external integer format MUST follow a declared rounding, saturation and dither policy.

### EU-0007-073 {#EU-0007-073}

A conversion to an external format MUST check that each value is finite and within the destination's range.

### EU-0007-074 {#EU-0007-074}

A conversion to an external format MUST report each overload of the destination's range.

### EU-0007-075 {#EU-0007-075}

Dither MUST NOT be applied in cache compression or peer sync.

### EU-0007-076 {#EU-0007-076}

An offline export profile MUST hold its dither algorithm and seed.

### EU-0007-077 {#EU-0007-077}

The range of floating-point samples MUST NOT raise the output level the local output protection allows.

### EU-0007-078 {#EU-0007-078}

The default rendered-file export MUST be an uncompressed floating-point file at the working precision where the destination supports it.

### EU-0007-079 {#EU-0007-079}

A floating-point file export MUST keep the bits of each finite sample at the selected rate and layout, without normalization.

### EU-0007-080 {#EU-0007-080}

The export preview MUST disclose each change of rate, layout or gain the export makes.

### EU-0007-081 {#EU-0007-081}

The export preview MUST distinguish the native lossless bundle, floating-point files, integer files and each other verified target.

### EU-0007-082 {#EU-0007-082}

An integer export MUST state its bit depth and its conversion policy.

### EU-0007-083 {#EU-0007-083}

An integer export MUST warn of out-of-range values before it completes.

### EU-0007-084 {#EU-0007-084}

An export to integer audio MUST NOT be described as lossless relative to the floating-point render.

### EU-0007-085 {#EU-0007-085}

An export MUST leave the floating-point render and the source originals in place.

### EU-0007-086 {#EU-0007-086}

A raw copy of an export kept in the project store MUST hold the measurement a raw exception needs.
