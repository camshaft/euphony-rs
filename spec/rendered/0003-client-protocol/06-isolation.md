## Client failure and several clients {#isolation}

A client runs in its own process, apart from the session host and the audio process, so its failure,
a hang or the closing of its window ends only its attachment. The gestures it held end with a
recorded forced end, its momentary controls release, and the device stream, the take and every other
output go on. A client that comes back attaches again with a new generation and a fresh snapshot,
and doesn't repeat input from before. Several clients attach to one session at once, each with its
own layout, and they share one session state: one selected branch, one transport and one history.
No client becomes a second authority, and adding a kind of client doesn't add a rule to that state.
The session host admits clients and their display traffic by the measured cost of each, and under
pressure drops only display frames that a newer frame replaces.

### EU-0003-077 {#EU-0003-077}

The failure, hang or closing of a client MUST NOT stop playback.

### EU-0003-078 {#EU-0003-078}

The failure, hang or closing of a client MUST NOT stop or interrupt recording.

### EU-0003-079 {#EU-0003-079}

The failure, hang or closing of a client MUST NOT lose any recorded sample.

### EU-0003-080 {#EU-0003-080}

The failure, hang or closing of a client MUST NOT change any other output of the session.

### EU-0003-081 {#EU-0003-081}

The failure of a client MUST end only the attachment of that client.

### EU-0003-082 {#EU-0003-082}

Where an attachment ends, each gesture that it held MUST end with a recorded forced end.

### EU-0003-083 {#EU-0003-083}

Where an attachment ends, each momentary control that it held MUST release.

### EU-0003-084 {#EU-0003-084}

A client that attaches again MUST receive a new generation and a fresh snapshot.

### EU-0003-085 {#EU-0003-085}

A client that attaches again MUST NOT replay input from before its earlier attachment ended.

### EU-0003-086 {#EU-0003-086}

A message of an ended generation MUST NOT change the session.

### EU-0003-087 {#EU-0003-087}

The number of clients attached to a session MUST NOT be bounded by a fixed number.

### EU-0003-088 {#EU-0003-088}

Every client attached to a session MUST share its one selected branch and its one transport.

### EU-0003-089 {#EU-0003-089}

A client MUST NOT hold state of the session that overrides the state the session host holds.

### EU-0003-090 {#EU-0003-090}

Under load, the session host MUST NOT drop any traffic to a client other than display frames that a newer frame replaces.
