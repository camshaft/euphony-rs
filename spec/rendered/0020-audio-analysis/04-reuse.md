## Reuse, invalidation and staleness {#reuse}

The identity of a request is everything that can change its answer: the source asset or the
description of the render that made it, the decoded sample format and channel map, the interval,
the method and its version, its parameters, the version of each decoding and preprocessing step,
the model and its digest, the seed where the method uses one, the execution profile it runs under,
and its context. Each method declares the context it needs: a whole-song estimate of key or
structure depends on the whole song, not only the passage shown, and context can include selected
stems, preceding state and the version of a beat grid. A method can declare that its result over a
window depends on that window alone, and conformance tests show that the declaration holds; the
result of such a window can then be reused inside a larger request. A change of hardware or of
model that can change a method's answers is a new execution profile. A result whose dependencies
changed is stale: it stays visible as stale until someone asks for it to be computed again. An
existing project keeps the results it accepted, and the methods that made them, so that a newer
method doesn't change how it sounds.

### EU-0020-032 {#EU-0020-032}

The identity of a request MUST include every input that can change its result.

### EU-0020-033 {#EU-0020-033}

Each method MUST declare the context its results depend on beyond the analyzed interval.

### EU-0020-034 {#EU-0020-034}

A request whose identity matches an earlier one MUST reuse the validated result of that request instead of running again.

### EU-0020-035 {#EU-0020-035}

A change to any input in a request's identity MUST invalidate each result that depends on that input.

### EU-0020-036 {#EU-0020-036}

The system MUST NOT reuse the result of a window inside a larger request unless its method declares that its results over a window depend on that window alone.

### EU-0020-037 {#EU-0020-037}

A change of hardware or of model that can change a method's answers MUST be treated as a new execution profile.

### EU-0020-038 {#EU-0020-038}

For a method whose runs can differ, the system MUST keep the exact result it produced, identified by its digest.

### EU-0020-039 {#EU-0020-039}

A stale result MUST remain inspectable and marked as stale.

### EU-0020-040 {#EU-0020-040}

The system MUST NOT recompute a stale result unless someone asks for it to be recomputed.

### EU-0020-041 {#EU-0020-041}

A new method or method version MUST NOT rewrite an existing result, correction or accepted interpretation.

### EU-0020-042 {#EU-0020-042}

Each accepted result and its dependencies MUST be retained for as long as the project that accepted the result is retained.

### EU-0020-043 {#EU-0020-043}

An upgrade of a method MUST NOT change the sound of a project that uses the results of an earlier version.
