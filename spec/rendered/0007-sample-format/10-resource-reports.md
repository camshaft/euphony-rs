## Resource reports and release refusals {#resource-reports}

Every release shows that its optimizations save resources without changing the sound or missing
playback deadlines. Euphony reduces repeated processing, copies and retained bytes before it adds
machines. Each optimization states the resource it targets and the workload it is measured on, and
the report compares the same project, software profile, hardware and failure scenario before and
after it. A report holds measured values alone: a compression ratio or a memory target is never
set before measurement, and the sample format alone doesn't promise a saving. Where an
optimization trades one resource for another, the report states the other resources beside the
targeted one.

### EU-0007-110 {#EU-0007-110}

Each release MUST hold a resource report of processor time for rendering, encoding and decoding, peak and steady resident memory, allocation and copy counts, persistent and temporary disk bytes, transferred bytes, cache reuse and playback underruns.

### EU-0007-111 {#EU-0007-111}

A resource report MUST compare the same project, software profile, hardware and failure scenario before and after each optimization.

### EU-0007-112 {#EU-0007-112}

The workloads of a resource report MUST include cold and warm playback, edits, many listeners, spatial outputs, resumed wide-area sync and high-entropy audio.

### EU-0007-113 {#EU-0007-113}

A resource report MUST state only measured values.

### EU-0007-114 {#EU-0007-114}

Each optimization's change record MUST state its targeted resource, one of memory, processor work, disk and transfer.

### EU-0007-115 {#EU-0007-115}

Each optimization's change record MUST state its workload, from the representative projects the project configures.

### EU-0007-116 {#EU-0007-116}

The build MUST NOT accept an optimization's change record that lacks its targeted resource or its workload.

### EU-0007-117 {#EU-0007-117}

A resource report MUST state the change of each other resource beside the targeted one.

### EU-0007-118 {#EU-0007-118}

The release step MUST NOT publish a commit that has no resource report for that exact commit.

### EU-0007-119 {#EU-0007-119}

The release step MUST NOT publish a commit whose resource report shows a missed playback deadline.

### EU-0007-120 {#EU-0007-120}

The release step MUST NOT publish a commit whose resource report shows a changed decoded sample.

### EU-0007-121 {#EU-0007-121}

The release step MUST NOT publish a commit whose resource report doesn't show a reduction of each resource its optimizations target on their workloads.
