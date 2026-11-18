## Picture in step with sound {#playback}

The audio device that plays the session is the reference. It publishes the sample position it is
playing and the device time at which that sample reaches the output. At each display refresh, a
screen shows the frame whose interval holds the sample that will be heard at that refresh's
presentation time. That choice takes into account the audio output's delay, the time of the
refresh and the display's delay as the platform reports them, and a per-display offset corrects
what neither the audio device nor the display reports. The flash-and-tone test plays a video that
shows a white frame with a tone on the same frame once a second, and records a light sensor at the
screen and a microphone at the listening position together through the audio interface; it measures
the time between each flash and its tone, and from it the display's offset. Scrubbing moves the
playback position by hand while the scrubbed sound plays. Varispeed plays the timeline slower or
faster than it was recorded.

### EU-0022-041 {#EU-0022-041}

At each display refresh during playback, a screen MUST show the frame whose interval holds the sample heard at that refresh's presentation time.

### EU-0022-042 {#EU-0022-042}

The choice of frame MUST take into account the audio output's delay, the refresh time and the display's delay as the platform reports them.

### EU-0022-043 {#EU-0022-043}

Each display MUST have an offset that corrects the delay that neither the audio device nor the display reports.

### EU-0022-044 {#EU-0022-044}

A person MUST be able to measure a display's offset with the flash-and-tone test.

### EU-0022-045 {#EU-0022-045}

A person MUST be able to adjust a display's offset in fractions of a frame.

### EU-0022-046 {#EU-0022-046}

Euphony MUST show a display's measured offset beside any adjustment of it.

### EU-0022-047 {#EU-0022-047}

Euphony MUST show the measured difference between the picture and the sound on each screen.

### EU-0022-048 {#EU-0022-048}

On each supported computer and display, over the whole length of a video at each timecode rate, the flash-and-tone test MUST see every flash during the frame whose tone is heard, during playback and after each jump, loop and scrub.

### EU-0022-049 {#EU-0022-049}

Looping MUST snap the ends of a loop to frames by default.

### EU-0022-050 {#EU-0022-050}

The jump back to a loop's start MUST show the loop's first frame on the first refresh after the jump.

### EU-0022-051 {#EU-0022-051}

Scrubbing MUST show the frame for the scrub position while the scrubbed sound plays.

### EU-0022-052 {#EU-0022-052}

Varispeed MUST show the frame for each position, repeating or skipping frames as the rate needs.

### EU-0022-053 {#EU-0022-053}

Reverse playback MUST show the frame for each position.
