## Headphone stations {#stations}

A headphone station is a small networked computer with a headphone output, rotary encoders with
push switches, and lights, placed where a listener sits. It joins a session on the local network and
is the sink of one listener. The session host calculates the station's sink mix as it calculates
every other sink mix and sends the station the finished stream, which the station plays after its
own output protection. The station doesn't run the session host, compile the graph or render for the
session, and doesn't hold project objects or stems. The hardware description of a station lists its
encoders, switches and light groups by stable identity, its output formats and its headphone socket state. A
station reaches its listener's sink alone: that sink's stream and meters, its mix parameters, its mix
revision and its control map. A wired network connection is the recommended path, and a wireless
connection is supported and measured. A station isn't a mixer on its own: it plays and controls its
listener's mix only while it is connected to the session's host.

### EU-0015-041 {#EU-0015-041}

A headphone station MUST appear in its session as the sink of one listener.

### EU-0015-042 {#EU-0015-042}

The host MUST calculate a station's sink mix as it calculates every other sink mix.

### EU-0015-043 {#EU-0015-043}

A station MUST NOT receive the separate sources of its listener's mix.

### EU-0015-044 {#EU-0015-044}

A station MUST NOT hold project objects.

### EU-0015-045 {#EU-0015-045}

A station MUST NOT play or control a mix while it isn't connected to the session's host.

### EU-0015-046 {#EU-0015-046}

A station's hardware description MUST list each of its encoders, switches and light groups by a stable identity.

### EU-0015-047 {#EU-0015-047}

A station MUST read the rates, formats, period sizes and headphone detection its output device supports from the device.

### EU-0015-048 {#EU-0015-048}

A station MUST apply its output protection before it plays its sink's stream.

### EU-0015-049 {#EU-0015-049}

Once paired and connected, a station MUST register its sink with its device and stream identity, channel layout, supported formats, measured latency, clock domain and headphone socket state.

### EU-0015-050 {#EU-0015-050}

Where its hardware reports headphone insertion, a station MUST report each insertion and removal as an event.

### EU-0015-051 {#EU-0015-051}

Where its hardware doesn't report headphone insertion, a station MUST report its headphone socket state as unknown.

### EU-0015-052 {#EU-0015-052}

The host MUST take a station's class from its pairing record, whatever the station sends.

### EU-0015-053 {#EU-0015-053}

The host MUST NOT give a station any record other than its own sink's stream, meters, mix parameters, mix revision and control map.

### EU-0015-054 {#EU-0015-054}

The host MUST NOT accept a station's command for anything other than its own sink's mix parameters.

### EU-0015-055 {#EU-0015-055}

A station SHOULD connect to the session over a wired network.

### EU-0015-056 {#EU-0015-056}

The readiness view MUST label a station on a wireless connection as wireless.
