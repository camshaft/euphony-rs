## Key constraints and diagnostics {#key-constraints}

A phrase can declare a key. In such a phrase an ordinary note is written by its scale degree in that
key, and a note outside the key is written through an explicit chromatic constructor, which marks it
as intended. A note outside a key is an error only where a phrase declared that key; an accidental
is not wrong by itself. Checks run at three points: the type check before a program runs, which
also reports each invalid literal value; the constructors while it runs, for the values it computes
or reads; and the validation of its output by the project before the output is accepted. The type
check alone is never the trust boundary. A diagnostic names the span of the program text where the
problem is, the musical value at fault and the relationship the program was expected to keep.

### EU-0009-024 {#EU-0009-024}

A phrase MUST be able to declare a key that it is constrained to.

### EU-0009-025 {#EU-0009-025}

In a phrase that declares a key, an ordinary note MUST be built from a scale degree of that key.

### EU-0009-026 {#EU-0009-026}

An explicit chromatic constructor MUST let a program write a note outside the declared key of its phrase.

### EU-0009-027 {#EU-0009-027}

An ordinary note that breaks the declared key of its phrase MUST fail validation.

### EU-0009-028 {#EU-0009-028}

A note MUST NOT be reported as outside a key unless its phrase declares that key.

### EU-0009-029 {#EU-0009-029}

A composition program MUST be type-checked before it runs.

### EU-0009-030 {#EU-0009-030}

The type check of a program MUST report each invalid literal musical value it holds.

### EU-0009-031 {#EU-0009-031}

A constructor called while a program runs MUST validate the value it builds.

### EU-0009-032 {#EU-0009-032}

The project MUST validate each musical value of the output of a program before it accepts the output.

### EU-0009-033 {#EU-0009-033}

A program whose output fails validation MUST NOT change the project.

### EU-0009-034 {#EU-0009-034}

A diagnostic of a program MUST name the span of the program text where the problem is.

### EU-0009-035 {#EU-0009-035}

A diagnostic of a program MUST name the musical value at fault.

### EU-0009-036 {#EU-0009-036}

A diagnostic of a program MUST name the musical relationship the program was expected to keep.
