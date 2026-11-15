## The command gateway {#gateway}

Every persistent action on a session is a typed command, and every command reaches the gateway of
the session host, whichever client sent it. On every action the gateway checks that the caller is
the owner, a paired device, an assistant the owner connected, a script of the owner's account or a
link participant whose level allows the action, and checks the expected revision. A caller reads the
session through revisioned queries, sends several commands as one transaction, retries a command
with its idempotency key, cancels an operation it started, and follows changes through an event
subscription that resumes from a cursor. Each query answers one page, and the caller reads the
rest through the paged reads of the RFC of the project model. A collaborator's peer receives only
the objects of the projects its links reach. The approval list holds the actions whose effect the owner approves
before it applies, where anyone other than the owner starts them: pairing, a share link or an edit
admission, an export outside the owner's computers, pruning or purge, private data sent to a
provider, plain network audio, a capture session, and telemetry sending or a problem report. For
such an action the gateway records an approval proposal of its exact effect. An edit that can be
undone doesn't wait for an approval. The commands themselves are specified in the RFC of the project
model; the generation of the assistants' tools and the interface bindings from the command
definitions, the discovery of supported schema versions and the session-view interface in the RFC of
the client protocol; the parity of an assistant with the owner in the overview RFC; and the
permission prompts a browser asks a person for in the RFC of the desktop interface and the browser
companion.

### EU-0002-084 {#EU-0002-084}

Every persistent action on a session MUST be a typed command.

### EU-0002-057 {#EU-0002-057}

The session host MUST check every command through one gateway, whichever client sent it.

### EU-0002-058 {#EU-0002-058}

The gateway MUST refuse a command whose origin is not the owner, a paired device, an assistant the owner connected, a script of the owner's account, or a link participant whose level allows the command.

### EU-0002-059 {#EU-0002-059}

The gateway MUST check each command's expected revision before applying it.

### EU-0002-060 {#EU-0002-060}

Where a command's expected revision is not current, the gateway MUST answer with a conflict that names the current head revision.

### EU-0002-083 {#EU-0002-083}

A conflict answer MUST hold the current view, as far as the caller may see it, of only the parts of the project the command touches.

### EU-0002-061 {#EU-0002-061}

The gateway MUST NOT recompute a stale command for its caller.

### EU-0002-062 {#EU-0002-062}

A successful answer to a command MUST identify the committed revision or the asynchronous job the command started.

### EU-0002-063 {#EU-0002-063}

The gateway MUST NOT report a pending render as a completed export.

### EU-0002-085 {#EU-0002-085}

The session host MUST answer each query with the revision the answer reflects.

### EU-0002-086 {#EU-0002-086}

The session host MUST apply a batch of commands as one transaction against one expected revision.

### EU-0002-087 {#EU-0002-087}

The session host MUST NOT apply a command again when it arrives with an idempotency key that the session host already applied for the same origin.

### EU-0002-088 {#EU-0002-088}

An undo MUST revert a committed transaction as one step.

### EU-0002-089 {#EU-0002-089}

The session host MUST let the caller that started an operation cancel it.

### EU-0002-090 {#EU-0002-090}

An event subscription MUST resume from its durable cursor after a disconnection.

### EU-0002-091 {#EU-0002-091}

The session host MUST serve a collaborator's peer the objects of only the projects that peer's links reach.

### EU-0002-092 {#EU-0002-092}

The session host MUST NOT serve a peer the bytes of an object from a project its links don't reach, whatever digest the peer presents.

### EU-0002-093 {#EU-0002-093}

The session host MUST NOT reveal to a peer whether a project its links don't reach exists.

### EU-0002-064 {#EU-0002-064}

Where anyone other than the owner starts an action of the approval list, the gateway MUST record an approval proposal of its exact effect before applying the action.

### EU-0002-065 {#EU-0002-065}

An approval proposal MUST show its recipient, its destination, its affected assets and its revision.

### EU-0002-066 {#EU-0002-066}

The gateway MUST apply an approved action only where its effect still matches the effect the proposal described.

### EU-0002-067 {#EU-0002-067}

The gateway MUST NOT require an approval for an edit that can be undone.

### EU-0002-068 {#EU-0002-068}

The session's history MUST record the proposer and the approver of each approved action separately.
