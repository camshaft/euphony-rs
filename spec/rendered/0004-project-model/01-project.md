## The project and its timeline {#project}

A project is a piece of music as euphony holds it: its tracks, regions, events, automation, routing
and processors with their settings, and the links between them. Each project has an identity and a
sample rate, a whole number of samples per second greater than 0. Every time in a project is a
whole number of samples at that rate from the start of its
timeline, whatever the clock of the computer that edits, plays or renders it. Each link from one part
of a project to another is a typed field: a region refers to the audio it plays by the content
identity of that audio, and a processor refers to the definition it runs by the identity and version
of that definition. A copy of a project is the project as one computer holds it. Rendering, content
identities and the processors themselves are the subjects of other RFCs.
A project comes to exist through the command that creates it, a command of a type the command
schema defines that is sent alone and not in a batch, since no project exists yet for a batch to
name. The command names the sample rate of the new project, its actor, which is the caller that
euphony authenticated and the party the first revision records as having made the change, and a
key that identifies it across retries; it doesn't name an identity.
euphony checks the command against the command schema before it creates the project, and refuses
as invalid input a creation that fails the check, such as one whose sample rate is 0.
euphony chooses the identity of the new project without regard to the identities other projects
hold, and the outcome of the command identifies the project and its first revision, which the
first batch sent to the project names as its expected revision. A creation sent again with the same
actor and key receives the outcome of the first and doesn't create a second project. Only the owner,
a paired owner device and an assistant the owner connected may create a project: a collaborator
reaches only the projects that its share links reach.

### EU-0004-001 {#EU-0004-001}

Each project MUST have an identity that no other project has.

### EU-0004-002 {#EU-0004-002}

Each project MUST have exactly one sample rate.

### EU-0004-092 {#EU-0004-092}

The command schema MUST define the sample rate that the command creating a project names as a whole number of samples per second greater than 0.

### EU-0004-070 {#EU-0004-070}

The command that creates a project MUST be sent alone, and not in a batch.

### EU-0004-071 {#EU-0004-071}

A command that creates a project MUST NOT name an identity for the project it creates.

### EU-0004-072 {#EU-0004-072}

euphony MUST choose the identity of each project it creates without regard to the identities other projects hold.

### EU-0004-073 {#EU-0004-073}

The outcome of a command that creates a project MUST identify the new project and its first revision.

### EU-0004-074 {#EU-0004-074}

euphony MUST refuse as denied a command that creates a project from a caller other than the owner, a paired owner device or an assistant the owner connected, before it checks the command's idempotency key.

### EU-0004-075 {#EU-0004-075}

The command that creates a project MUST name its actor and an idempotency key.

### EU-0004-076 {#EU-0004-076}

Where the actor that a command creating a project names is not the caller that euphony authenticated, euphony MUST refuse the command as denied before it checks its idempotency key.

### EU-0004-090 {#EU-0004-090}

euphony MUST check the command that creates a project against the command schema before it creates the project.

### EU-0004-091 {#EU-0004-091}

Where the command that creates a project fails the check against the command schema, euphony MUST refuse the command as invalid input.

### EU-0004-077 {#EU-0004-077}

A command that creates a project sent again with the actor, idempotency key and sample rate of a committed creation MUST receive the outcome of that creation.

### EU-0004-078 {#EU-0004-078}

A command that creates a project with the actor and idempotency key of a committed creation MUST NOT create another project.

### EU-0004-079 {#EU-0004-079}

Where a command that creates a project has the actor and idempotency key of a committed creation and another sample rate, euphony MUST refuse it as invalid input.

### EU-0004-080 {#EU-0004-080}

The outcome of a refused command that creates a project MUST be one of denied, invalid input or unavailable storage.

### EU-0004-003 {#EU-0004-003}

Every time in a project MUST be a whole number of samples at the sample rate of the project.

### EU-0004-004 {#EU-0004-004}

The times of a project MUST NOT depend on the clock of any computer that holds, plays or renders it.

### EU-0004-005 {#EU-0004-005}

Each link from one part of a project to another MUST be a typed field of the project.

### EU-0004-006 {#EU-0004-006}

A region MUST refer to the audio it plays by the content identity of that audio.

### EU-0004-007 {#EU-0004-007}

A processor MUST refer to the definition it runs by the identity and version of that definition.

### EU-0004-008 {#EU-0004-008}

A decision of euphony MUST NOT rest on the text of a label, a description or a message in a project.
