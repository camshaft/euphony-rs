## Numerical profiles and processor accuracy {#numerical-profiles}

A processor can compute in a wider precision inside, such as for long sums or numerically sensitive
filters, and returns working samples at its interface. Its numerical profile states the rules its
arithmetic follows: internal precision, rounding, fused operations, vector behavior and subnormal
handling. The identity of the profile and the identity of the processor are digests of their
definitions, so a change of either changes the identity, and every cached result that depends on
the processor follows it, without a version label anyone chooses. The binding checks the declared
rules; conformance tests establish that an implementation follows them. The precision of the
working format never stands in for checking an algorithm: each processor shows, against a
higher-precision reference, that its error stays within the bound its test allows, and it can't
render until it has.

### EU-0007-008 {#EU-0007-008}

Each processor MUST declare a numerical profile that states its internal precision, rounding, fused operations, vector behavior and subnormal handling.

### EU-0007-009 {#EU-0007-009}

The identity of a numerical profile MUST be a domain-separated digest of the profile's versioned canonical fields.

### EU-0007-010 {#EU-0007-010}

The identity of a processor MUST cover its numerical profile identity, the digest of its implementation and the versions of its parameters and schema.

### EU-0007-011 {#EU-0007-011}

Registration of a processor MUST refuse a definition whose identities, computed again from the submitted definitions, don't match the identities it declares.

### EU-0007-012 {#EU-0007-012}

Render descriptors and checkpoints MUST keep the processor and profile identities they were made with.

### EU-0007-013 {#EU-0007-013}

A checkpoint MUST keep a processor's state at the precision the processor holds it.

### EU-0007-014 {#EU-0007-014}

A change of a processor's identity MUST invalidate each cached result that depends on it.

### EU-0007-015 {#EU-0007-015}

Registration MUST NOT accept a processor identity whose record lacks a passing higher-precision reference test for that exact identity.

### EU-0007-016 {#EU-0007-016}

A processor's accuracy record MUST hold its identity, the reference precision, the error bound its test allows and the result.

### EU-0007-017 {#EU-0007-017}

The higher-precision reference test of a processor MUST cover cancellation, long sums, low-level decays, filters and nonlinear processing.

### EU-0007-018 {#EU-0007-018}

The release step MUST NOT release a commit that holds an unregistered processor identity.
