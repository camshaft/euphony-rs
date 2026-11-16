## Workers and scheduling {#workers}

The host of a project keeps its render job queue and the journal of that queue. A worker is
described to the owner's device discovery service as data: its render profiles, the formats it
accepts and the addresses of its job and transfer endpoints. The discovery service reports each
worker's presence and configures the connection to it, and carries none of the render traffic. The
scheduler reads every capacity signal, such as completions, latency, errors and storage pressure,
from the job protocol itself. An idle worker pulls a lease on a job. Render work has two priority
levels, interactive preparation for what the musician is about to hear, then speculative
preparation, and the messages that start, cancel and answer an attempt come ahead of both. Each new
attempt at a job replaces the earlier attempts of that job. A local authority orders the edits of
each branch of a project, and independent branches, render jobs and transfers proceed beside it.

### EU-0006-046 {#EU-0006-046}

A unit of render work MUST be a render island over a time range.

### EU-0006-047 {#EU-0006-047}

Render jobs and transfers MUST NOT wait on the authority that orders a branch's edits.

### EU-0006-048 {#EU-0006-048}

Adding a worker MUST NOT require a change to the owner's device discovery service.

### EU-0006-049 {#EU-0006-049}

The scheduler MUST stop placing work on a worker once the discovery service reports that worker gone.

### EU-0006-050 {#EU-0006-050}

The scheduler MUST NOT impose a fixed limit on the work that a worker or a link takes.

### EU-0006-051 {#EU-0006-051}

The scheduler MUST size the work it assigns to each worker and each link from that worker's measured completions, latency, errors and storage pressure.

### EU-0006-052 {#EU-0006-052}

Interactive preparation MUST be scheduled ahead of speculative preparation.

### EU-0006-053 {#EU-0006-053}

Projects at the same priority level MUST each receive a fair share of the available capacity.

### EU-0006-054 {#EU-0006-054}

The scheduler SHOULD place a job on the worker with the least measured transfer time to its inputs.

### EU-0006-055 {#EU-0006-055}

The host MUST refuse a result from an attempt that a newer attempt of the same job has replaced.

### EU-0006-056 {#EU-0006-056}

A message that starts, cancels or answers an attempt MUST NOT wait behind queued bulk transfers on a connection.

### EU-0006-057 {#EU-0006-057}

Every connection between a worker and a host MUST be authenticated and encrypted.

### EU-0006-058 {#EU-0006-058}

A computer that isn't one of the owner's paired computers MUST NOT be given render work.

### EU-0006-059 {#EU-0006-059}

An accepted render job MUST NOT be dropped for want of capacity.

### EU-0006-079 {#EU-0006-079}

A worker's lease on a job MUST end only when the attempt it holds answers or is cancelled, the discovery service reports that worker gone, or the connection to that worker closes.
