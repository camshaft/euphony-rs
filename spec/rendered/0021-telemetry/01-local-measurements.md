## Measurements kept on the computer {#local-measurements}

Each process of euphony records typed telemetry events on the computer it runs on, into a telemetry
journal that is separate from the history of every project. An event belongs to one family, and each
family is a closed set of fields. The families are deadline misses, render timings, resource samples,
device latencies, crashes, errors, tool calls and exposure. The component that records an event is
one of a closed set: the session host, the audio device process, a render worker, the composition
program runner, the assistant adapter, a station client, a network MIDI host adapter, an analysis
worker and a control surface adapter. Each event carries a reference to its local project and
session, which stays on the computer. The trail of a crash is the events its process recorded
before it ended. Exposure records intervals of use, so that a count of failures can be read against
how much the failing work was used, and says for each family whether recording was complete over
the interval. Each paired computer keeps its own journal, and the performance and problems view
reads the journals of the owner's paired computers over the owner's authenticated connections.
Measures are split only by closed values, while the identities of jobs, devices and diagnostics
stay attributes of the local event.

### EU-0021-001 {#EU-0021-001}

Each process of euphony MUST record its telemetry events into a telemetry journal on the computer it runs on.

### EU-0021-002 {#EU-0021-002}

The telemetry journal MUST be kept apart from the history of every project.

### EU-0021-003 {#EU-0021-003}

The telemetry journal MUST NOT hold project state.

### EU-0021-004 {#EU-0021-004}

Each telemetry event MUST carry its family, the public schema version, the release version, the build identity, the component that recorded it, the computer's monotonic time and its local project and session reference.

### EU-0021-005 {#EU-0021-005}

Each field that a measure is split by MUST take its value from a closed set that the public schema lists.

### EU-0021-006 {#EU-0021-006}

The identity of a job, a device or a diagnostic MUST NOT be a field that a measure is split by.

### EU-0021-007 {#EU-0021-007}

Each missed audio deadline MUST be recorded as a deadline miss event that states the device period, the buffer size, the part that missed, how late it was, and whether the output went silent or played a prepared result.

### EU-0021-008 {#EU-0021-008}

Recording a deadline miss MUST NOT allocate memory, take a lock or touch a file in the audio callback.

### EU-0021-009 {#EU-0021-009}

Each render, bounce, export, model fitting or analysis job MUST be recorded as a render timing event that states its job kind, frames, wall time, processor time, the worker's render profile class and whether the cache already held its result.

### EU-0021-010 {#EU-0021-010}

Each component MUST record a resource sample of its processor time, peak and steady resident memory, disk bytes read and written, temporary bytes and network bytes when a job completes, when transport starts or stops, when the operating system signals memory or storage pressure, and when a session ends.

### EU-0021-011 {#EU-0021-011}

A component MUST NOT record a resource sample on a timer.

### EU-0021-012 {#EU-0021-012}

Each measured round trip, jitter and clock-lock change of an audio interface, a network MIDI host, a headphone station, a shared-session participant or a control surface MUST be recorded as a device latency event.

### EU-0021-013 {#EU-0021-013}

Each abnormal end of a process MUST be recorded as a crash event that states the process, the release and build identity, the exception or signal kind from a closed set, the frames of the faulting thread that lie in euphony's own public code as module and offset, the trail, the uptime and the computer's hardware class.

### EU-0021-014 {#EU-0021-014}

A crash of the session host itself MUST be recorded in its telemetry journal at the session host's next start.

### EU-0021-015 {#EU-0021-015}

Each typed failure that an interface of euphony returns MUST be recorded as an error event that states the interface, the operation and the result kind.

### EU-0021-016 {#EU-0021-016}

Each assistant tool call MUST be recorded as a tool call event that states the tool, the result kind, the duration, the schema version and, for an action that needs the owner's approval, whether it waited for that approval and how the owner decided.

### EU-0021-017 {#EU-0021-017}

A tool call event MUST NOT hold the call's arguments or its answer.

### EU-0021-018 {#EU-0021-018}

Each exposure interval MUST state its identity, its session, its start and end, how the session ended, and the use it covers in open hours, transport-active hours, render jobs of each kind and attempts of each tool.

### EU-0021-019 {#EU-0021-019}

Each exposure interval MUST state, for each family, whether recording was on throughout, the count of events recorded, the count lost to storage pressure and the count of batches built for it.

### EU-0021-020 {#EU-0021-020}

An exposure interval MUST NOT hold event content.

### EU-0021-021 {#EU-0021-021}

The performance and problems view MUST show, for each project and session of every paired computer of the owner, the deadline misses on the project's timeline, the render and bounce times, each component's resource use, each device's latency, the crashes with their trails, the errors and the failed tool calls.

### EU-0021-022 {#EU-0021-022}

An assistant that the owner connects MUST be able to read the performance and problems view as the owner does.

### EU-0021-023 {#EU-0021-023}

A collaborator, a link participant or a listener endpoint MUST NOT read the performance and problems view.

### EU-0021-024 {#EU-0021-024}

The person MUST be able to turn local recording of each family off.

### EU-0021-025 {#EU-0021-025}

While a family's recording is off, its events MUST NOT be kept.

### EU-0021-026 {#EU-0021-026}

Turning off the recording of deadline misses MUST NOT remove the visible indication of a missed deadline.

### EU-0021-027 {#EU-0021-027}

Local recording of every family MUST be on in a new installation.

### EU-0021-028 {#EU-0021-028}

The telemetry journal MUST keep events for as long as the person's retention setting allows.

### EU-0021-029 {#EU-0021-029}

Under storage pressure, the telemetry journal MUST yield space before any cached result or original.

### EU-0021-030 {#EU-0021-030}

When the telemetry journal yields space, it MUST remove the oldest events first and keep one summary for each release.

### EU-0021-031 {#EU-0021-031}

The performance and problems view MUST say how far back its record reaches.
