## Input capture and recording {#capture}

While a stream runs, the audio device process captures the input of its device. Recording keeps
the raw input apart from any audio rendered from the project: rendered audio can be calculated
again, and a performance can't. A take enters the project as regions of the same immutable model as
any other audio. Each recorded sample is aligned with the sample of playback it was recorded
against, so a take sounds in time with the passage it was played over. The process measures the
round-trip latency of the active device, and the musician chooses a monitoring mode after seeing
it; euphony doesn't promise a latency of its own.

### EU-0005-041 {#EU-0005-041}

Recording MUST keep the raw input it captures independently of any rendered output.

### EU-0005-042 {#EU-0005-042}

A recorded take MUST enter the project as regions of the same immutable model as other audio.

### EU-0005-043 {#EU-0005-043}

Each recorded sample MUST be aligned with the sample of playback it was recorded against.

### EU-0005-044 {#EU-0005-044}

The audio device process MUST measure the round-trip latency of the active device.

### EU-0005-045 {#EU-0005-045}

Euphony MUST show the musician the measured round-trip latency of the active device before the musician chooses a monitoring mode.

### EU-0005-046 {#EU-0005-046}

Input captured before the loss of a device MUST stay in the recording.
