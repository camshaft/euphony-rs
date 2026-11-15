## Independent copies and signed history {#history}

Each collaborator holds a local repository of immutable revisions and content objects. A revision
names its parent revisions, the canonical root of the project's state, the project's schema version,
its author's signature and an optional message. A branch is a reference to a head revision that one
author owns and signs; two branches with the same display label stay two branches of their authors.
A branch head is a signed, numbered record. It holds the identity of the branch, made of the
author's key and a branch nonce, the identity of the head revision, a sequence number, the identity
of the previous head record and a timestamp, and the author's device key signs its stored bytes. Two
validly signed head records of one branch with the same sequence number are an equivocation. The
local command authority of a copy orders edits within its own working branch alone, so collaborators
edit while disconnected and exchange revisions later. Playback plays a chosen revision on the
computer that plays it, and collaboration leaves every audio deadline on that computer.

### EU-0013-001 {#EU-0013-001}

Each collaborator MUST hold an editable copy of the project with its whole history on their own computer.

### EU-0013-002 {#EU-0013-002}

Collaboration MUST NOT require a project hosting service or an elected server.

### EU-0013-003 {#EU-0013-003}

A revision MUST be immutable once it is made.

### EU-0013-004 {#EU-0013-004}

A revision MUST name its parent revisions, the root of its project state, its schema version and the signature of its author.

### EU-0013-005 {#EU-0013-005}

Each branch MUST be owned by exactly one author, whose device key alone signs its head records.

### EU-0013-006 {#EU-0013-006}

An equal display label MUST NOT give one peer control of a branch that another author owns.

### EU-0013-007 {#EU-0013-007}

A branch head record MUST hold the branch identity, the head revision identity, a sequence number, the identity of the previous head record and a timestamp, signed by its author.

### EU-0013-008 {#EU-0013-008}

A peer MUST accept a new branch head record only where its sequence number is higher than that of the accepted record and it names the accepted record as its previous record.

### EU-0013-009 {#EU-0013-009}

Where two validly signed head records of one branch hold the same sequence number, a peer MUST keep both as a typed conflict and show it.

### EU-0013-010 {#EU-0013-010}

A peer MUST NOT advance a branch whose head records conflict until its author signs a record that names one of them.

### EU-0013-011 {#EU-0013-011}

The local command authority of a copy MUST order edits only within its own working branch.

### EU-0013-012 {#EU-0013-012}

Two people MUST be able to edit disconnected branches at the same time and exchange their revisions later.

### EU-0013-013 {#EU-0013-013}

Playback MUST play a revision chosen on the computer that plays it.

### EU-0013-014 {#EU-0013-014}

Collaboration MUST NOT carry the deadline of an audio device across the network.

### EU-0013-015 {#EU-0013-015}

Undo MUST make a new revision and leave the shared history unchanged.
