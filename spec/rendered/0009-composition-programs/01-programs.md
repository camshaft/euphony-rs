## Composition programs {#programs}

A composition program is a short typed program that a musician or an agent writes to generate
notes, rhythms and arrangements. It is an asset of the project, kept in versions. Each version holds
the text of the program, the version of the musical type library it is written against, the version
of the runtime that runs it, the assets it imports and the musical inputs it reads, and each run
uses an explicit random seed. A program runs offline, when a musician or an agent asks. Its output
is a set of musical events and arrangement proposals, which become ordinary editable tracks once the
project accepts them; a program doesn't make sound during playback. The live transform of incoming
events, which uses the same musical types, belongs to RFC 0016, and the conversion of musical values
to modulation controls belongs to RFC 0012.

### EU-0009-001 {#EU-0009-001}

A composition program MUST be an asset of its project, kept in versions.

### EU-0009-002 {#EU-0009-002}

Each version of a composition program MUST record its text, the version of the musical type library it is written against and the version of the runtime that runs it.

### EU-0009-003 {#EU-0009-003}

Each version of a composition program MUST declare every asset it imports and every musical input it reads.

### EU-0009-004 {#EU-0009-004}

Each run of a composition program MUST use an explicit random seed.

### EU-0009-005 {#EU-0009-005}

The output of a composition program MUST be musical events and arrangement proposals.

### EU-0009-006 {#EU-0009-006}

A composition program MUST NOT make sound during playback.

### EU-0009-007 {#EU-0009-007}

The accepted output of a composition program MUST become editable tracks of the project.

### EU-0009-008 {#EU-0009-008}

A musician MUST be able to write, type-check, preview and apply a composition program.

### EU-0009-009 {#EU-0009-009}

An agent MUST be able to write, type-check, preview and apply a composition program through the same interface as the editor.
