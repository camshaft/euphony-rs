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
Each attempt reports its progress over the job protocol, and the scheduler measures each
worker's rate of progress from those reports as it measures its other capacity signals.
While a worker holds a lease, the job protocol closes the connection to that worker once the
transport reports that the worker stopped acknowledging what is sent on it, which ends that lease.
While the lease is held, the job protocol keeps sending liveness probes on that connection, at a
pace the transport discovers, so that the transport can report a worker that stops acknowledging.
An attempt that hangs while its worker's transport keeps acknowledging stops sending progress
reports, and the next other worker that can take its job and asks for a lease receives a replacing
attempt, judged from the intervals between the first worker's own reports, or between the reports
of every worker of its render profile until the first worker's own show a variation above zero.
A lease request that the host can't answer with a job at once waits at the host, and the host
judges the hung attempt again on each liveness probe it sends and each message it receives, so
the waiting worker receives the replacing attempt without asking again.
Render work and its answers travel over the job protocol's connections. Inputs and results travel
over the content store's connections, whose pairing and encryption rules RFC 0008 states.

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

Each connection of the job protocol MUST be authenticated and encrypted on every network.

### EU-0006-058 {#EU-0006-058}

A computer that isn't one of the owner's paired computers MUST NOT be given render work.

### EU-0006-059 {#EU-0006-059}

An accepted render job MUST NOT be dropped for want of capacity.

### EU-0006-079 {#EU-0006-079}

A worker's lease on a job MUST end only when the attempt it holds answers or is cancelled, the discovery service reports that worker gone, the connection to that worker closes, or the owner unpairs that worker.

### EU-0006-091 {#EU-0006-091}

Once the progress an attempt reports falls behind what its worker's measured rate of progress predicts by more than the variation that worker's own progress reports have shown, and another present worker renders the job's render profile, the host MUST cancel that attempt and start a replacing attempt of its job on such a worker.

### EU-0006-092 {#EU-0006-092}

While a worker holds a lease on a job, the job protocol MUST close the connection to that worker once the transport reports that the worker stopped acknowledging the data and the liveness probes sent on that connection.

### EU-0006-093 {#EU-0006-093}

While a worker holds a lease on a job, the job protocol MUST send liveness probes on the connection to that worker, at a pace the transport discovers from that connection rather than at a fixed interval.

### EU-0006-094 {#EU-0006-094}

When a present worker that renders a job's render profile and doesn't hold that job's running attempt asks for a lease, and the time since that attempt started or last reported progress, whichever is later, exceeds the largest of the attempt's measured report intervals by more than their population standard deviation, the host MUST offer that job to the asking worker at the job's priority level and cancel that attempt once a replacing attempt starts. An attempt's measured report intervals are its worker's report intervals for the job's render profile once their population standard deviation is above zero, which takes at least two unequal intervals, and otherwise the report intervals of every worker for that render profile once their population standard deviation is above zero, each counting the time from an attempt's start to its first progress report as an interval.

### EU-0006-095 {#EU-0006-095}

A lease request that the host can't answer with a job when it arrives MUST wait at the host until the host offers it a job, the same worker sends a newer lease request, which takes its place, the discovery service reports that worker gone, the connection to that worker closes, or the owner unpairs that worker.

### EU-0006-096 {#EU-0006-096}

While lease requests wait at the host, the host MUST offer a job whose running attempt meets the condition of EU-0006-094 to one waiting request of a worker that EU-0006-094 would offer it to, chosen by the priority levels of EU-0006-052 and the fair shares of EU-0006-053, on the first event the host handles after that attempt meets the condition, where an event is a liveness probe the job protocol sends under EU-0006-093, or a progress report, an answer or a lease request the host receives.

### EU-0006-097 {#EU-0006-097}

The host MUST answer a lease request, and handle each event of EU-0006-096, with work that doesn't grow with the report intervals, the waiting lease requests, the render profiles other than those that the asking worker or the worker whose request it answers renders, or the running attempts other than those that the reporting worker or the worker whose request it answers holds.
