## What euphony keeps and what assistants do {#boundary}

Preferences and workflow learning let the assistants a person chooses learn that person's musical
taste and the way they like to work, and improve repeated work over time, while the person keeps
control of what is learned. Euphony keeps, for each owner, a learning store of evidence, preference
versions, workflow mechanisms, test and effect reports and retrospective work. The owner is the
person whose taste and workflow the store records. An assistant is an external client of the client
protocol that reasons for the person; euphony doesn't reason for itself. Euphony serves the store to
the assistant, receives the assistant's evidence and proposals, and applies only what the owner
accepted. Learning runs beside the command gateway and away from the real-time audio path, so that
playback, recording and ordinary editing go on whatever the learning store does.

### EU-0019-001 {#EU-0019-001}

Euphony MUST keep one learning store for each owner, holding that owner's evidence, preference versions, mechanism versions, test and effect reports and retrospective work.

### EU-0019-002 {#EU-0019-002}

Euphony MUST NOT run an assistant, an inference model for an assistant, a connection to a model provider or a reasoning loop of its own.

### EU-0019-003 {#EU-0019-003}

Euphony MUST NOT hold the credentials an assistant uses for a model service.

### EU-0019-004 {#EU-0019-004}

Euphony MUST serve the learning store to assistants through the resources, tools and procedures of its client protocol alone.

### EU-0019-005 {#EU-0019-005}

Euphony MUST NOT depend on an assistant running a procedure or reading a notification that euphony offers it.

### EU-0019-006 {#EU-0019-006}

Learning work MUST NOT run on the real-time audio path.

### EU-0019-007 {#EU-0019-007}

Playback, recording and ordinary editing MUST go on while the learning store is unavailable or failing.

### EU-0019-008 {#EU-0019-008}

Ordinary native operation MUST stay available whether or not an assistant is attached.

### EU-0019-009 {#EU-0019-009}

Euphony MUST define the assistants it supports by the protocol capabilities they declare, not by their vendor or product.
