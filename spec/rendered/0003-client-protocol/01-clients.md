## Clients and the one protocol {#clients}

A client is any program that shows or changes a session without being part of the engine: the
desktop interface, with any of its layouts and themes, the terminal interface, the command line, a
script, the assistant adapter, the phone controllers, the browser companion, the headphone stations
and the control surfaces. The engine is the set of processes that run a session on the person's
computers, among them the session host, which holds the state, the history and the transport of the
session, and the audio process, which runs the device stream and recording. Every client speaks one
versioned client protocol with the session host, the same for every kind of client, whether it runs
on the computer that runs the session or on another of the person's computers. The protocol is
defined once, from the definitions of the commands of the session, and no client adds a command of
its own. The assistant adapter is a client in a process of its own, so that assistants reach a
session through the path every interface uses.

### EU-0003-001 {#EU-0003-001}

Every client MUST reach a session only through the client protocol.

### EU-0003-002 {#EU-0003-002}

The client protocol MUST be the same for every kind of client.

### EU-0003-003 {#EU-0003-003}

The client protocol MUST carry every command of the session.

### EU-0003-004 {#EU-0003-004}

A client MUST NOT define a command of its own.

### EU-0003-005 {#EU-0003-005}

A command MUST have the same validation and the same effect on the session whichever client sends it.

### EU-0003-006 {#EU-0003-006}

Each action that a person can take on a session from the desktop interface MUST be available from the command line.

### EU-0003-007 {#EU-0003-007}

Each action that a person can take on a session MUST be available to an authorized assistant through the assistant adapter.

### EU-0003-008 {#EU-0003-008}

A client on another of the person's computers MUST be able to take every action that a client on the computer running the session can take.

### EU-0003-009 {#EU-0003-009}

The same commands sent from the desktop interface, the command line and the assistant adapter MUST produce the same history and the same audio.
