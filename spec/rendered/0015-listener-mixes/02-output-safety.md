## Alignment, monitoring delay and output protection {#output-safety}

Each sink stream is aligned to the session by the measured latency of its device and its network
path, and is resampled where it crosses from one clock domain to another. The monitoring delay of a
sink is the time from the session's timeline to the sound at the listener's ears. A listener chooses
the monitoring mode they accept for their sink after its delay is measured, and a sink whose delay is
longer than that mode stays available with its delay shown. Moving rendering to another computer
doesn't shorten a monitoring path. Output protection is the gain protection, the mute on an unsafe
transition and the hearing-safety policy of an output. The owner chooses the output limits of a sink
from the calibrated capability of its device and the needs of the session; these limits bound the
signal for safety, and are neither a permission nor a capacity limit. A performer listens to their own
instrument from the computer that records it.

### EU-0015-015 {#EU-0015-015}

The host MUST align each sink stream to the session using the measured latency of its device and its network path.

### EU-0015-016 {#EU-0015-016}

A sink stream that crosses from one clock domain to another MUST be resampled explicitly.

### EU-0015-017 {#EU-0015-017}

Each listener MUST be shown the measured monitoring delay of their sink.

### EU-0015-018 {#EU-0015-018}

A sink whose measured delay is longer than the monitoring mode its listener accepted MUST NOT be described as live monitoring.

### EU-0015-019 {#EU-0015-019}

Output protection MUST act on the computer that plays the output, without waiting on the network.

### EU-0015-020 {#EU-0015-020}

After a change of its output device, a sink MUST stay muted until its route and limits are validated.

### EU-0015-021 {#EU-0015-021}

A performer's monitoring of their own instrument MUST play from the computer that records that instrument.
