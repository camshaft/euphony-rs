## Citing requirements from an implementation {#implementations}

An implementation cites a requirement in its code or its tests with the tracing tool. A citation
names the rendered text of the section with the identifier as its anchor, and quotes the statement,
so that a changed statement shows each citation that quotes the old words. Which requirements an
implementation implements and verifies is read from the tracing tool's report over its citations.
Each implementation decides how it verifies each requirement, by the tests, proofs or other means its
language offers.

### EU-0000-043 {#EU-0000-043}

A citation of a requirement MUST name the rendered text of its section with the identifier as the anchor.

### EU-0000-044 {#EU-0000-044}

A citation of a requirement MUST quote the requirement's statement.

### EU-0000-045 {#EU-0000-045}

An implementation that leaves a requirement unbuilt on purpose MUST say so in its citation of that requirement, with its reason.

### EU-0000-046 {#EU-0000-046}

An implementation's tracing configuration MUST NOT list the rendered text as a specification to extract.
