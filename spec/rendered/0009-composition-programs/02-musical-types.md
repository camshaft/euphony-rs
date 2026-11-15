## Musical types {#musical-types}

The musical model of the project defines each kind of musical value as a type of its own: pitch
spelling, sounding pitch, interval with its quality and direction, scale degree with the identity of
its scale, key, chord, meter, tempo, musical position, duration in whole notes, a count of tempo
beats with its beat unit, frequency in hertz, frequency ratio and duration in samples. A program is
checked against types that define the same kinds and rules as that model, and builds each value
through a constructor that validates it. Musical positions and durations are exact rational
numbers, each held reduced with its sign on the numerator. Scales and tunings, the built-in ones and
those a user defines, are versioned data of the project. Each value is encoded with a tag of its
kind, and each rational member as an integer of any size.

### EU-0009-010 {#EU-0009-010}

The musical model MUST define each kind of musical value that this section lists as a distinct type.

### EU-0009-011 {#EU-0009-011}

An operation MUST NOT accept a musical value of a kind it is not defined for, such as an interval added to a duration.

### EU-0009-012 {#EU-0009-012}

The types a program is checked against MUST define the same kinds of value and the same rules as the musical model of the project.

### EU-0009-013 {#EU-0009-013}

Each constructor of a musical value MUST reject a value that breaks the rules of its kind with a typed error.

### EU-0009-014 {#EU-0009-014}

Musical positions and durations MUST be exact rational numbers.

### EU-0009-015 {#EU-0009-015}

A rational musical value MUST be held reduced, with its sign on the numerator.

### EU-0009-016 {#EU-0009-016}

A rational musical value MUST NOT have a denominator of zero.

### EU-0009-017 {#EU-0009-017}

An arithmetic operation on musical values MUST return its exact result or a typed error.

### EU-0009-018 {#EU-0009-018}

An arithmetic operation on musical values MUST apply its operands in the order the program writes them.

### EU-0009-019 {#EU-0009-019}

The encoding of a musical value MUST carry a tag of its kind.

### EU-0009-020 {#EU-0009-020}

The encoding of a rational musical value MUST hold its numerator and denominator as integers of any size that every client reads without loss.

### EU-0009-021 {#EU-0009-021}

Scales and tunings MUST be versioned data of the project.

### EU-0009-022 {#EU-0009-022}

A user MUST be able to define a scale or a tuning of their own.

### EU-0009-023 {#EU-0009-023}

Pitch spelling and interval quality MUST be kept apart from sounding pitch.
