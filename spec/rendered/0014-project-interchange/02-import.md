## Reading a source project {#import}

An import is staged. It reads a source project the musician chose and has access to, identifies its
format version from the data the project records, reads the alternative the project marks active,
takes an inventory of its arrangement and files, reads the content it supports apart from the rest
of the system, and builds a proposed project with a conversion report. Nothing changes until the
musician accepts the proposal; a refusal, a failure or an interruption leaves the source project and
every euphony project as they were. A reader of a format version is verified when its reading of
that version matches saves the musician made in the source workstation, each changing one known
value. The folders the musician selects bound the files an import reads.

### EU-0014-006 {#EU-0014-006}

The import MUST identify the format version of a source project before it reads the project's content.

### EU-0014-007 {#EU-0014-007}

The import MUST refuse a source project of a format version it has no verified reader for.

### EU-0014-008 {#EU-0014-008}

The refusal of a source project's format version MUST show that version.

### EU-0014-009 {#EU-0014-009}

Where a source project holds several alternatives, the import MUST read the one the project marks active.

### EU-0014-010 {#EU-0014-010}

The import MUST resolve a file reference outside the folders the musician selected only with the musician's permission.

### EU-0014-011 {#EU-0014-011}

The import MUST NOT run executable content that a source project holds.

### EU-0014-012 {#EU-0014-012}

The import MUST refuse a path in a source project that leads outside the folders it reads.

### EU-0014-013 {#EU-0014-013}

The import MUST refuse a malformed record of a source project.

### EU-0014-014 {#EU-0014-014}

The import MUST judge whether a size a source project asks for is too large from the resources available, not from a fixed bound.

### EU-0014-015 {#EU-0014-015}

An interrupted import MUST leave its proposed project unapplied.

### EU-0014-016 {#EU-0014-016}

The import MUST show the musician the proposed project and its conversion report before it commits anything.

### EU-0014-017 {#EU-0014-017}

The import MUST NOT commit a proposed project the musician hasn't accepted.

### EU-0014-018 {#EU-0014-018}

The import MUST commit an accepted project as a new branch, through the commands every client uses.

### EU-0014-019 {#EU-0014-019}

Importing the same source project again at the same importer and mapping versions MUST produce the same project and the same report.

### EU-0014-020 {#EU-0014-020}

The import result MUST record the versions of the importer and of each mapping it used.
