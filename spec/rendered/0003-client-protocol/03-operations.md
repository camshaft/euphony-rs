## Session operations, commands and state {#operations}

The session operations list the sessions of a computer, open a project, attach to a session, detach
and close it. Each attachment has a generation, and a new attach starts a new one. A command is one
typed action on the session: it carries the revision of the session it was made against, an
idempotency key that identifies it across a resend, and the transaction group it commits with. The
session host answers it with the revision it committed or with a typed refusal, and accepts it only
once it has decoded, validated and recorded it. Each refusal has one kind. A command whose expected
revision isn't the current revision of the session is refused as a conflict that names the current
head revision, as RFC 0002 states, and that conflict is the only refusal of a stale revision. A
command from an origin that may not take it is denied, whatever its expected revision. A command
that names an object the session doesn't hold, or one outside the part of the session the client
may see, has an unknown target, so the kind of the refusal doesn't tell the client whether the
object exists. A command that needs a capability the session host doesn't serve has an unsupported
capability. A command that decodes but fails validation, such as one that sets a value outside the
range its target takes, is an invalid command; a command on an object outside the part of the
session the client may see has an unknown target, and is never refused as an invalid command. The
state of the session reaches a client as a snapshot of the part the client may see and then as an
ordered stream of deltas, each at a revision, which the client resumes from a cursor. A gesture,
such as a fader drag, is a token that begins, is updated in sequence with the
time each update was captured, and ends or is cancelled. Undo and redo are commands on the one
history of the session, and RFC 0002 states how the session host keeps that
history, applies each command once and resumes a subscription from its cursor. The state of a view
has shared parts, which belong to the session, and transient parts, such as scrolling, which stay
with each client. A client draws the effect of its own command at once, as a predicted value, until
the command is acknowledged or refused.

### EU-0003-026 {#EU-0003-026}

The client protocol MUST offer the session operations list, open, attach, detach and close.

### EU-0003-027 {#EU-0003-027}

An attach MUST return the generation of the session, its control epoch, a snapshot of the part of the session the client may see and a subscription cursor.

### EU-0003-028 {#EU-0003-028}

Each command MUST carry the revision it expects, an idempotency key and its transaction group.

### EU-0003-029 {#EU-0003-029}

Each command MUST be answered with the revision it committed or with a typed refusal.

### EU-0003-030 {#EU-0003-030}

A refusal MUST state its kind as one of denied, conflict, unknown target, unsupported capability and invalid command.

### EU-0003-031 {#EU-0003-031}

A command MUST NOT be acknowledged before the session host has validated it and recorded it in the history of the session.

### EU-0003-032 {#EU-0003-032}

A client MUST learn the outcome of an unacknowledged command from its idempotency key before it sends that command again.

### EU-0003-034 {#EU-0003-034}

The state of a session MUST reach a client as a snapshot followed by a stream of deltas, each at a revision of the session.

### EU-0003-036 {#EU-0003-036}

A client MUST NOT apply part of a delta.

### EU-0003-037 {#EU-0003-037}

A client that cannot apply a delta MUST ask for a fresh snapshot.

### EU-0003-038 {#EU-0003-038}

A gesture MUST be carried as a token with begin, update, end and cancel operations.

### EU-0003-039 {#EU-0003-039}

Each update of a gesture MUST carry its sequence and its capture time.

### EU-0003-040 {#EU-0003-040}

Two updates of one gesture MUST NOT be applied out of their sequence.

### EU-0003-042 {#EU-0003-042}

Undo MUST undo a gesture as one step.

### EU-0003-043 {#EU-0003-043}

The shared parts of the state of each view, such as the selected track, the follow mode of the playhead and named screensets, MUST be part of the state of the session.

### EU-0003-044 {#EU-0003-044}

The transient view state of a client, such as its scroll, zoom and pointer position, MUST NOT reach another client.

### EU-0003-045 {#EU-0003-045}

A client MAY follow or ignore the shared view state, as its configuration sets.

### EU-0003-046 {#EU-0003-046}

A predicted value MUST NOT change audio.

### EU-0003-047 {#EU-0003-047}

A predicted value MUST NOT hide a refusal of its command.

### EU-0003-048 {#EU-0003-048}

A client MUST drop or revert a predicted value when its command is acknowledged or refused.
