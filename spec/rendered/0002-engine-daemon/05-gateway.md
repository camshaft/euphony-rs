## The command gateway {#gateway}

Every persistent action on a session is a typed command, and every command reaches the gateway of
the session host, whichever client sent it. On every action the gateway checks that the caller is
the owner, a paired device, an assistant the owner connected or a link participant whose level
allows the action, and checks the expected revision. The approval list holds the actions whose
effect the owner approves before it applies, where anyone other than the owner starts them: pairing,
a share link or an edit admission, an export outside the owner's computers, pruning or purge,
private data sent to a provider, plain network audio, a capture session, and telemetry sending or a
problem report. For such an action the gateway records an approval proposal of its exact effect. An
edit that can be undone doesn't wait for an approval. The commands themselves are specified in the
RFC of the project model.

### EU-0002-057 {#EU-0002-057}

The session host MUST check every command through one gateway, whichever client sent it.

### EU-0002-058 {#EU-0002-058}

The gateway MUST refuse a command whose origin is not the owner, a paired device, an assistant the owner connected, or a link participant whose level allows the command.

### EU-0002-059 {#EU-0002-059}

The gateway MUST check each command's expected revision before applying it.

### EU-0002-060 {#EU-0002-060}

Where a command's expected revision is not current, the gateway MUST answer with a conflict and a current view the caller may see.

### EU-0002-061 {#EU-0002-061}

The gateway MUST NOT recompute a stale command for its caller.

### EU-0002-062 {#EU-0002-062}

A successful answer to a command MUST identify the committed revision or the asynchronous job the command started.

### EU-0002-063 {#EU-0002-063}

The gateway MUST NOT report a pending render as a completed export.

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
