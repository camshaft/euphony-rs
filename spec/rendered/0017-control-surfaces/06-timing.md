## Timing, capacity and readiness {#timing}

Controls fall in four classes. Expressive controls are faders, knobs, XY pads and expressive
parameter sources, whose response is measured from contact to applied sample, through device
sampling, uplink, command admission, local rendering and output delay. Transport commands are play,
stop, locate and recording, which follow the session's transport control and its shared transport
mode. Patch edits and layout changes need responsive previews and an explicit commit or activation
state. Meters and remote playheads need display updates fresh enough for the surface's negotiated
display capability. Every other control counts as ordinary editing. The acceptable delay for each
use stays with the musician after measurement, and the readiness view of RFC 0001 shows it. Input
timestamps can't create missing samples or remove network delay. Surface traffic uses the shared
adaptive fair queue, with control, capture and replaceable display traffic each in its own order.
Where a remote path fails, local hardware stays the path for timing-sensitive playing.

### EU-0017-103 {#EU-0017-103}

The system MUST measure the response and jitter from contact to applied sample of expressive controls for each device and path.

### EU-0017-104 {#EU-0017-104}

A surface MUST mark a value it shows on contact as predicted until the session host acknowledges it.

### EU-0017-105 {#EU-0017-105}

A predicted value MUST NOT change the audio.

### EU-0017-106 {#EU-0017-106}

A surface MUST show a refused value as refused instead of its prediction.

### EU-0017-107 {#EU-0017-107}

A sample-capable target MUST apply a timestamped control event at its admitted sample offset.

### EU-0017-108 {#EU-0017-108}

A block-only target MUST report the interval at which it applies control events.

### EU-0017-109 {#EU-0017-109}

A late ordinary adjustment MUST apply at the next admitted boundary with its lateness reported.

### EU-0017-110 {#EU-0017-110}

A scheduled sample-accurate performance that misses its deadline MUST be refused, with the gap recorded and its original events kept for inspection.

### EU-0017-111 {#EU-0017-111}

The session host MUST acknowledge play, stop, locate and recording commands with the transport generation and the position at which they applied.

### EU-0017-112 {#EU-0017-112}

Stop MUST stay responsive on the local host while a remote surface is unavailable.

### EU-0017-113 {#EU-0017-113}

A remote stop that hasn't been acknowledged MUST be shown as unconfirmed until the actual state arrives.

### EU-0017-114 {#EU-0017-114}

A quantized start MUST be scheduled in session time.

### EU-0017-115 {#EU-0017-115}

A recording control MUST NOT become a durable offline command.

### EU-0017-116 {#EU-0017-116}

A patch edit or a layout change MUST show its preview and its commit or activation state.

### EU-0017-117 {#EU-0017-117}

While a changed live graph is pending, the previous graph MUST keep playing with the pending state shown.

### EU-0017-118 {#EU-0017-118}

A meter or a remote playhead MUST report the age of what it shows.

### EU-0017-119 {#EU-0017-119}

Meter delivery MUST NOT decide the automation recording rate.

### EU-0017-120 {#EU-0017-120}

The readiness view MUST show control-to-sound latency, acknowledgment latency, clock uncertainty and display age, separated by device, path and control class.

### EU-0017-121 {#EU-0017-121}

A remote path that fails its expressive-control mode MUST stay available for editing and transport, with the limitation shown.

### EU-0017-122 {#EU-0017-122}

The system MUST NOT claim sample accuracy for a network gesture without measured clock and delivery bounds.

### EU-0017-123 {#EU-0017-123}

The system MUST order control, capture and replaceable display traffic of surfaces independently.

### EU-0017-124 {#EU-0017-124}

The system MUST NOT cap surfaces at a fixed surface count or event rate.

### EU-0017-125 {#EU-0017-125}

Ending a gesture, cancelling one and safety commands MUST stay serviceable while display traffic is overloaded.

### EU-0017-126 {#EU-0017-126}

Under congestion, the system MUST reduce only replaceable display traffic.

### EU-0017-127 {#EU-0017-127}

The system MUST NOT coalesce anything other than superseded, unaccepted preview samples and display frames.

### EU-0017-128 {#EU-0017-128}

A capacity failure while performance is being captured MUST end its admission visibly and record the gap.

### EU-0017-129 {#EU-0017-129}

The real-time audio path MUST NOT perform network, layout or journal work.
