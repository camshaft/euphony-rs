## Imported projects and the project format {#scope}

An import reads a project saved by another music workstation, called the source project here, and
proposes a euphony project built from it. The source workstation's format isn't published, and what
is known of it holds for the format versions that were measured. euphony keeps every project in its
own format, which stays the authoritative copy after an import. euphony doesn't send projects back
to a source workstation: a musician who wants to return keeps the source project, which the import
leaves as it was. Rendered audio, stems or notes exported for other programs aren't a project in a
source workstation's format, and other RFCs state them. The import and its reports serve the
owner's desktop interface and connected assistants alike, through the commands every client uses.

### EU-0014-001 {#EU-0014-001}

An imported project MUST be kept in euphony's own project format.

### EU-0014-002 {#EU-0014-002}

The import MUST leave the source project unchanged.

### EU-0014-003 {#EU-0014-003}

euphony MUST NOT write a project in the format of a source workstation.

### EU-0014-004 {#EU-0014-004}

A request for a project in the format of a source workstation MUST be answered with an error.

### EU-0014-005 {#EU-0014-005}

The import and its reports MUST be available equally to the owner's desktop interface and to connected assistants.
