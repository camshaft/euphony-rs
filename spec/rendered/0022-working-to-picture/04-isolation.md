## Keeping the sound safe from the picture {#isolation}

Video decoding on a computer runs apart from the work that plays and records the sound, and reads
only the session's published positions and the video's items in the local store. On the computer
that plays the audio, it runs below the priority of the audio work. A missed audio deadline is a
buffer of audio that isn't ready when the audio device needs it, as RFC 0021 measures locally.

### EU-0022-054 {#EU-0022-054}

Showing video MUST NOT cause a missed audio deadline on the computer that plays the audio.

### EU-0022-055 {#EU-0022-055}

Showing video MUST NOT interrupt a recording.

### EU-0022-056 {#EU-0022-056}

Showing video MUST NOT change the sound that the computer plays.

### EU-0022-057 {#EU-0022-057}

A failure of video decoding MUST leave recording and the audio output running.

### EU-0022-058 {#EU-0022-058}

Video decoding on the computer that plays the audio MUST count against that computer's measured capacity for audio work.

### EU-0022-059 {#EU-0022-059}

Where the computer that plays the audio misses audio deadlines while it decodes a video's original file, Euphony MUST show the scrub copy there instead.

### EU-0022-174 {#EU-0022-174}

While Euphony shows the scrub copy instead of the original file to protect the sound, it MUST tell the person on that computer that the scrub copy is shown.

### EU-0022-060 {#EU-0022-060}

A person MUST be able to move the picture from the computer that plays the audio to another of their computers.

### EU-0022-061 {#EU-0022-061}

On each supported computer, a playback and recording test with video playing MUST show no more missed audio deadlines than the same test without video.
