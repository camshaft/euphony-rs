## Listener outputs and listener mixes {#listener-outputs}

A sink is one listening output of a session: a device or a stream that plays audio to one listener,
such as headphones at a player's seat, a pair of speakers or a headphone station. Each sink names its
listener and states its device and stream identity, its clock domain, its channel layout and its
latency. The owner of a session assigns the controller of each sink and sets its gain limit. A sink
mix is the mix one sink plays: it can take any track of the session, and its gains, pans and
processing are its own. The main mix is the mix of the session itself. Each sink mix has a revision
of its own, so a change to one sink mix is a new revision of that mix alone. Every client that
changes a mix, the desktop interface, a phone or an assistant, sends the same commands to the
session host, which applies the same rules to each of them.

### EU-0015-001 {#EU-0015-001}

Each listening output of a session MUST be a sink that states its device and stream identity, its listener, its clock domain, its channel layout and its latency.

### EU-0015-002 {#EU-0015-002}

The host MUST NOT let anyone other than the owner assign the controller of a sink.

### EU-0015-003 {#EU-0015-003}

The host MUST NOT let anyone other than the owner set the gain limit of a sink.

### EU-0015-004 {#EU-0015-004}

A listener MUST be able to adjust the mix of their own sink.

### EU-0015-005 {#EU-0015-005}

A listener's sink mix MAY include any track of the session.

### EU-0015-006 {#EU-0015-006}

The host MUST NOT accept a command from a listener who isn't the owner that changes another listener's sink mix.

### EU-0015-007 {#EU-0015-007}

The host MUST NOT accept a command from a listener who isn't the owner that changes the main mix.

### EU-0015-008 {#EU-0015-008}

The host MUST apply the same rules to a mix command whether it comes from the desktop interface, a phone or an assistant.

### EU-0015-009 {#EU-0015-009}

Each sink MUST have a mix revision of its own.

### EU-0015-010 {#EU-0015-010}

Every gain of a sink mix MUST stay within the gain limit the owner set for that sink.

### EU-0015-011 {#EU-0015-011}

A change of one sink mix MUST NOT change the output of any other sink.

### EU-0015-012 {#EU-0015-012}

A change of one sink mix MUST NOT change the main mix.

### EU-0015-013 {#EU-0015-013}

A change of one sink mix MUST NOT change a recording in progress.

### EU-0015-014 {#EU-0015-014}

A change of one sink mix MUST NOT cause a new rendering of any output that doesn't depend on that sink mix.
