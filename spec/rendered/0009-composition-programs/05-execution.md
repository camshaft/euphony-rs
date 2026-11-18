## Running a program {#execution}

A program runs in a restricted runner. The runner reads only the objects the program declares and
the deterministic random and time services the host supplies. It has no access of its own to files
or to the network, and it can't change the project. Once a program finishes, its canonical output
reaches the authority of the project as a proposed change, which the authority validates and
applies through the ordinary commands of the project. A program runs when a musician or an agent
asks, and never on the audio path. Independent composition units, such as separate movements or
tracks whose dependencies are explicit, run as separate jobs across the owner's computers, while a
sequential program stays one job. A worker reports the resources it has, and where they run out,
the authority keeps the paused work and its diagnostics. The owner can cancel a program that runs
away. Every composition command passes the authorization of the caller that sends it before the
runner starts a program for it, and again before the authority applies the program's output. The
browser companion sends authorized composition commands to a native host and doesn't run programs
itself.

### EU-0009-050 {#EU-0009-050}

The runner MUST NOT give a program access to a file or to the network beyond the objects the program declares.

### EU-0009-051 {#EU-0009-051}

A program MUST read randomness and time only from the services the host supplies.

### EU-0009-052 {#EU-0009-052}

A program MUST NOT change the project directly.

### EU-0009-053 {#EU-0009-053}

The output of a program MUST reach the project as a proposed change that the authority validates and applies as an ordinary command.

### EU-0009-054 {#EU-0009-054}

A program MUST NOT gain authority through what it returns or through a capability it asks for.

### EU-0009-055 {#EU-0009-055}

A program MUST NOT run on the audio path.

### EU-0009-056 {#EU-0009-056}

Independent composition units whose dependencies are explicit MUST be able to run as separate jobs on different computers of the owner.

### EU-0009-057 {#EU-0009-057}

A sequential program MUST run as one job.

### EU-0009-058 {#EU-0009-058}

Where the resources of a worker run out, the authority MUST keep the paused work and its diagnostics.

### EU-0009-059 {#EU-0009-059}

The owner of a project MUST be able to cancel a running program.

### EU-0009-060 {#EU-0009-060}

The cancel of a program MUST NOT discard an accepted edit of the project.

### EU-0009-061 {#EU-0009-061}

The browser companion MUST NOT run a composition program itself.

### EU-0009-083 {#EU-0009-083}

A composition command MUST pass the authorization of the caller that sends it before the runner starts a program for it, and again before the authority applies the output of that program.
