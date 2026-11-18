## Loss, reconnection and threats {#recovery}

A surface is lost when it detaches, is suspended, loses its transport, sees a change of authority or
is unpaired. The session host and the audio device go on with the recording, the other surfaces
and the other listener mixes. Reconnection begins a new generation of the surface's attachment, and
independent offline editing on a branch of its own stays as RFC 0013 states it. The threats this
section answers extend the threat model of RFC 0001 with surfaces: an unpaired surface that reads
the session, a hostile layout that embeds executable input, replayed gestures that change recording
or gain, and a busy client that disrupts audio. Its boundaries are discovery to pairing, client
input to the session host's validation, the authorized projection to the device, and compiled
control to the audio device. A device that was paired keeps what it displayed, and unpairing can't
erase that copy; device unlock, physical privacy and trusted delivery of the app are assumptions
that the pairing screen shows.

### EU-0017-130 {#EU-0017-130}

When a surface is lost, the session host MUST invalidate its gesture tokens.

### EU-0017-164 {#EU-0017-164}

When a surface is lost, the session host MUST close each gesture the surface owns.

### EU-0017-131 {#EU-0017-131}

The loss of a surface MUST NOT stop a recording, another surface or another listener mix.

### EU-0017-132 {#EU-0017-132}

The loss of a surface MUST keep its acknowledged edits and its saved layouts.

### EU-0017-133 {#EU-0017-133}

The loss of the main host MUST make its remote controls unavailable.

### EU-0017-134 {#EU-0017-134}

A surface MUST NOT make itself the recorder or fork the active session when the main host is lost.

### EU-0017-135 {#EU-0017-135}

A reconnection MUST begin with a fresh generation, an authorized snapshot and a validation of the layout before the surface enables its controls.

### EU-0017-136 {#EU-0017-136}

A reconnected surface MUST resume changes only from a verified compatible cursor.

### EU-0017-165 {#EU-0017-165}

A reconnected surface whose cursor isn't compatible MUST replace its local view with a fresh authorized snapshot.

### EU-0017-137 {#EU-0017-137}

Control input made before a surface disconnected MUST NOT take effect after it reconnects.

### EU-0017-138 {#EU-0017-138}

The session host MUST reject a message of an earlier generation, including one that arrives after reconnection.

### EU-0017-139 {#EU-0017-139}

A malformed or unauthorized command MUST NOT become accepted durable work.

### EU-0017-140 {#EU-0017-140}

The adapter and the audio device MUST keep control values within the device's and the output's ranges.

### EU-0017-141 {#EU-0017-141}

Contention among the surfaces of one session MUST NOT serialize unrelated projects.
