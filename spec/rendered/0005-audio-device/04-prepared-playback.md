## Playing prepared audio {#prepared-playback}

Prepared audio is audio of the project rendered ahead of playback, and it is the normal input of
playback. A passage whose prepared audio matches the current revision plays from that audio, and a
fully prepared project replays without calculating any instrument or effect. Prepared audio is
streamed from storage into buffers that are full before the callback reads them, and playback can
start at any sample of a prepared buffer. Playback moves between prepared audio and the live render
island at aligned sample positions, compensates the latency of each path, crosses between them with
a transition envelope and carries tails and instrument state across the switch. The browser
companion plays prepared audio to its output; live rendering and capture run in the native audio
device process.

### EU-0005-029 {#EU-0005-029}

A passage whose prepared audio matches the current revision MUST play from that prepared audio.

### EU-0005-030 {#EU-0005-030}

Replaying a fully prepared project MUST NOT render any of its instruments or effects during playback.

### EU-0005-031 {#EU-0005-031}

Each buffer of prepared audio MUST be filled before the callback reads it.

### EU-0005-032 {#EU-0005-032}

Playback MUST stream prepared audio from storage as it plays.

### EU-0005-033 {#EU-0005-033}

Playback MUST be able to start at any sample position of a prepared buffer.

### EU-0005-034 {#EU-0005-034}

A switch between prepared audio and the live render island MUST happen at aligned sample positions.

### EU-0005-035 {#EU-0005-035}

A switch between prepared audio and the live render island MUST compensate the latency of each path.

### EU-0005-036 {#EU-0005-036}

A switch between prepared audio and the live render island MUST cross between them with a transition envelope.

### EU-0005-037 {#EU-0005-037}

A switch between prepared audio and the live render island MUST carry tails and instrument state across the switch.

### EU-0005-038 {#EU-0005-038}

The browser companion MUST play prepared audio to its output device.

### EU-0005-039 {#EU-0005-039}

Live rendering MUST run in a native audio device process.

### EU-0005-040 {#EU-0005-040}

Input capture MUST run in a native audio device process.
