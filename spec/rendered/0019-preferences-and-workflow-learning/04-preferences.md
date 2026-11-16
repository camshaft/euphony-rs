## Preference versions and conflicts {#preferences}

A preference version holds the subject person, a property with a versioned domain, a typed desired
value or declared range, context predicates for project, genre, client and session, a strength, a
proposed confidence and its stated basis, the evidence that supports and contradicts it, its
proposer, the versions it supersedes, the time the person last confirmed it and the approval that
adopted it. The strength is tentative, preferred or required. A free-text explanation accompanies
the typed fields. A property whose domain the store doesn't support stays an inspectable note. An
assistant proposes a preference with its evidence, and the proposal waits for the owner's decision.
Where approved preferences of the project, the client and the genre overlap and the owner hasn't
approved a rule of precedence between them, the overlap is a conflict that the owner resolves.

### EU-0019-045 {#EU-0019-045}

A preference version MUST hold a typed value or a declared range of a property whose domain is versioned.

### EU-0019-206 {#EU-0019-206}

A preference version MUST hold its context predicates for project, genre, client and session, its proposed confidence with its stated basis, its proposer and the approval that adopted it.

### EU-0019-046 {#EU-0019-046}

A preference version MUST link the evidence that supports it and the evidence that contradicts it.

### EU-0019-047 {#EU-0019-047}

The strength of a preference version MUST be tentative, preferred or required.

### EU-0019-048 {#EU-0019-048}

A preference whose domain the store doesn't support MUST NOT be applied as executable policy.

### EU-0019-049 {#EU-0019-049}

The last confirmation time of a preference MUST change only when the person confirms it.

### EU-0019-050 {#EU-0019-050}

A required preference MUST NOT relax an access, safety, rights or project requirement.

### EU-0019-051 {#EU-0019-051}

Proposing a preference MUST carry its evidence versions, the expected learning revision and an idempotency key.

### EU-0019-052 {#EU-0019-052}

Proposing a preference MUST return the proposal and a reviewable diff, or a typed refusal of denied, missing authorized evidence, invalid domain, conflict or unavailable storage.

### EU-0019-053 {#EU-0019-053}

Euphony MUST validate a proposal's subject, schema, evidence access and typed relations before it stores the proposal.

### EU-0019-198 {#EU-0019-198}

A typed relation MUST NOT grant access.

### EU-0019-054 {#EU-0019-054}

A stored preference version MUST NOT change.

### EU-0019-055 {#EU-0019-055}

A proposal that narrows or replaces a preference MUST link the version it supersedes.

### EU-0019-056 {#EU-0019-056}

Contradicting evidence MUST be kept with its context.

### EU-0019-057 {#EU-0019-057}

The preferences of two subjects MUST stay independent of each other.

### EU-0019-058 {#EU-0019-058}

An accepted session override MUST take precedence only within the scope it declares.

### EU-0019-059 {#EU-0019-059}

Overlapping approved preferences without an approved rule of precedence between them MUST yield an unresolved conflict.

### EU-0019-060 {#EU-0019-060}

Euphony MUST NOT resolve a conflict between preferences by ranking or averaging them.

### EU-0019-061 {#EU-0019-061}

While a conflict between preferences is unresolved, euphony MUST NOT apply a behavior other than the last approved behavior that still applies.

### EU-0019-196 {#EU-0019-196}

While a conflict between preferences is unresolved, euphony MAY present the alternatives to the owner.

### EU-0019-062 {#EU-0019-062}

Euphony MUST NOT change a persistent default to resolve a conflict between preferences.

### EU-0019-063 {#EU-0019-063}

The age of a preference MUST NOT change an approved preference.

### EU-0019-064 {#EU-0019-064}

Euphony MUST show the age of a preference as uncertainty.
