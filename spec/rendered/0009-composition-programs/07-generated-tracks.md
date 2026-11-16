## Generated tracks and regeneration {#generated-tracks}

Each generated track stays linked to the version of the program that made it and to the identity of
each asset the program imported and each musical input it read. Each generated event has an
explicit start and an explicit end, which an editor shows and
moves, and an identity that stays the same across regenerations of the same program. Regenerating a
track shows the difference first. The musician's own edits of a generated track stay in a layer of
their own, and replacing them is an explicit command that can be undone. A project made only of
notes and programs keeps every input needed to rebuild its music, and a new version of a definition
leaves the meaning of saved values as it was until an explicit migration changes it.

### EU-0009-070 {#EU-0009-070}

A generated track MUST stay linked to the version of the program that made it and to the identity of each asset the program imported and each musical input it read.

### EU-0009-071 {#EU-0009-071}

Each generated event MUST have an explicit start and an explicit end.

### EU-0009-072 {#EU-0009-072}

Each generated event MUST keep the same identity across regenerations of the same program.

### EU-0009-073 {#EU-0009-073}

A regeneration MUST show a preview of its difference before it changes the project.

### EU-0009-074 {#EU-0009-074}

The edits a musician makes to a generated track MUST be kept in a layer apart from the generated events.

### EU-0009-075 {#EU-0009-075}

A regeneration MUST NOT replace the edits of a musician except through an explicit command.

### EU-0009-076 {#EU-0009-076}

The command that replaces the edits of a musician with generated events MUST be undoable.

### EU-0009-077 {#EU-0009-077}

A project made only of notes and composition programs MUST keep every input needed to rebuild its musical content.

### EU-0009-078 {#EU-0009-078}

A new version of a definition MUST NOT change the meaning of a value that a saved project holds without an explicit migration.

### EU-0009-079 {#EU-0009-079}

A migration of the values of a saved project to a new version of a definition MUST be undoable.
