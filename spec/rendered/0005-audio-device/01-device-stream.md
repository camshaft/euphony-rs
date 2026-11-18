## The audio device and its stream {#device-stream}

The audio device process is the component of euphony that owns the hardware stream of one audio
device on a computer, the capture of its input and the live render island. It starts only once the
musician activates a device. The session host, which owns the project, sends it compiled graph
revisions and the audio to play, and the audio device process sends the session host the events of
the device. Before a stream starts, the process reads the capabilities of the device and prepares a
stream in a format the device accepts: the sample rate, the channel layout and the period come from
the device. Preparing a stream answers with the negotiated format, or with one typed failure:
unsupported format, clock unlocked, device lost or transport failure. Each buffer the process
reports carries its timestamp and the quality of its clock. An underrun or the loss of the device
reaches the session host as an event.

### EU-0005-001 {#EU-0005-001}

The audio device process MUST start a hardware stream only after the musician activates its device.

### EU-0005-002 {#EU-0005-002}

The audio device process MUST report the capabilities of a device when asked.

### EU-0005-003 {#EU-0005-003}

The audio device process MUST take the sample rate, channel layout and period of a stream from the format it negotiated with the device.

### EU-0005-004 {#EU-0005-004}

Preparing a stream MUST answer with the negotiated format or with exactly one typed failure of unsupported format, clock unlocked, device lost or transport failure.

### EU-0005-005 {#EU-0005-005}

Each buffer the audio device process reports MUST carry its timestamp and the quality of its clock.

### EU-0005-006 {#EU-0005-006}

The audio device process MUST stop a stream when the musician stops its device.

### EU-0005-007 {#EU-0005-007}

The audio device process MUST report each underrun to the session host as a typed event.

### EU-0005-008 {#EU-0005-008}

The audio device process MUST report the loss of its device to the session host as a typed event.

### EU-0005-009 {#EU-0005-009}

Reporting an event of the device MUST NOT make the callback wait.
