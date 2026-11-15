## Routes, sources and targets {#routes}

A route connects one source to one target control. A source is an audio tap of a track, taken
before its effects, after its effects or after its fader; a drawn automation lane, read before
modulation; the effective value of a control, which is a separate source with its own dependency;
note and controller events, selected by port, channel and event type; or a generated control
output, such as a low-frequency oscillator or an envelope follower. A target is a sound control of
a processor of any track or effect. A route records its source and tap, its target processor and
control, its conversion and the version of that conversion, its event selector, polarity, depth,
offset, output range, curve, requested rate, smoothing, delay, scope, aggregation and fallback.
Each processor declares its controls in a registry of parameters, which is where a route finds
what a control accepts. Ordinary sidechains and curve editing stay available beside routes.

### EU-0012-001 {#EU-0012-001}

A musician MUST be able to connect a source of any track to any modulatable sound control of any track or effect.

### EU-0012-002 {#EU-0012-002}

Each route MUST be stored in the project as typed data with an identifier that it keeps for its life.

### EU-0012-003 {#EU-0012-003}

A route MUST refer to its source and its target by their stable identifiers, never by a track's label.

### EU-0012-004 {#EU-0012-004}

Each track MUST offer its audio before its effects, after its effects and after its fader as sources.

### EU-0012-005 {#EU-0012-005}

A route that reads an automation lane MUST read the lane's value before modulation.

### EU-0012-006 {#EU-0012-006}

Reading the effective value of a control MUST be a source distinct from reading its automation lane, with its own dependency.

### EU-0012-007 {#EU-0012-007}

A source of note or controller events MUST select its events by port, channel and event type.

### EU-0012-008 {#EU-0012-008}

Following an audio signal's envelope and using the raw signal MUST be distinct conversions.

### EU-0012-009 {#EU-0012-009}

A conversion from a signal of more than one channel to a control value MUST state how it selects or mixes the channels.

### EU-0012-010 {#EU-0012-010}

A conversion from notes to a control value MUST state how it normalizes values and how it combines notes, including their release and sustain.

### EU-0012-011 {#EU-0012-011}

Each built-in processor MUST declare every public control in the parameter registry, with its unit, legal domain, mapping from normalized to physical value, value kind, update capability and effect on state.

### EU-0012-012 {#EU-0012-012}

A continuous control MUST accept numeric modulation.

### EU-0012-013 {#EU-0012-013}

A switch or a choice among values MUST accept modulation only through an explicit threshold with hysteresis or through index selection.

### EU-0012-014 {#EU-0012-014}

A control that selects a resource MUST accept modulation only as a typed choice among prepared resources, applied as one control change.

### EU-0012-015 {#EU-0012-015}

Where a route needs a conversion its target doesn't support, the system MUST explain why before the route is activated.

### EU-0012-016 {#EU-0012-016}

Muting a track's audible output MUST NOT mute the sources it offers before its fader.

### EU-0012-017 {#EU-0012-017}

A source taken after the fader MUST follow its track's mute and gain.

### EU-0012-018 {#EU-0012-018}

Soloing a track to audition it MUST NOT change the saved routes.

### EU-0012-019 {#EU-0012-019}

A source whose live input is missing MUST be marked unavailable, with a warning the musician sees.

### EU-0012-020 {#EU-0012-020}

The route of an unavailable source MUST contribute what its stored fallback selects: the neutral contribution, or the last value it held.

### EU-0012-021 {#EU-0012-021}

The fallback of a new route MUST be a contribution of zero, offset included.

### EU-0012-022 {#EU-0012-022}

An offline render MUST NOT render a range for which a live source's input wasn't captured.
