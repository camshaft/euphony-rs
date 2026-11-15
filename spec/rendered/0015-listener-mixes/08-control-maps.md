## Control maps, encoders and lights {#control-maps}

A control map is a versioned record that binds each encoder, switch and light group of one station
to a target in its listener's sink. An encoder targets a parameter of that sink mix: a source send
level, a source pan, the click level, the overall level of the sink or its mute. Each encoder binding
sets its range within the owner's bounds, its curve, such as a decibel taper, its step size and its
acceleration. A switch targets a mute, a cue solo or a change of bank, which lets a few encoders
reach more sources in turn. A light group targets a meter or a value: the output level of the sink,
the level of a source, the value of an encoder's bound parameter, clip indication or mute state, each
with its display mode. A person sets a map with the same command from the desktop mixer, a phone or
an assistant, or on the station by a learn gesture: holding an encoder switch, then touching a
control of that listener's sink in the desktop mixer. Maps are kept in the project's session
configuration, keyed by station identity and listener, so a station paired to the same listener in
another project has its own map there, and each station has a default map that seeds its map in a
new project. An encoder turn is a relative change that carries the sink's control epoch, the map
revision, a sequence number and an idempotency identity. The host combines the turns it receives
into the sink's mix revision, ramps the gain to avoid audible steps, and acknowledges the resulting
value.

### EU-0015-084 {#EU-0015-084}

A control map MUST bind the encoders, switches and light groups of one station only to targets in that station's sink.

### EU-0015-085 {#EU-0015-085}

Each encoder binding MUST set a range within the owner's bounds, a curve, a step size and an acceleration.

### EU-0015-086 {#EU-0015-086}

A person MUST be able to set a station's control map from the desktop interface, a phone or an assistant with the same command.

### EU-0015-087 {#EU-0015-087}

A person MAY bind a station control by a learn gesture on the station.

### EU-0015-088 {#EU-0015-088}

The host MUST refuse a control map binding to a control the station's hardware description doesn't list.

### EU-0015-089 {#EU-0015-089}

The host MUST refuse a control map binding to a parameter outside the station's sink.

### EU-0015-090 {#EU-0015-090}

The host MUST check each command a control map produces again when the command is used.

### EU-0015-091 {#EU-0015-091}

Control maps MUST be kept in the project's session configuration, keyed by station identity and listener.

### EU-0015-092 {#EU-0015-092}

Each station MUST have a default control map that seeds its map in a new project.

### EU-0015-093 {#EU-0015-093}

A station MUST fetch the current control map on each reconnection.

### EU-0015-094 {#EU-0015-094}

A station MUST receive each later change of its control map as an event, without polling.

### EU-0015-095 {#EU-0015-095}

An encoder turn MUST carry the sink's control epoch, the map revision, a sequence number and an idempotency identity.

### EU-0015-096 {#EU-0015-096}

The host MUST apply each encoder turn once, however often it arrives and in whatever order.

### EU-0015-097 {#EU-0015-097}

The host MUST refuse an encoder turn that carries a stale control epoch.

### EU-0015-098 {#EU-0015-098}

The host MUST ramp each gain change of a sink mix so that it doesn't make an audible step.

### EU-0015-099 {#EU-0015-099}

The host MUST acknowledge the value each encoder turn results in.

### EU-0015-100 {#EU-0015-100}

A station's lights MUST show the acknowledged value of each bound parameter.

### EU-0015-101 {#EU-0015-101}

A station MUST drop the encoder turns made while it is disconnected.

### EU-0015-102 {#EU-0015-102}

A station MUST calculate its output-level meters from the audio it plays.

### EU-0015-103 {#EU-0015-103}

The host MUST publish the source meters of a station's sink mix as events at the refresh rate the station reports for its lights.

### EU-0015-104 {#EU-0015-104}

A continuous encoder turn MUST be one grouped change in the project's history.
