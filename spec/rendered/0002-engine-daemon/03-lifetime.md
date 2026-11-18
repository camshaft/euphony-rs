## Session lifetime and attached clients {#lifetime}

A client attaches to a session to control it and detaches when it leaves. Each attachment has a
generation, which a later attachment of the same client replaces. The desktop interface, a command
line, a script, an assistant's adapter, a phone, a hardware controller and a terminal interface are
all clients, and any number of them attach to one session at once, each with its own layout. They
share one session state: one selected branch and transport, one control epoch for transport and
recording, one owner for the targets of each gesture, and one undo history. A session closes when a
client closes it, when the computer shuts down, or when the person quits the last attached client
while the session is neither recording nor playing and the person didn't choose to keep it running.
The overview RFC states that the failure of an interface doesn't stop playback or recording; this
section states how the session host keeps that true. The client protocol is specified in its own
RFC.

### EU-0002-030 {#EU-0002-030}

A session MUST stay open until a client closes it, the computer shuts down, or the person quits the last client attached to it.

### EU-0002-031 {#EU-0002-031}

The session host MUST NOT close a session on a timer.

### EU-0002-032 {#EU-0002-032}

Where the person quits the last client attached to a session, the session MUST stay open while it records, plays, or is marked by the person to keep running.

### EU-0002-080 {#EU-0002-080}

When the person quits the last client attached to a session that is neither recording nor playing and that the person didn't mark to keep running, the session host MUST close the session.

### EU-0002-033 {#EU-0002-033}

A client's failure MUST NOT close a session.

### EU-0002-034 {#EU-0002-034}

A crash, a hang or a closed window of a client MUST end only that client's attachment.

### EU-0002-035 {#EU-0002-035}

When a client's attachment ends, the session host MUST end each gesture that client held with a recorded forced end.

### EU-0002-036 {#EU-0002-036}

When a client's attachment ends, the session host MUST release each momentary control that client held.

### EU-0002-037 {#EU-0002-037}

A client that attaches again after its attachment ended MUST attach in a new generation.

### EU-0002-038 {#EU-0002-038}

The session host MUST NOT apply input that a client sent in an ended generation.

### EU-0002-039 {#EU-0002-039}

The session host MUST NOT cap the number of clients attached to a session.

### EU-0002-040 {#EU-0002-040}

The session host MUST admit attachments and display traffic under an adaptive fair queue, from the measured cost of each.

### EU-0002-041 {#EU-0002-041}

Under load, the session host MUST reduce only display frames that a newer frame replaces.

### EU-0002-042 {#EU-0002-042}

All clients attached to a session MUST share one session state.

### EU-0002-043 {#EU-0002-043}

A client MUST NOT hold authority over its session's state.

### EU-0002-044 {#EU-0002-044}

A session MUST keep one undo history for all of its clients.
