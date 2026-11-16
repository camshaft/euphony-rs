## The capture session {#capture-session}

A capture measures one external device or chain: an effect chain wired between outputs and inputs
of an audio interface, or a synthesizer that receives instrument control messages and returns audio
to an input. A capture runs only on the computer whose interface the gear is wired to, in a capture
session that the owner approves before any output. The approval names the device, the output and
input channels, the sample rate, the clock source, the calibration and the highest output level the
session may send. The audio engine enforces the approved route and level itself, not a mixer of the
interface. During a session, input monitoring is off and every other output of the session is
muted. A session stops on a change of its conditions, on signs of feedback, on clipping, on a
missing return, on the musician's stop and on the loss of the client that controls it.

### EU-0010-017 {#EU-0010-017}

A capture MUST run on the computer whose audio interface the gear is wired to.

### EU-0010-018 {#EU-0010-018}

A capture session MUST begin only after the owner approves it.

### EU-0010-019 {#EU-0010-019}

The approval of a capture session MUST name the device, the output and input channels, the sample rate, the clock source, the calibration and the highest output level.

### EU-0010-020 {#EU-0010-020}

A capture session MUST NOT emit to an output channel that its approval does not name.

### EU-0010-021 {#EU-0010-021}

A capture session MUST NOT read from an input channel that its approval does not name.

### EU-0010-022 {#EU-0010-022}

A capture session MUST NOT send a signal above the highest output level that its approval names.

### EU-0010-023 {#EU-0010-023}

The audio engine MUST enforce the route and the level bound of a capture session itself, whatever the mixer of the audio interface is set to.

### EU-0010-024 {#EU-0010-024}

A capture session MUST mute every output of the session that its approval does not name.

### EU-0010-025 {#EU-0010-025}

A capture session MUST keep input monitoring off for its whole duration.

### EU-0010-026 {#EU-0010-026}

A capture session MUST stop emission before its next segment when its device, channels, sample rate, clock source or channel map changes.

### EU-0010-027 {#EU-0010-027}

A capture session MUST stop emission when its calibration is older than the session.

### EU-0010-028 {#EU-0010-028}

A capture session MUST mute its output within one device period on signs of feedback, clipping or a missing return.

### EU-0010-029 {#EU-0010-029}

A capture session MUST mark a segment unusable when it mutes its output for feedback, clipping or a missing return.

### EU-0010-030 {#EU-0010-030}

A capture session MUST stop on the musician's stop.

### EU-0010-031 {#EU-0010-031}

A capture session MUST stop on the loss of the client that controls it.

### EU-0010-032 {#EU-0010-032}

After a stop for a change of its conditions, a capture session MUST have a new calibration and a new approval before it emits again.

### EU-0010-209 {#EU-0010-209}

A capture session MUST stop emission when the owner unpairs its computer during a take.

### EU-0010-211 {#EU-0010-211}

A capture session MUST mark a take incomplete when it stops emission for an unpairing or a reservation conflict.
