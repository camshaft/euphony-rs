## Connecting from any computer {#connection}

On the computer that runs the session, a client connects to a local endpoint at a location that
follows the convention of each platform. From another computer, a client connects as a paired
device or as a participant of a share link, and uses discovery on the local network only as a hint
of where a session runs. RFC 0002 states who may connect, the client session that each attachment
receives, the origin of each command, and the approvals that only the person's native interface
decides. A record of the person's interaction comes from the person's input on that interface and
counts for one action alone. The messages are the same locally and remotely; only the transport and
the delivery of display streams differ.

### EU-0003-019 {#EU-0003-019}

A record of the person's interaction MUST NOT be accepted more than once.

### EU-0003-023 {#EU-0003-023}

Discovery on the local network MUST NOT by itself admit a connection.

### EU-0003-024 {#EU-0003-024}

The client protocol MUST carry the same messages on the computer running the session and from another computer.

### EU-0003-025 {#EU-0003-025}

The location of the local endpoint MAY be configured.
