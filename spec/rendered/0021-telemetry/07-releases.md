## Releases and the submission schema {#releases}

A release's public configuration sets what its clients send and where. The release check reads
each release before it is published and refuses one whose telemetry breaks what this RFC states.
Each release build publishes the symbol files of its own code, so that the receiving service turns
crash frames into function names without the client sending them.

### EU-0021-163 {#EU-0021-163}

The public configuration of each release MUST set the submission schema version and the destination.

### EU-0021-164 {#EU-0021-164}

A change of the submission schema MUST be a reviewed public change that assigns each new field its privacy class.

### EU-0021-165 {#EU-0021-165}

The release notes MUST describe each change of the submission schema.

### EU-0021-166 {#EU-0021-166}

Each release build MUST publish the symbol files of its own code.

### EU-0021-167 {#EU-0021-167}

A sent crash frame MUST NOT hold a function name.

### EU-0021-168 {#EU-0021-168}

The release check MUST NOT pass a release in which sending is on in a new installation.

### EU-0021-169 {#EU-0021-169}

The release check MUST NOT pass a release with a submission field that has no privacy class or is free text.

### EU-0021-170 {#EU-0021-170}

The release check MUST NOT pass a release whose destination differs from its public configuration.

### EU-0021-171 {#EU-0021-171}

The release check MUST NOT pass a release whose symbol files are missing.
