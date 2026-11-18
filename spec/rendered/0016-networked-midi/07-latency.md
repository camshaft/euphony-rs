## Measured delay and readiness {#latency}

A route's delay is measured end to end: capture at the device, packetization, inbound network
transit, clock uncertainty, receive buffering, dispatch, transform execution, output scheduling,
outbound transport, and the delay of the synthesizer or audio device where there is one. Clock round
trips alone can't prove a one-way delay under asymmetric traffic, so the measurement uses
timestamped loopback on the target hardware. The accepted monitoring delay comes from the
monitoring mode the musician accepts, which the system overview RFC holds. A route's transform
budget is that accepted delay less the route's other measured delays and their uncertainty
allowance. A route is live-ready where its transform budget covers the measured execution of its
transforms.

### EU-0016-118 {#EU-0016-118}

The session host MUST measure each route's delay end to end on the hardware in use, with rendering under way.

### EU-0016-119 {#EU-0016-119}

The interface MUST show each route's delay distribution, worst observed delay, jitter, loss and late-event counts.

### EU-0016-120 {#EU-0016-120}

A delay measurement MUST report its uncertainty.

### EU-0016-121 {#EU-0016-121}

A one-way delay MUST NOT be claimed from clock round trips alone.

### EU-0016-122 {#EU-0016-122}

A route's transform budget MUST be the accepted monitoring delay less the route's other measured delays and their uncertainty allowance.

### EU-0016-123 {#EU-0016-123}

A route whose transform budget doesn't cover its transforms MUST be shown as not live-ready.

### EU-0016-124 {#EU-0016-124}

A route MUST NOT be shown as live-ready before its delay is measured.

### EU-0016-125 {#EU-0016-125}

Delay results over wired and wireless networks MUST be reported separately.

### EU-0016-126 {#EU-0016-126}

A simulated result MUST NOT be reported as a measurement on hardware.
