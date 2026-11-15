## Session context for an assistant {#session-context}

An assistant begins a learning context for an authenticated principal, a project and the client,
genre and session it works in, with the schema versions it supports and, where it resumes, a cursor.
The answer is a snapshot of the context: its identity, the policy and context revision it was read
at, the active preferences and mechanisms, the unresolved conflicts, the pending work the principal
can read, and cursors to page through each kind of record. Context labels select which records
apply. The assistant reads the records of the snapshot and acknowledges the snapshot by its
identity, and the context handle that results is bound to the principal, the scope and the revision
it was read at. An acknowledgement shows that the context was delivered, and doesn't show that the
assistant understood it. Changes reach a connected assistant as events of the resumable event
stream; an assistant that can't subscribe refreshes its context when it begins or resumes work, and
when a command answers that its context is stale.

### EU-0019-010 {#EU-0019-010}

Beginning a learning context MUST return the snapshot identity, the context revision, the active preference and mechanism references, the unresolved conflicts, the pending work the principal can read and a cursor for each kind of record.

### EU-0019-011 {#EU-0019-011}

A context label MUST NOT grant access to any record.

### EU-0019-012 {#EU-0019-012}

Each record a context read returns MUST carry its type and its provenance.

### EU-0019-013 {#EU-0019-013}

An acknowledgement of a learning context MUST record that the snapshot it names was delivered to its principal.

### EU-0019-014 {#EU-0019-014}

Euphony MUST NOT present an acknowledgement as evidence that an assistant understood the context.

### EU-0019-015 {#EU-0019-015}

A context handle MUST be bound to its principal, its scope and the revision it was read at.

### EU-0019-016 {#EU-0019-016}

A mutating command in an assisted session that doesn't carry a context handle MUST be refused as context-required.

### EU-0019-017 {#EU-0019-017}

A mutating command in an assisted session whose context handle is no longer current MUST be refused as context-stale, with a refreshed context the principal can read.

### EU-0019-018 {#EU-0019-018}

Read-only discovery MUST stay available to an assistant without a context handle.

### EU-0019-019 {#EU-0019-019}

Euphony MUST check the principal's current authority and the relevant context revision again when it applies each assisted command.

### EU-0019-020 {#EU-0019-020}

Unpairing a device or revoking a link MUST invalidate at once each context handle and subscription it affects.

### EU-0019-021 {#EU-0019-021}

A change to the learning context MUST reach a subscribed assistant as an event of the resumable event stream.

### EU-0019-022 {#EU-0019-022}

Euphony MUST NOT require an assistant to poll for changes to its context.

### EU-0019-023 {#EU-0019-023}

A reconnection MUST resume from the assistant's cursor, or from a revisioned snapshot where the history after that cursor was retired.

### EU-0019-024 {#EU-0019-024}

A context read MUST follow the context capacity the client declares.

### EU-0019-025 {#EU-0019-025}

A context read MUST NOT cut the preferences it returns at a fixed count.

### EU-0019-026 {#EU-0019-026}

A context read that returns part of the context MUST say that it is incomplete.

### EU-0019-027 {#EU-0019-027}

A failed context read MUST leave assisted mutation unavailable, with its reason.
