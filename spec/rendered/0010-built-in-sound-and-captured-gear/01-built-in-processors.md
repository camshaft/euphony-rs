## Built-in instruments and effects {#built-in-processors}

euphony runs only its own built-in processors: instruments, which turn events into audio, and
effects, which turn audio into audio. Each processor declares a contract: its typed parameters with
their ranges and units, how automation of each parameter takes effect, its latency, its tail after
the input stops, its supported sample rates, how it handles stereo, how it resets its state, and
which numerical results it reproduces. Each processor has a version, and a saved project names the
version of each processor it uses. A replacement is a built-in processor written for an instrument
or effect that the musician lists, and the musician accepts it against listening and signal tests
they choose. euphony doesn't promise an indistinguishable copy of an unspecified commercial effect.
A comparison with another product uses only a reference that its license permits and a method that
a legal review accepts.

### EU-0010-001 {#EU-0010-001}

Each built-in processor MUST declare its typed parameters, with the range and the unit of each.

### EU-0010-002 {#EU-0010-002}

Each built-in processor MUST declare how automation of each of its parameters takes effect.

### EU-0010-003 {#EU-0010-003}

Each built-in processor MUST declare its latency and its tail.

### EU-0010-004 {#EU-0010-004}

Each built-in processor MUST declare the sample rates it supports.

### EU-0010-005 {#EU-0010-005}

Each built-in processor MUST serialize its state so that a saved project restores it.

### EU-0010-006 {#EU-0010-006}

Each built-in processor MUST have a version that a saved project names.

### EU-0010-007 {#EU-0010-007}

A saved project MUST keep playing the processor version it names after a newer version exists.

### EU-0010-008 {#EU-0010-008}

A change that alters the output of a processor for any input MUST produce a new processor version.

### EU-0010-009 {#EU-0010-009}

A new processor version MUST have a cache identity distinct from that of every earlier version.

### EU-0010-010 {#EU-0010-010}

Moving a project to a new processor version MUST be an explicit edit that the musician can undo.

### EU-0010-011 {#EU-0010-011}

A move of a project to a new processor version MUST offer the musician a comparison of the sound before and after the move.

### EU-0010-012 {#EU-0010-012}

The engine MUST NOT load a processor that is not built into it.

### EU-0010-013 {#EU-0010-013}

For each instrument or effect the musician lists for replacement, euphony MUST offer a built-in processor that meets the listening and signal tests the musician accepts for it.

### EU-0010-014 {#EU-0010-014}

The code of each built-in processor MUST be distributable under the license of the code of euphony.

### EU-0010-015 {#EU-0010-015}

A comparison of a built-in processor with another product MUST NOT use a reference whose license does not permit that comparison.

### EU-0010-016 {#EU-0010-016}

euphony MUST record each listed instrument or effect that no accepted built-in processor replaces yet.
