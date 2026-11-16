## Commands and their outcomes {#commands}

Every change a person or an assistant makes to a project is a command. Commands are sent in batches.
A batch names the project, the branch it changes, the revision its sender expects to change on that
branch, a key that identifies the batch across retries, and the actor that sends it. The actor is the
caller that euphony authenticated, and euphony refuses as denied a batch that names another actor
before it reads any batch of that actor. euphony also refuses as denied a batch whose caller may not
take its commands on the branch it names, before it checks the batch's commands against the project
schema, its expected revision or its key. It refuses a batch that names a project that doesn't
exist, or a branch that the project doesn't hold, in the same way and at the same point of its
checks, so that a refusal doesn't tell a caller whether the project or the branch exists. euphony
then refuses as invalid input a batch that names a working branch another copy holds, and a batch
whose expected revision isn't in the history of the branch it names. euphony answers that history
check with a fixed number of reads, each bounded by an index, however long the branch's history is.
euphony checks each batch against the access rules that the overview RFC states and the project
schema, and then commits it whole as a new revision, or
refuses it whole with a typed reason: a conflict, a denial, invalid input or unavailable storage. A
conflict carries, at the head of the batch's branch, the view of each part of the project that the
batch's commands name or change and that its actor may read, with that part's own fields and the
identities of the parts it holds that its actor may read; it names that head to an actor who may
read the branch, and the sender reads the rest of the project in pages, as the revisions section
states. A gesture in an interface, a line of a script, the result of a composition program and the
call of an assistant all reach the project as such batches. A change of presentation that leaves the
project as it was, such as scrolling, isn't a command. The protocol that carries batches to euphony
is the subject of the client protocol RFC. The command gateway of the overview RFC refuses a batch
from an unpaired device, beyond the level of its link, or over an expired or revoked link, with the
reason the overview RFC sets for it, before euphony makes any check of this RFC, and asks for the
owner's approval only of a batch that the key check doesn't answer as a retry. A key identifies a
batch only among the batches one actor sends to one project: a retry of a committed batch on the
same branch receives that batch's outcome without a new approval, unless one of the checks that
euphony makes before it checks the key refuses it first, as a denial does once its caller may no
longer change the project, and a key used again for another branch or other commands is refused.
A batch of another actor or another project with the same key is a new batch under its own key.

### EU-0004-009 {#EU-0004-009}

Each persistent change to a project MUST be made by a command.

### EU-0004-010 {#EU-0004-010}

Each command MUST have a type that the command schema of euphony defines.

### EU-0004-011 {#EU-0004-011}

A batch MUST name its project, the branch it changes, its expected revision, its idempotency key and its actor.

### EU-0004-049 {#EU-0004-049}

Where the actor that a batch names is not the caller that euphony authenticated, euphony MUST refuse the batch as denied before it checks the batch's expected revision or its idempotency key.

### EU-0004-056 {#EU-0004-056}

Where the caller that euphony authenticated may not take every command of a batch on the branch that the batch names, euphony MUST refuse the batch as denied before it checks the batch's commands against the project schema, its expected revision or its idempotency key.

### EU-0004-095 {#EU-0004-095}

Where the branch that a batch names is not a branch of the project, euphony MUST refuse the batch as denied, with the same answer as EU-0004-056, before it checks the batch's commands against the project schema, its expected revision or its idempotency key.

### EU-0004-096 {#EU-0004-096}

Where the branch that a batch names is a working branch that another copy holds, euphony MUST refuse the batch as invalid input after the checks of EU-0004-056 and EU-0004-095 and before it checks the batch's expected revision or its idempotency key.

### EU-0004-064 {#EU-0004-064}

Where the expected revision of a batch is neither the head of the branch that the batch names nor a revision that this head follows, directly or through other revisions, euphony MUST refuse the batch as invalid input after the check of EU-0004-056 and before it checks the batch's idempotency key.

### EU-0004-059 {#EU-0004-059}

Where the project that a batch names does not exist, euphony MUST refuse the batch as denied before it checks the batch's commands against the project schema, its expected revision or its idempotency key, with a refusal whose kind and content, apart from the values the batch itself supplies, are those of its refusal of EU-0004-056 of a batch whose caller may not take its commands on the branch it names.

### EU-0004-067 {#EU-0004-067}

euphony MUST make the check of EU-0004-064 with a fixed number of reads, each bounded by an index and returning a fixed number of records, whatever the length of the history of the branch that the batch names.

### EU-0004-012 {#EU-0004-012}

euphony MUST check every command of a batch against the project schema before it commits the batch.

### EU-0004-013 {#EU-0004-013}

A batch MUST commit whole or not at all.

### EU-0004-014 {#EU-0004-014}

A committed batch MUST produce exactly one new revision.

### EU-0004-015 {#EU-0004-015}

The outcome of a committed batch MUST identify the revision it committed and the changes it made.

### EU-0004-016 {#EU-0004-016}

The outcome of a batch that the command gateway of the overview RFC accepted and euphony refused MUST be one of conflict, denied, invalid input or unavailable storage.

### EU-0004-017 {#EU-0004-017}

Where the expected revision of a batch is a revision that the head of the branch that the batch names follows, directly or through other revisions, and not that head, and no committed batch of the same project and actor holds its idempotency key, euphony MUST refuse the batch as a conflict.

### EU-0004-018 {#EU-0004-018}

A conflict MUST include, at the head of the batch's branch, the view of each part of the project that a command of the batch names or changes and that the actor of the batch may read, holding that part's own fields and the identities of the parts it holds that the actor of the batch may read, without their contents.

### EU-0004-051 {#EU-0004-051}

Where the actor of a batch may read the batch's branch, a conflict MUST identify the head revision of that branch.

### EU-0004-019 {#EU-0004-019}

A batch with the project, actor and idempotency key of a committed batch MUST NOT be applied again.

### EU-0004-020 {#EU-0004-020}

A batch sent again with the project, branch, actor, idempotency key and commands of a committed batch MUST receive the outcome of that committed batch, without a new approval by the owner, except where the command gateway of the overview RFC refuses it as unpaired, outside link level, or link expired or revoked before euphony makes the checks of EU-0004-049, EU-0004-056, EU-0004-059 and EU-0004-095, or where euphony refuses it before it checks its idempotency key, as denied by EU-0004-049, EU-0004-056, EU-0004-059 or EU-0004-095, or as invalid input by EU-0004-096 or EU-0004-064, such as a retry from a device the owner unpaired or over a share link the owner revoked after that batch committed, a retry from a caller who lost change access after that batch committed, or a retry that reaches a copy after another copy came to hold the branch of that batch.

### EU-0004-046 {#EU-0004-046}

Where a batch has the project, actor and idempotency key of a committed batch and names another branch or other commands, euphony MUST refuse the batch as invalid input.

### EU-0004-047 {#EU-0004-047}

euphony MUST NOT answer a batch with the outcome of a committed batch of another actor or another project.

### EU-0004-048 {#EU-0004-048}

Where a batch has the idempotency key of a committed batch of another actor or another project, and no committed batch of the same project and actor holds that key, euphony MUST handle it as a new batch.

### EU-0004-021 {#EU-0004-021}

A change MUST pass the same checks whichever interface, command line, script, composition program or assistant sends it.

### EU-0004-022 {#EU-0004-022}

Each committed batch MUST be undoable by a later batch.

### EU-0004-023 {#EU-0004-023}

A change of presentation that leaves the project as it was MUST NOT produce a revision.

### EU-0004-024 {#EU-0004-024}

Where a command starts work that ends after its batch commits, the outcome of the batch MUST identify that work apart from the revision.
