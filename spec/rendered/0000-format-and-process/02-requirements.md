## Sections, requirements and identifiers {#requirements}

A section file holds one section table, with the fields anchor, title, problem, motivation and prose,
and then one requirement table for each requirement. The fields of a requirement are id, label,
level, polarity, statement, conditions, compose, parts, relations, logic, params, binds_to and
rationale. The level is one of the levels the configuration file lists, from the strongest, through
the recommended, to the permissive, and the polarity is required or forbidden. A requirement binds
to the system, or to the process by which the specification itself changes. The label can change;
the identifier never does. A key word is written in capitals. An identifier is the prefix, a hyphen, the four-digit number of the RFC
that first held the requirement, a hyphen and a three-digit sequence number. The anchor of a section
is unique in its RFC and stays the same when its title changes. The rationale says why a requirement
exists, and the check doesn't read it. An RFC of the system group states where its component
is going, as a person or another component sees it, and leaves how to build it to each
implementation.

### EU-0000-013 {#EU-0000-013}

Each section MUST state the problem it solves and the motivation for its requirements.

### EU-0000-014 {#EU-0000-014}

Each requirement MUST have an identifier made of the prefix, the number of the RFC that first held it, and a sequence number.

### EU-0000-015 {#EU-0000-015}

A requirement MUST keep its identifier for its life, including after it moves to another RFC.

### EU-0000-016 {#EU-0000-016}

An identifier MUST NOT be held by two requirements, at the same time or one after the other.

### EU-0000-017 {#EU-0000-017}

An RFC MUST list in its retired list the identifier of each requirement it removes.

### EU-0000-018 {#EU-0000-018}

A requirement's statement MUST hold exactly one key word.

### EU-0000-019 {#EU-0000-019}

The key word of a statement MUST match the requirement's level and polarity.

### EU-0000-020 {#EU-0000-020}

A requirement's statement MUST state exactly one obligation.

### EU-0000-021 {#EU-0000-021}

A statement MUST NOT describe how any implementation meets it.

### EU-0000-022 {#EU-0000-022}

A statement MUST NOT name a product, vendor, library or technology where it can name the category.

### EU-0000-023 {#EU-0000-023}

An RFC MUST NOT state which requirements any implementation builds.

### EU-0000-024 {#EU-0000-024}

An RFC MUST NOT offer alternatives for an implementer to choose between.

### EU-0000-025 {#EU-0000-025}

Each RFC of the system group MUST state the end state of its component, whether or not an implementation has reached it.

### EU-0000-026 {#EU-0000-026}

The prose of a section MUST NOT hold a key word.
