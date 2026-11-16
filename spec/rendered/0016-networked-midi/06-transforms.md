## Live transforms {#transforms}

A transform is a version of a program in the composition language, which the composition RFC
specifies, written for its live entry point. It accepts one timestamped event, its declared typed
parameters and bounded state, and returns validated events or a typed diagnostic. The live profile
leaves out ambient input and output, unbounded loops, recursive expansion and undeclared timers. A
transform's compile result holds the digests of its source, library and runtime, its parameters,
the event classes it supports, its greatest output expansion, inferred from the validated program,
and its resource needs. A person attaches a transform in the route editor: they type-check it,
preview it against recorded input, inspect its destinations and measured readiness, and then commit
the route. A host execution profile runs a route's transforms on its MIDI host instead of the
session host, and the owner selects it for that host.

For example, a transform projects chromatic input onto C major by nearest note, with a note equally
far from two scale notes going to the lower one, keeps the scale degree and octave, and maps the
result onto D Dorian. MIDI notes 60 through 71 become 62, 62, 64, 64, 65, 67, 67, 69, 69, 71, 71
and 72. The projection, the change of tonic and the change of mode are separate typed operations,
and the interface shows the projection rule and the source and target tonic and mode. A musician
can choose another explicit projection rule instead of nearest note.

### EU-0016-098 {#EU-0016-098}

A transform MUST be a program in the composition language, written for its live entry point.

### EU-0016-099 {#EU-0016-099}

A transform MUST be type-checked and compiled before it is activated.

### EU-0016-100 {#EU-0016-100}

The session host MUST reject a program that uses a feature outside the live profile, with a diagnostic that points at its source.

### EU-0016-101 {#EU-0016-101}

The session host MUST validate the pitch, channel and range of each event a transform outputs as it runs.

### EU-0016-102 {#EU-0016-102}

A transform MUST run apart from audio playback, so that its failure or delay leaves playback and recording running.

### EU-0016-103 {#EU-0016-103}

A route's transforms MUST run on the session host unless the owner selects a host execution profile that has passed the same output, isolation and timing tests on that host's hardware.

### EU-0016-104 {#EU-0016-104}

Audio playback MUST receive only validated, timestamped events from a transform.

### EU-0016-105 {#EU-0016-105}

A route MUST record exactly one execution site and generation for its transforms.

### EU-0016-106 {#EU-0016-106}

A reconnection or a move of execution site MUST NOT apply a transform to an event twice.

### EU-0016-107 {#EU-0016-107}

Moving a route's transforms to another execution site MUST release the notes they own.

### EU-0016-108 {#EU-0016-108}

Bypassing a route's transforms MUST be an explicit revision of the route.

### EU-0016-109 {#EU-0016-109}

A failure MUST NOT bypass a route's transforms.

### EU-0016-110 {#EU-0016-110}

A change of a transform's source MUST create a new immutable version.

### EU-0016-111 {#EU-0016-111}

The session host MUST reserve a transform's state and output allocation before it activates the transform.

### EU-0016-112 {#EU-0016-112}

Admission MUST compare a transform's resource needs with the runtime and device capacity measured on the target hardware.

### EU-0016-113 {#EU-0016-113}

A program whose live resource bound can't be established MUST NOT be admitted live.

### EU-0016-114 {#EU-0016-114}

A program that isn't admitted live MUST stay usable for offline composition.

### EU-0016-115 {#EU-0016-115}

A missed transform deadline or exhausted event capacity MUST suspend the affected route.

### EU-0016-116 {#EU-0016-116}

Raw input and transformed output MUST be recordable as distinct streams.

### EU-0016-117 {#EU-0016-117}

A recording of transformed output MUST hold the transform's identity, parameters, seed, route generation and timing mapping.

### EU-0016-130 {#EU-0016-130}

After a move of execution site, a route's transforms MUST start with clean state.

### EU-0016-131 {#EU-0016-131}

When a route's transform stops, for any reason, the session host MUST release each note the route owns wherever its instrument is still reachable.

### EU-0016-132 {#EU-0016-132}

A transform MUST accept one timestamped event, its declared typed parameters and bounded state, and return validated events or a typed diagnostic.

### EU-0016-133 {#EU-0016-133}

A transform's compile result MUST hold the digests of its source, library and runtime, its parameters, the event classes it supports, its greatest output expansion inferred from the validated program, and its resource needs.

### EU-0016-134 {#EU-0016-134}

The route editor MUST let a person preview a transform against recorded input before they commit the route.

### EU-0016-135 {#EU-0016-135}

The route editor MUST show a transform's destinations and measured readiness before the route is committed.

### EU-0016-136 {#EU-0016-136}

A transform that projects chromatic input onto C major by nearest note, with a note equally far from two scale notes going to the lower one, and maps the result onto D Dorian keeping scale degree and octave, MUST map MIDI notes 60 through 71 to 62, 62, 64, 64, 65, 67, 67, 69, 69, 71, 71 and 72.

### EU-0016-137 {#EU-0016-137}

The projection, the change of tonic and the change of mode MUST be separate typed operations.

### EU-0016-138 {#EU-0016-138}

The interface MUST show the projection rule and the source and target tonic and mode of each transform that projects onto a scale.

### EU-0016-139 {#EU-0016-139}

The interface MUST let a musician choose an explicit projection rule other than nearest note.
