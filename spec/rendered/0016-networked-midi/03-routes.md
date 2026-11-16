## Routes {#routes}

A route is a record of the project. It holds an identity and a revision; a device binding and a
source port; a selector of channels and event classes; the ordered versions of its transforms and
their parameters; a destination, which is a track, an instrument, an external port or a declared
control target; an output channel mapping; a timing profile; and the execution site and generation
of its transforms. An input route records raw events, plays an instrument or maps declared
controller values onto parameters the controller is bound to. An output route plays an external
synthesizer. The operations on routes are to list and subscribe to devices; to preview, set and
remove a route; to compile, preview and attach a transform; to activate and suspend a route; and
to reset the notes a route owns. Each mutation answers with a committed revision, or with one of
the refusals denied, stale revision, ambiguous device, absent port, unsupported format, invalid
cycle, invalid program and insufficient timing readiness. A route edit is a musical edit of the
project. The membership of a session isn't music, and a merge of music never changes it. Device
messages are data: a control change, a system exclusive message or a program change plays or
records, and changes a parameter only through a controller bound to that parameter.

### EU-0016-027 {#EU-0016-027}

Each route MUST be a typed record of the project with an identity and a revision.

### EU-0016-028 {#EU-0016-028}

Each route mutation MUST carry its actor, project, expected revision and idempotency identity.

### EU-0016-029 {#EU-0016-029}

Each route mutation MUST answer with a committed revision or a typed refusal.

### EU-0016-030 {#EU-0016-030}

A route mutation that repeats the idempotency identity of an earlier mutation MUST answer as that mutation answered, without changing the project again.

### EU-0016-031 {#EU-0016-031}

The command gateway MUST refuse a route mutation whose expected revision isn't the route's current revision.

### EU-0016-032 {#EU-0016-032}

The command gateway MUST refuse a route whose source or destination has the wrong direction.

### EU-0016-033 {#EU-0016-033}

The command gateway MUST refuse a route made over a port description that isn't current.

### EU-0016-034 {#EU-0016-034}

The command gateway MUST refuse a route whose event encoding its port doesn't support.

### EU-0016-035 {#EU-0016-035}

The command gateway MUST refuse a route whose destination doesn't exist.

### EU-0016-036 {#EU-0016-036}

The command gateway MUST refuse a route mutation from a caller the trust model doesn't authorize for the project.

### EU-0016-037 {#EU-0016-037}

The command gateway MUST refuse a route that closes a routing cycle the route doesn't declare.

### EU-0016-038 {#EU-0016-038}

A declared routing loop MUST carry an explicit delay and a tested feedback policy.

### EU-0016-039 {#EU-0016-039}

The desktop interface, each phone client that supports routes and each assistant MUST read the same revisioned view of a project's routes.

### EU-0016-040 {#EU-0016-040}

An assistant the owner connects MUST be able to perform each route and transform operation a person can perform.

### EU-0016-041 {#EU-0016-041}

A route edit MUST take part in the project's undo and merge as other musical edits do.

### EU-0016-042 {#EU-0016-042}

A merge of route edits MUST NOT change the membership of a session.

### EU-0016-043 {#EU-0016-043}

A route's acknowledgement MUST distinguish its stored intent from its active transport.

### EU-0016-044 {#EU-0016-044}

A route MUST keep each event's source channel and identity before any remapping.

### EU-0016-045 {#EU-0016-045}

A message from a device MUST NOT act as a project command or change a permission.

### EU-0016-046 {#EU-0016-046}

A controller MUST change only the target it is explicitly bound to, within that target's declared range.

### EU-0016-047 {#EU-0016-047}

A default binding MUST NOT arm recording.

### EU-0016-048 {#EU-0016-048}

A default binding MUST NOT change a pairing or a link.

### EU-0016-049 {#EU-0016-049}

A default binding MUST NOT transmit system exclusive messages.

### EU-0016-050 {#EU-0016-050}

Preparing a route MUST NOT delay audio playback.

### EU-0016-051 {#EU-0016-051}

The session host MUST activate a route's new generation atomically.

### EU-0016-052 {#EU-0016-052}

The output adapter MUST refuse each event of a route generation that isn't current.

### EU-0016-053 {#EU-0016-053}

The session host MUST NOT send events to a host after the owner unpairs it.
