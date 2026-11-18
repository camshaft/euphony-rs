## Commands, compute and access {#commands}

Four typed commands drive a rebuild, the same for the desktop interface and for each assistant the
musician connects: prepare a rebuild, run or cancel it, start the blind comparison of a candidate,
and accept a candidate. Fitting runs on the workers of distributed rendering through its
scheduler. The entry points of a rebuild are source media, structure proposals and worker results;
a proposal is checked against the processor catalog, and a worker's result is a candidate. This
RFC doesn't add an access rule of its own: an assistant's rebuild commands act as the musician's
own, under the trust model of the system overview.

### EU-0023-058 {#EU-0023-058}

Each rebuild command MUST be available equally to the desktop interface and to each assistant the musician connects.

### EU-0023-059 {#EU-0023-059}

A connected assistant MUST be able to start, follow and compare a rebuild as the musician does.

### EU-0023-060 {#EU-0023-060}

The prepare command MUST take a source item and interval, or the identity of a capture, with optional notes, the kinds of structure to try and the expected project revision.

### EU-0023-061 {#EU-0023-061}

The prepare command MUST return a plan, or one of denied, missing asset, unsupported source, not enough material and interval outside the item.

### EU-0023-062 {#EU-0023-062}

The run command MUST return durable state events and each candidate as it finishes.

### EU-0023-063 {#EU-0023-063}

Where a run doesn't finish, the run command MUST return one of cancelled and failed fit.

### EU-0023-064 {#EU-0023-064}

Where a worker is lost, the queued work of the run MUST resume on another worker.

### EU-0023-065 {#EU-0023-065}

The blind comparison command MUST return its trials and the recorded result.

### EU-0023-066 {#EU-0023-066}

The accept command MUST return the committed revision, or one of denied, stale revision, missing candidate and no blind result.

### EU-0023-067 {#EU-0023-067}

The accept command MUST return the outcome no blind result for a candidate whose blind comparison the musician hasn't taken, whether the musician or an assistant calls it.

### EU-0023-068 {#EU-0023-068}

Fitting MUST run on the workers of distributed rendering through its scheduler.

### EU-0023-069 {#EU-0023-069}

Fitting MUST NOT be held to a fixed bound on rounds, workers or jobs.

### EU-0023-070 {#EU-0023-070}

A rebuild MUST NOT send anything outside the owner's computers.

### EU-0023-071 {#EU-0023-071}

A worker's result MUST NOT change the project.

### EU-0023-076 {#EU-0023-076}

The run command MUST return worker lost when a worker of the run is lost.
