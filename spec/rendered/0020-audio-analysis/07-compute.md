## Running analysis {#compute}

Analysis runs on workers, apart from the process that drives live audio. The session host prepares
each request and keeps authority over the project; a worker decodes the input, runs the method in
isolation and publishes a candidate result, which the session host validates before it links the
result to a revision of the project. The live audio path only emits meter observations it prepared
ahead of time and reads control curves that were already admitted. An offline analysis can read the
whole song; a streaming method declares the context it reads, the lookahead it needs and its
delay, and its conformance tests compare the declared delay with the delay they measure. While
recording continues, musical inference waits until the take ends and reads the whole take. A browser
shows results the host computed, and
measures material it already holds only where its runtime passed the conformance tests; a phone used
as a controller doesn't run analysis for that reason alone. Analysis is admitted against
the capacity the system measures, and each job's completion records its measurements. The relative
cost of a method varies with the channels, the sample rate, the duration, the hardware, the model
and its precision, so it is measured on each job rather than assumed.

### EU-0020-063 {#EU-0020-063}

Analysis MUST NOT run in the process that drives live audio.

### EU-0020-064 {#EU-0020-064}

Analysis MUST NOT interrupt recording or playback.

### EU-0020-065 {#EU-0020-065}

Where live meter observations overflow, the system MUST record a gap in their coverage instead of holding up the audio.

### EU-0020-066 {#EU-0020-066}

A worker MUST NOT change a project.

### EU-0020-067 {#EU-0020-067}

The session host MUST validate a result's request, source identities, schema, authority and completeness before it links the result to a project revision.

### EU-0020-068 {#EU-0020-068}

A streaming method MUST declare the context it reads, the lookahead it needs and its delay.

### EU-0020-069 {#EU-0020-069}

The system MUST refuse a streaming method whose declaration lacks its context, its lookahead or its delay.

### EU-0020-070 {#EU-0020-070}

The conformance tests of a streaming method MUST compare its declared delay with the delay they measure.

### EU-0020-071 {#EU-0020-071}

While recording continues, musical inference SHOULD wait until the take ends and read the whole take.

### EU-0020-072 {#EU-0020-072}

An analysis job MUST resume after an interruption of its machine without losing its accepted work.

### EU-0020-073 {#EU-0020-073}

Cancelling an analysis job MUST leave every accepted edit as it was.

### EU-0020-074 {#EU-0020-074}

Each job's completion MUST record its time for each second of audio, its peak memory, the bytes it transferred, the storage it added, its wait in the queue and the audio deadlines missed while it ran.

### EU-0020-075 {#EU-0020-075}

The system MUST admit analysis work against the capacity it measures on each machine.

### EU-0020-076 {#EU-0020-076}

The system MUST NOT impose a fixed limit on the number of workers, the duration of audio or the number of jobs.

### EU-0020-077 {#EU-0020-077}

A load failure or memory pressure MUST lower the capacity the system offers while keeping the queued work.

### EU-0020-078 {#EU-0020-078}

A browser MUST run a measurement of its own material only where its runtime has passed the conformance tests.
