## Transport, timing and loss {#transport}

Each event carries the time its host captured it and the generation of that host's clock mapping.
A host computes each peer's clock offset from every clock exchange, keeps a history of its drift,
and starts a new clock generation where the offset jumps or a peer's session restarts. The session
host maps each event's capture time onto the session's sample timeline through the engine's clock
mapping. Hosts, and the other workstations euphony plays with, carry events over an interoperable
network MIDI session protocol with its loss recovery journal. A gap is a typed event that marks
events lost or dropped. The owner marks which networks are theirs, and the system overview RFC
holds that rule. A transport verification record holds the results of peer and session binding
tests for one implementation digest, version and transport profile. Live performance is played
once: an accepted project edit and offline preparation are durable, and a late live event isn't
kept to play later.

### EU-0016-074 {#EU-0016-074}

Each event MUST carry its capture time and the clock generation of the host that captured it to the session host.

### EU-0016-075 {#EU-0016-075}

A host MUST compute each peer's clock offset from every clock exchange with that peer.

### EU-0016-076 {#EU-0016-076}

A host MUST start a new clock generation when a peer's clock offset jumps or the peer's session restarts.

### EU-0016-077 {#EU-0016-077}

The session host MUST map each event's capture time onto the session's sample timeline.

### EU-0016-078 {#EU-0016-078}

The session host MUST order events by capture time within the admitted timing window.

### EU-0016-079 {#EU-0016-079}

The session host MUST adapt its receive buffering to the measured jitter, within the accepted monitoring delay.

### EU-0016-080 {#EU-0016-080}

A clock discontinuity MUST suspend scheduled admission until the new clock mapping is verified.

### EU-0016-081 {#EU-0016-081}

A host MUST send the loss recovery journal of the network MIDI session protocol with the events it sends.

### EU-0016-082 {#EU-0016-082}

Where a peer sends a recovery journal, the receiving host MUST rebuild a lost note-off from it.

### EU-0016-083 {#EU-0016-083}

A sequence gap the recovery journal doesn't cover MUST reach the route as a gap event.

### EU-0016-084 {#EU-0016-084}

On a gap the recovery journal doesn't cover, the session host MUST reset the state the affected route owns.

### EU-0016-085 {#EU-0016-085}

A transport without verified loss recovery MUST NOT claim an uninterrupted performance.

### EU-0016-086 {#EU-0016-086}

In the default live profile, a late note-on MUST be dropped with a gap event.

### EU-0016-087 {#EU-0016-087}

A late release MUST still clear the sounding note it owns.

### EU-0016-088 {#EU-0016-088}

The session host MUST keep raw captured events, with a visible gap where events were lost.

### EU-0016-089 {#EU-0016-089}

The session host MUST NOT keep a late live event to play later.

### EU-0016-090 {#EU-0016-090}

Network MIDI MUST use an authenticated encrypted transport by default.

### EU-0016-091 {#EU-0016-091}

An unencrypted interoperable transport MUST run only on an interface whose network the owner marks as theirs.

### EU-0016-092 {#EU-0016-092}

The transport boundary MUST be enforced by the session host where events enter and leave it, and by each MIDI host's network service, not only in the user interface.

### EU-0016-093 {#EU-0016-093}

The session host MUST refuse to activate a transport unless a verification record holds passing peer and session binding tests for its exact implementation digest, version and transport profile.

### EU-0016-094 {#EU-0016-094}

A missing, failed or mismatched verification record MUST produce an unverified transport result.

### EU-0016-095 {#EU-0016-095}

A pairing or a saved route MUST NOT bypass the transport verification.

### EU-0016-096 {#EU-0016-096}

The session host MUST check each packet it accepts against the current peer and session generation.

### EU-0016-097 {#EU-0016-097}

A passing verification record MUST NOT grant access.

### EU-0016-127 {#EU-0016-127}

A host MUST read the loss recovery journal that a peer sends.

### EU-0016-128 {#EU-0016-128}

A host MUST keep a history of each peer's clock drift from successive clock exchanges.

### EU-0016-129 {#EU-0016-129}

A missing, failed or mismatched verification record MUST raise an event that says why the transport is unverified.
