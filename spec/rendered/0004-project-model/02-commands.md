## Commands and their outcomes {#commands}

Every change a person or an assistant makes to a project is a command. Commands are sent in batches.
A batch names the project, the branch it changes, the revision its sender expects to change on that
branch, a key that identifies the batch across retries, and the actor that sends it. The actor is the
caller that euphony authenticated, and euphony refuses as denied a batch that names another actor
before it reads any batch of that actor. euphony also refuses as denied a batch whose caller may not
take its commands on the branch it names, or that names a branch the project doesn't hold, with one
answer for both, before it checks the batch's expected revision or its key. It then refuses as
invalid input a batch that names a working branch another copy holds, and a batch whose expected
revision isn't in the history of that branch.
euphony checks each batch against the project schema and
the access rules that the overview RFC states, and then commits it whole as a new revision, or
refuses it whole with a typed reason: a conflict, a denial, invalid input or unavailable storage. A
gesture in an interface, a line of a script, the result of a composition program and the call of an
assistant all reach the project as such batches. A change of presentation that leaves the project as
it was, such as scrolling, isn't a command. The protocol that carries batches to euphony is the
subject of the client protocol RFC. A key identifies a batch only among the batches one actor sends
to one project: a retry of a committed batch on the same branch that passes every check euphony
makes before it checks the key receives that batch's outcome, and a key used again for another
branch or other commands is refused.

### EU-0004-009 {#EU-0004-009}

Each persistent change to a project MUST be made by a command.

### EU-0004-010 {#EU-0004-010}

Each command MUST have a type that the command schema of euphony defines.

### EU-0004-011 {#EU-0004-011}

A batch MUST name its project, the branch it changes, its expected revision, its idempotency key and its actor.

### EU-0004-049 {#EU-0004-049}

Where the actor that a batch names is not the caller that euphony authenticated, euphony MUST refuse the batch as denied before it checks the batch's expected revision or its idempotency key.

### EU-0004-056 {#EU-0004-056}

Where the caller that euphony authenticated may not take every command of a batch on the branch that the batch names, euphony MUST refuse the batch as denied before it checks the batch's expected revision or its idempotency key.

### EU-0004-092 {#EU-0004-092}

Where the branch that a batch names is not a branch of the project, euphony MUST refuse the batch as denied, with the same answer as EU-0004-056, before it checks the batch's expected revision or its idempotency key.

### EU-0004-093 {#EU-0004-093}

Where the branch that a batch names is a working branch that another copy holds, euphony MUST refuse the batch as invalid input after the checks of EU-0004-056 and EU-0004-092 and before it checks the batch's expected revision or its idempotency key.

### EU-0004-064 {#EU-0004-064}

Where the expected revision of a batch is neither the head of the branch that the batch names nor a revision that this head follows, directly or through other revisions, euphony MUST refuse the batch as invalid input after the check of EU-0004-056 and before it checks the batch's idempotency key.

### EU-0004-012 {#EU-0004-012}

euphony MUST check every command of a batch against the project schema before it commits the batch.

### EU-0004-013 {#EU-0004-013}

A batch MUST commit whole or not at all.

### EU-0004-014 {#EU-0004-014}

A committed batch MUST produce exactly one new revision.

### EU-0004-015 {#EU-0004-015}

The outcome of a committed batch MUST identify the revision it committed and the changes it made.

### EU-0004-016 {#EU-0004-016}

The outcome of a refused batch MUST be one of conflict, denied, invalid input or unavailable storage.

### EU-0004-017 {#EU-0004-017}

Where the expected revision of a batch is a revision that the head of the branch that the batch names follows, directly or through other revisions, and not that head, and no committed batch of the same project and actor holds its idempotency key, euphony MUST refuse the batch as a conflict.

### EU-0004-018 {#EU-0004-018}

A conflict MUST include the current view of the project that the actor of the batch may read.

### EU-0004-019 {#EU-0004-019}

A batch with the project, actor and idempotency key of a committed batch MUST NOT be applied again.

### EU-0004-020 {#EU-0004-020}

A batch sent again with the project, branch, actor, idempotency key and commands of a committed batch MUST receive the outcome of that committed batch, except where euphony refuses it before it checks its idempotency key, as denied by EU-0004-049, EU-0004-056 or EU-0004-092, or as invalid input by EU-0004-093 or EU-0004-064, such as a retry from a caller who lost change access after that batch committed, or a retry that reaches a copy after another copy came to hold the branch of that batch.

### EU-0004-046 {#EU-0004-046}

Where a batch has the project, actor and idempotency key of a committed batch and names another branch or other commands, euphony MUST refuse the batch as invalid input.

### EU-0004-047 {#EU-0004-047}

euphony MUST NOT answer a batch with the outcome of a committed batch of another actor or another project.

### EU-0004-021 {#EU-0004-021}

A change MUST pass the same checks whichever interface, command line, script, composition program or assistant sends it.

### EU-0004-022 {#EU-0004-022}

Each committed batch MUST be undoable by a later batch.

### EU-0004-023 {#EU-0004-023}

A change of presentation that leaves the project as it was MUST NOT produce a revision.

### EU-0004-024 {#EU-0004-024}

Where a command starts work that ends after its batch commits, the outcome of the batch MUST identify that work apart from the revision.
