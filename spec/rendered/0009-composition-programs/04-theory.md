## Musical theory operations {#theory}

The theory operations receive a tagged musical value, the version of each definition they use and an
explicit conversion context. They validate a value; transpose it; expand a scale degree to a pitch
and collapse a pitch to a scale degree under a declared rounding policy; resolve a pitch to a
frequency through a tuning and a reference frequency; and convert a musical position or duration,
through a revision of the tempo and meter map, to a position or a duration on the sample grid under
a declared rounding. Each operation answers a canonical value, with a report of any quantization it
made, or a typed error: an invalid ratio, an overflow, an unknown definition, an unsupported tuning,
a note out of the scale, an ambiguous interpretation or an incompatible time basis. A duration in
whole notes and a count of tempo beats are different quantities: a quarter note is one tempo beat
only in a meter whose beat unit is the quarter note. Time stays exact while a program composes, and
only the conversion to the sample grid rounds. Every client can read which theory operations and
which definition versions the project supports, and every client sees the same diagnostics and
conversions. A typed error states where its cause is: the span of the program text, where a program
reached the operation, or the field of the command, where a command reached it.

### EU-0009-037 {#EU-0009-037}

Each theory operation MUST receive the version of each definition it uses.

### EU-0009-038 {#EU-0009-038}

Each theory operation MUST receive its conversion context explicitly.

### EU-0009-039 {#EU-0009-039}

The expansion of a scale degree to a pitch and the collapse of a pitch to a scale degree MUST follow a rounding policy that the caller declares.

### EU-0009-040 {#EU-0009-040}

The resolution of a pitch to a frequency MUST use a tuning and a reference frequency that the caller states.

### EU-0009-041 {#EU-0009-041}

A conversion of a musical position or duration to the sample grid MUST read a revision of the tempo and meter map that the caller states.

### EU-0009-042 {#EU-0009-042}

A conversion to the sample grid MUST follow a rounding that the caller declares.

### EU-0009-043 {#EU-0009-043}

A theory operation that rounds a value MUST report the quantization it made.

### EU-0009-044 {#EU-0009-044}

A theory operation MUST answer either a canonical value or a typed error that states its cause.

### EU-0009-045 {#EU-0009-045}

A duration in whole notes MUST NOT be read as a count of tempo beats without the beat unit of the meter.

### EU-0009-046 {#EU-0009-046}

A conversion across a change of tempo MUST integrate the tempo map over the interval it converts.

### EU-0009-047 {#EU-0009-047}

Each musical event a program generates MUST be placed at an exact rational position.

### EU-0009-048 {#EU-0009-048}

A theory operation MUST answer a typed error for an input that is not a finite number.

### EU-0009-049 {#EU-0009-049}

A theory operation MUST NOT grant access to an asset or a device, or a way to run native code.

### EU-0009-080 {#EU-0009-080}

The theory operations MUST include the transposition of a musical value by an interval.

### EU-0009-081 {#EU-0009-081}

Every client MUST be able to discover the theory operations and the definition versions the project supports.

### EU-0009-082 {#EU-0009-082}

A typed error of a theory operation reached through a command MUST identify the field of the command at fault.
