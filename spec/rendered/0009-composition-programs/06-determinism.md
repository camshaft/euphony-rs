## Reproducing and reusing output {#determinism}

The identity of the output of a program is everything that made it: the text of the program, each
asset it imports, each musical input it reads, its seed, and the version of the runtime, the type
library, each definition, each tuning, each rounding policy and the numerical profile it ran under.
Two runs with the same identity
on compatible computers produce the same events, and a stored output with that identity is reused
instead of run again; reused output then feeds the ordinary rendering and its cache. Every random
source of the runtime is a specified algorithm with an explicit seed and a version, so its output
doesn't depend on the computer, on the version of a dependency or on how the seed was written. The
output of a program is put in canonical form before it is proposed. After a regeneration, only the
passages whose output changed are loaded again.

### EU-0009-062 {#EU-0009-062}

Two runs of a program with the same text, inputs, seed and runtime profile MUST produce the same musical events on compatible computers.

### EU-0009-063 {#EU-0009-063}

Every random source of the composition runtime MUST be a specified algorithm with an explicit seed and a version.

### EU-0009-064 {#EU-0009-064}

The output of a random source of the composition runtime MUST NOT depend on the computer that runs it.

### EU-0009-065 {#EU-0009-065}

The output of a random source of the composition runtime MUST NOT depend on how its seed was written.

### EU-0009-066 {#EU-0009-066}

The identity of the output of a program MUST cover its text, each asset it imports, each musical input it reads and its seed, and the versions of the runtime, the type library, each definition, each tuning, each rounding policy and the numerical profile.

### EU-0009-067 {#EU-0009-067}

A stored output whose identity matches a requested run MUST be reused instead of run again.

### EU-0009-068 {#EU-0009-068}

The output of a program MUST be in canonical form before it is proposed to the project.

### EU-0009-069 {#EU-0009-069}

A regeneration MUST load again only the passages whose output changed.
