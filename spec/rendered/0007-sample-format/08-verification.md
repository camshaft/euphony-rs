## Verified codecs, file formats and adapters {#verification}

A codec, a file format and a network audio implementation are adopted only with a typed
verification record, keyed by the exact locked version of the dependency, its source digest and
the target and features it builds for. The record links public evidence and conformance results to
the checks the project configuration requires; authorized maintainers record the evidence, and the
build checks its binding and its passing results. An export adapter target has a record of the
same kind, keyed by the exact version of the adapter, that declares the sample formats, rates and
layouts the target accepts. The choice of working format doesn't by itself show that any codec
saves space or that any algorithm is accurate enough.

### EU-0007-087 {#EU-0007-087}

Each adopted codec, file-format and network audio dependency MUST have a typed verification record keyed by its exact locked version, its source digest and its target and features.

### EU-0007-088 {#EU-0007-088}

A verification record MUST link its public evidence and its conformance results to the checks the project configuration requires.

### EU-0007-089 {#EU-0007-089}

The build MUST NOT adopt a dependency whose verification record is missing, failed or doesn't match it.

### EU-0007-090 {#EU-0007-090}

A verification record for one version of a dependency MUST NOT satisfy another version.

### EU-0007-091 {#EU-0007-091}

Each export adapter target MUST have a verification record keyed by the adapter's exact version that declares the sample formats, rates and layouts it accepts.

### EU-0007-092 {#EU-0007-092}

Export MUST NOT deliver to an adapter target whose record is missing, failed or doesn't declare the formats the delivery needs.

### EU-0007-093 {#EU-0007-093}

The export preview MUST list, from the adapter's record, each format the delivery needs that the record doesn't declare.

### EU-0007-122 {#EU-0007-122}

The evidence of a verification record MUST be recorded by an authorized maintainer.

### EU-0007-123 {#EU-0007-123}

The build MUST NOT accept a verification record whose evidence was recorded by a principal that is not an authorized maintainer.
