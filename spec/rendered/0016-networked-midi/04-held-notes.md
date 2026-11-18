## Note ownership and release {#held-notes}

The note ledger records each note-on a route emits, by input device, port, channel, occurrence of
the input note and route generation, with the exact destinations and output pitches it produced.
Repeated note-ons of one pitch, and several input pitches projected onto one output pitch, are
counted by occurrence and by destination, so that one release stops only the note it belongs to.
Sustain and other held controllers are channel state that the route which set them owns. A
channel-wide reset is a message that affects every note or controller of a channel, such as all
notes off. A route owns a destination channel exclusively where the channel allocation of the
session assigns that channel to it alone in the current session generation. A reset-sensitive route
is one whose recovery sends a channel-wide reset. An instrument is reachable while the transport to
it carries events.

### EU-0016-054 {#EU-0016-054}

The session host MUST record each note-on a route emits in the note ledger with the exact destinations and pitches it produced.

### EU-0016-055 {#EU-0016-055}

A note-off MUST release the destinations and pitches its note-on produced, whatever key, route or transform edit came between them.

### EU-0016-056 {#EU-0016-056}

A note-on with zero velocity MUST be read as a release.

### EU-0016-057 {#EU-0016-057}

A release MUST NOT stop a note that another note-on still holds.

### EU-0016-058 {#EU-0016-058}

A route MUST own the sustain and other held controller state it sets on a channel.

### EU-0016-059 {#EU-0016-059}

Before it activates a route's new generation, the session host MUST release each note and controller the old generation owns.

### EU-0016-060 {#EU-0016-060}

The session host MUST discard the pending events of a route generation it replaces.

### EU-0016-061 {#EU-0016-061}

When a port is removed, its transport fails, its host is lost or its host is unpaired, the session host MUST mark each route over it unavailable.

### EU-0016-062 {#EU-0016-062}

When a route becomes unavailable, the session host MUST release each note the route owns wherever its instrument is still reachable.

### EU-0016-063 {#EU-0016-063}

The loss of a port, a transport or a host MUST NOT stop other routes, audio playback or capture.

### EU-0016-064 {#EU-0016-064}

The output adapter MUST refuse each channel-wide reset from a route that doesn't own the destination channel exclusively in the current session generation.

### EU-0016-065 {#EU-0016-065}

The output adapter MUST emit a typed refusal event for each channel-wide reset it refuses.

### EU-0016-066 {#EU-0016-066}

The output adapter MUST read the channel allocation at the moment it emits a channel-wide reset, whichever transform or recovery path requested it.

### EU-0016-067 {#EU-0016-067}

The session host MUST give each reset-sensitive route an exclusive destination channel or refuse the route.

### EU-0016-068 {#EU-0016-068}

Where a remote synthesizer is unreachable, the session host MUST record that its sounding notes can't be cleared remotely.

### EU-0016-069 {#EU-0016-069}

On reconnection, the session host MUST clear the remote note and controller state its routes own before it admits new notes.

### EU-0016-070 {#EU-0016-070}

A reconnection MUST start a new session generation.

### EU-0016-071 {#EU-0016-071}

The session host MUST NOT replay a buffered performance or controller gesture after a reconnection.

### EU-0016-072 {#EU-0016-072}

A host MUST send, on teardown, session timeout and source loss, a note-off for each note-on it forwarded without one.

### EU-0016-073 {#EU-0016-073}

A MIDI host MUST NOT send a channel-wide reset.
