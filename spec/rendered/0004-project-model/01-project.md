## The project and its timeline {#project}

A project is a piece of music as euphony holds it: its tracks, regions, events, automation, routing
and processors with their settings, and the links between them. Each project has an identity and a
sample rate. Every time in a project is a whole number of samples at that rate from the start of its
timeline, whatever the clock of the computer that edits, plays or renders it. Each link from one part
of a project to another is a typed field: a region refers to the audio it plays by the content
identity of that audio, and a processor refers to the definition it runs by the identity and version
of that definition. A copy of a project is the project as one computer holds it. Rendering, content
identities and the processors themselves are the subjects of other RFCs.

### EU-0004-001 {#EU-0004-001}

Each project MUST have an identity that no other project has.

### EU-0004-002 {#EU-0004-002}

Each project MUST have exactly one sample rate.

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
