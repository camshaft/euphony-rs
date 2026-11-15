## Connecting and client sessions {#connection}

On the computer that runs the session, a client connects to a local endpoint that only processes of
the same operating-system user reach, at a location that follows the convention of each platform.
Being admitted to the endpoint doesn't say who the client is. At each attach the client receives a
client session, bound to the session and to the generation of that attachment, and the origin of the
client session is one of the person's native interface, an assistant or a script. The person's
native interface proves its origin with the key of the device the person paired; every other client
is an assistant or a script, whatever it says about itself. Only the native interface records the
person's own interactions, from the person's input on that interface, so an action that waits for
the person's approval waits for the person. From another computer, a client connects as a paired
device over a connection that both ends authenticate and that is encrypted, using discovery on the
local network only as a hint, and attaches as on the same computer. A participant of a share link
attaches at the level of its link, to listen, to comment or to edit. The messages are the same
locally and remotely; only the transport and the delivery of display streams differ.

### EU-0003-010 {#EU-0003-010}

The local endpoint MUST NOT admit a connection from a process of another operating-system user.

### EU-0003-011 {#EU-0003-011}

Admitting a connection MUST NOT assign it an origin.

### EU-0003-012 {#EU-0003-012}

Each attachment MUST be given a client session bound to the session and to the generation of the attachment.

### EU-0003-013 {#EU-0003-013}

The origin of each command MUST be the origin of the client session that sent it.

### EU-0003-014 {#EU-0003-014}

A claim that a client makes about its own origin MUST NOT change the origin of its client session.

### EU-0003-015 {#EU-0003-015}

A command that a client forwards, queues or replays MUST keep the origin of the client session that first sent it.

### EU-0003-016 {#EU-0003-016}

A client session MUST hold the origin of the person's native interface only where its client proves that origin with the key of a paired device.

### EU-0003-017 {#EU-0003-017}

A client session other than the person's native interface MUST NOT create a record of the person's interaction.

### EU-0003-018 {#EU-0003-018}

A decision on an approval MUST NOT be accepted from a client session other than the person's native interface.

### EU-0003-019 {#EU-0003-019}

A record of the person's interaction MUST NOT be accepted more than once.

### EU-0003-020 {#EU-0003-020}

An unpaired computer MUST NOT attach to a session except through a share link.

### EU-0003-021 {#EU-0003-021}

The connection of a client on another computer MUST be mutually authenticated and encrypted.

### EU-0003-022 {#EU-0003-022}

A participant of a share link MUST receive only the part of the session that the level of its link allows.

### EU-0003-023 {#EU-0003-023}

Discovery on the local network MUST NOT by itself admit a connection.

### EU-0003-024 {#EU-0003-024}

The client protocol MUST carry the same messages on the computer running the session and from another computer.

### EU-0003-025 {#EU-0003-025}

The location of the local endpoint MAY be configured.
