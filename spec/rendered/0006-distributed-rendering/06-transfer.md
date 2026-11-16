## Resumable transfer {#transfer}

Every render input and every result moves between computers as an item of the content store, in
chunks, as RFC 0008 states: the chunk list of each item, the fetch of only the chunks a receiver
lacks after a disconnect, and the check of each chunk before it counts as present. The transfer runs
apart from the job queue, and the owner's device discovery service advertises and configures it
without carrying its traffic. A result is written apart from the results the cache shows, checked,
and only then shown.

### EU-0006-060 {#EU-0006-060}

Every render input and every result MUST move between computers as an item of the content store.

### EU-0006-065 {#EU-0006-065}

A result MUST become visible in the render cache only once it is complete and checked.

### EU-0006-066 {#EU-0006-066}

A transfer MUST run at the priority level of the work it serves.

### EU-0006-090 {#EU-0006-090}

A computer MUST NOT send a chunk of a render input or a result to a computer that isn't one of the owner's paired computers.
