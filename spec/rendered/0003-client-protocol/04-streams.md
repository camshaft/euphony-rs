## Display streams, waveforms and content {#streams}

Display streams carry the values that change many times a second: meters and level history, the
playhead and the transport position, and analysis displays such as the gain reduction of a
compressor or a spectrum. Each stream is a sequence of frames, of which a client shows the newest at
each refresh of its display, and the engine publishes each frame without waiting for any reader.
Deltas follow the same rule: the session host holds the deltas a client has not yet consumed, and
where a client falls behind further than that, it ends the subscription rather than waiting. A
waveform tile holds the peak data of one audio item at one zoom level and position; since the
content of an item never changes, neither does its tile. Content transfer covers the imports,
exports and recordings that a client sends or receives. A client on the same computer hands the
engine the file the person chose, as RFC 0002 states, and a client on another computer sends the
content into the content store of the session. No client writes the memory of the engine, and the
engine treats every message from a client as untrusted until it has checked it.

### EU-0003-049 {#EU-0003-049}

Each display stream MUST carry its values as frames of which a client shows only the newest.

### EU-0003-050 {#EU-0003-050}

Each display frame MUST state its age.

### EU-0003-051 {#EU-0003-051}

The engine MUST NOT wait for a client before it publishes a display frame or a delta.

### EU-0003-052 {#EU-0003-052}

A client MUST NOT show a display frame made of parts of two frames.

### EU-0003-053 {#EU-0003-053}

Where the audio process stops, a client MUST show the last complete display frame with its age growing.

### EU-0003-054 {#EU-0003-054}

Where a client falls further behind a stream of deltas than the session host holds for it, the session host MUST end that subscription.

### EU-0003-055 {#EU-0003-055}

A client whose subscription ended MUST take a fresh snapshot before it shows the session again.

### EU-0003-056 {#EU-0003-056}

A client that falls behind MUST NOT cause an accepted change to be lost.

### EU-0003-057 {#EU-0003-057}

An acknowledgement of a delta that is forged, repeated, out of order or from an ended generation MUST NOT release that delta before the client has consumed it.

### EU-0003-058 {#EU-0003-058}

A waveform tile MUST be identified by the identity of its audio item, its zoom level, its position and the version of its peak format.

### EU-0003-059 {#EU-0003-059}

A client MAY keep a waveform tile by its identity without ever invalidating it.

### EU-0003-062 {#EU-0003-062}

A client on another computer MUST send the content it imports into the content store of the session.

### EU-0003-063 {#EU-0003-063}

A client MUST NOT be able to write any memory of the engine.

### EU-0003-064 {#EU-0003-064}

A malformed or oversized message MUST NOT change any state of the session.
