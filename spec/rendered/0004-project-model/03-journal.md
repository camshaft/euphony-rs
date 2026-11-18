## The journal and the history {#journal}

The journal of a copy holds every batch that copy committed, in order, with its revision, its actor
and the changes it made. It also holds the creation of each project that copy created, with the actor
of the creation and the first revision of the project, ahead of every batch of that project. Where a
change needed an approval, its entry holds the actor that proposed it and the person who approved it.
When a copy opens after its computer or its program stopped, it rebuilds each project from its
journal, a project whose creation it recorded among them. Where a journal is lost, the owner restores
it from a backup of that journal or from a revision of another copy that the owner chooses. Which
changes need an approval, and who may read a project, are stated in the overview RFC.

### EU-0004-025 {#EU-0004-025}

A copy MUST record a batch durably in its journal before the outcome of the batch reports it committed.

### EU-0004-081 {#EU-0004-081}

A copy MUST record the creation of a project, the actor of the creation and the first revision of the project durably in its journal before the outcome of the creation reports the project created.

### EU-0004-026 {#EU-0004-026}

Each journal entry MUST record the actor of its batch.

### EU-0004-027 {#EU-0004-027}

Where a change needed an approval, its journal entry MUST record its proposer and its approver as separate actors.

### EU-0004-028 {#EU-0004-028}

Anyone who may read a project MUST be able to read, for each of its revisions, the changes it made and its actor.

### EU-0004-029 {#EU-0004-029}

A copy opened after an outage MUST hold every revision its journal recorded.

### EU-0004-082 {#EU-0004-082}

A copy opened after an outage MUST hold each project whose creation its journal recorded.

### EU-0004-030 {#EU-0004-030}

A lost journal MUST be restorable from a backup of that journal or from a revision of another copy that the owner chooses.

### EU-0004-031 {#EU-0004-031}

A restore MUST NOT take a revision from a copy that the owner did not choose.
