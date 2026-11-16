## Resumable transfer {#transfer}

Every render input and every result moves between computers as fixed-size immutable chunks. Each
chunk is named by a cryptographic digest of its stored bytes, and a signed manifest lists the chunks
of each object. After a disconnect, the receiver asks which chunks it still lacks and fetches only
those. The transfer runs apart from the job queue, and the owner's device discovery service
advertises and configures it without carrying its traffic. A result is written apart from the
results the cache shows, checked, and only then shown.

### EU-0006-060 {#EU-0006-060}

Every render input and every result MUST move between computers as immutable chunks, each named by a cryptographic digest of its bytes.

### EU-0006-061 {#EU-0006-061}

The chunks of each object MUST be listed in a signed manifest.

### EU-0006-062 {#EU-0006-062}

After a disconnect, a receiver MUST fetch only the chunks it still lacks.

### EU-0006-063 {#EU-0006-063}

A receiver MUST check each chunk against its digest before it counts the chunk as present.

### EU-0006-064 {#EU-0006-064}

A receiver MUST record a chunk as present only after its data is durable.

### EU-0006-065 {#EU-0006-065}

A result MUST become visible in the render cache only once it is complete and checked.

### EU-0006-066 {#EU-0006-066}

A transfer MUST run at the priority level of the work it serves.

### EU-0006-090 {#EU-0006-090}

A computer MUST NOT send a chunk of a render input or a result to a computer that isn't one of the owner's paired computers.
