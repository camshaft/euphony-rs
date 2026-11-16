## Phone controllers {#phones}

The phone controller is a native app for phones and tablets, built on the session's client
protocol with controls native to the platform. A phone is discovered, paired and reported as the
pairing section of this RFC states, and attaches to a selected session through the session's control
service, on the local network or through the authenticated connection between networks otherwise.
Its default layout holds transport, cue and recording actions alone; the layouts and panels section
of this RFC adds optional panels. Arrangement editing, program authoring, processor mapping, and
pairing and link management stay on the desktop interface and with assistants. A phone is a
controller, never a render worker or a microphone the session takes from automatically.

### EU-0017-032 {#EU-0017-032}

The system MUST offer a native phone controller app for each mobile platform the project's configuration lists.

### EU-0017-033 {#EU-0017-033}

The phone controller MUST offer play, pause, stop, locating a marker, toggling the loop, reading the transport and recording state, and adjusting the click and its assigned cue levels.

### EU-0017-034 {#EU-0017-034}

The phone controller on the owner's paired phone MUST offer arming a track and starting a recording.

### EU-0017-035 {#EU-0017-035}

A collaborator's phone joined by link MUST change only its own listener mix and, where the session shares its transport, playback.

### EU-0017-036 {#EU-0017-036}

Every recording and mix control on a phone MUST show the session and the output it acts on.

### EU-0017-037 {#EU-0017-037}

The phone controller MUST NOT offer pairing or link management.

### EU-0017-153 {#EU-0017-153}

The phone controller MUST NOT offer arrangement editing, program authoring or processor mapping.

### EU-0017-038 {#EU-0017-038}

The reduced set of the phone controller MUST NOT reduce what the assistant interface offers.

### EU-0017-039 {#EU-0017-039}

A disconnected phone controller MUST show its controls as unavailable.

### EU-0017-040 {#EU-0017-040}

Locking a device or suspending its app MUST NOT leave a gesture changing gain without end.
