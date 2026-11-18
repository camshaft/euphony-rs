## Stations joining and leaving {#joining-and-leaving}

A station leaves when its connection closes, when the transport reports it lost, when it powers off
or when it is unpaired. Its sink then becomes unavailable and the host stops calculating its output.
Its sink mix revision and its control map stay in the session, so a station that reconnects plays the
same mix once it is validated again. Removing the headphones keeps the station connected and mutes
its output until headphones are inserted again.

### EU-0015-105 {#EU-0015-105}

A station joining or leaving a session MUST NOT change the output of any other sink.

### EU-0015-106 {#EU-0015-106}

A station joining or leaving a session MUST NOT change the main mix.

### EU-0015-107 {#EU-0015-107}

A station joining or leaving a session MUST NOT change a recording in progress.

### EU-0015-108 {#EU-0015-108}

On a new connection or a headphone insertion, a station's output MUST start muted and rise to its listener's mix level only after its route and limits are validated.

### EU-0015-109 {#EU-0015-109}

When a station leaves, the host MUST make its sink unavailable.

### EU-0015-110 {#EU-0015-110}

A station's sink mix revision and control map MUST stay in the session after the station leaves.

### EU-0015-111 {#EU-0015-111}

A station that reconnects MUST restore its sink's mix once it is validated again.

### EU-0015-112 {#EU-0015-112}

A headphone removal MUST mute the station's output until headphones are inserted again.

### EU-0015-114 {#EU-0015-114}

When a station leaves, the host MUST stop calculating its output.

### EU-0015-115 {#EU-0015-115}

A headphone removal MUST NOT end the station's connection.
