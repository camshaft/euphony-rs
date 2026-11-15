## Station audio, clock and delay {#station-audio}

The live audio of a station uses the session's real-time transport: each stream carries media
timestamps and the session's clock mapping, its packet time is negotiated by both ends, and the
station holds a jitter buffer. Where the board of a station can discipline its audio clock in
hardware, it does; otherwise the station resamples the stream to its device clock. The monitoring
delay of a station is the sum of the host's output buffer, the packet time, the network transit, the
jitter buffer, the resampling and the period of the station's device. A loopback test measures it,
and the specification doesn't promise a number: the musician accepts a monitoring mode after the
measurement.

### EU-0015-076 {#EU-0015-076}

A station's audio stream MUST carry media timestamps and the session's clock mapping.

### EU-0015-077 {#EU-0015-077}

The packet time of a station's stream MUST be negotiated by both ends.

### EU-0015-078 {#EU-0015-078}

A station whose audio clock isn't disciplined in hardware MUST resample the stream to its device clock.

### EU-0015-079 {#EU-0015-079}

A station MUST report its clock drift and lock quality to the host.

### EU-0015-080 {#EU-0015-080}

A station that loses its clock MUST mute its output rather than play misaligned audio.

### EU-0015-081 {#EU-0015-081}

A station's monitoring delay MUST be measured by a loopback test at pairing and on each reconnection.

### EU-0015-082 {#EU-0015-082}

A station's sink MUST report its measured monitoring delay as its latency.

### EU-0015-083 {#EU-0015-083}

The readiness view MUST show the measured monitoring delay of each station.
