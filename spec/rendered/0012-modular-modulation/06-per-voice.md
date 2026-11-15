## Per-voice modulation {#per-voice}

A route's scope is global or per-voice. A per-voice route reads a source of one voice, such as a
note's velocity, key, release velocity or expression, or an envelope or oscillator that starts with
the note, and reaches a control of one voice of a processor that plays several notes at once. A
voice is identified by the port, channel, key and note identifier of the note that started it, and
lives from its note-on until its processor reports its end after the release tail. For each voice,
the summed contribution is the global contribution plus that voice's own contributions, and the
target's combination mode, clamp and mapping then apply to it.

### EU-0012-096 {#EU-0012-096}

Each route MUST store its scope, global or per-voice.

### EU-0012-097 {#EU-0012-097}

The source of a per-voice route MUST be a source of one voice: a note's velocity, key, release velocity or expression, or an envelope or oscillator that starts with the note.

### EU-0012-098 {#EU-0012-098}

The target of a per-voice route MUST be a control of one voice of a processor that plays several notes at once.

### EU-0012-099 {#EU-0012-099}

The parameter registry MUST declare for each control whether it accepts per-voice modulation.

### EU-0012-100 {#EU-0012-100}

A preview MUST refuse a per-voice route to a control that doesn't accept it, with that reason, unless the musician chooses the global scope.

### EU-0012-101 {#EU-0012-101}

A voice MUST be identified by the port, channel, key and note identifier of the note that started it.

### EU-0012-102 {#EU-0012-102}

A note identifier MUST NOT depend on the order in which notes arrive from several computers.

### EU-0012-103 {#EU-0012-103}

A voice MUST exist from its note-on until its processor reports its end after the release tail.

### EU-0012-104 {#EU-0012-104}

The per-voice modulator state of a voice MUST end when the voice ends, including when the voice is stolen.

### EU-0012-105 {#EU-0012-105}

Voice allocation and stealing MUST follow the processor's declared deterministic policy over the note events.

### EU-0012-106 {#EU-0012-106}

For each voice, the summed contribution MUST be the global contribution plus the voice's own contributions, summed in the order of their route identifiers.

### EU-0012-107 {#EU-0012-107}

A source of the whole instrument that reaches a per-voice target MUST reach every voice with the same value.

### EU-0012-108 {#EU-0012-108}

A per-voice source that reaches a global target MUST be aggregated to one value by the route's stored aggregation: sum, maximum, mean or newest voice.

### EU-0012-109 {#EU-0012-109}

The aggregation of a new route MUST be the sum.

### EU-0012-110 {#EU-0012-110}

With no active voice, an aggregation MUST return the neutral contribution.

### EU-0012-111 {#EU-0012-111}

The modulator state of each voice MUST be kept under the route's stored reset policy.

### EU-0012-112 {#EU-0012-112}

A checkpoint MUST hold the per-voice state of each active voice.

### EU-0012-113 {#EU-0012-113}

The renderer MUST send per-voice modulation to a hosted processor as events addressed by voice, with the global contribution already included.

### EU-0012-114 {#EU-0012-114}

Voice allocation and per-voice values MUST match in prepared playback, in live playback and after a checkpoint is restored.
