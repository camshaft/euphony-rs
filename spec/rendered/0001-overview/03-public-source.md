## Public source and publication {#public-source}

Euphony's source, documentation and release history are public, under a permissive license that
stays as it is. A publication is a push of commits or history to the public repository, or a
release. A file is upstream-derived where it carries or adapts code of another project. A provenance
record of an upstream-derived file states the upstream project, the source path, the pinned commit,
the identity of the local file, a summary of the local changes and the copyright or permission
notice that applies. The pinned upstream inventory lists the upstream sources the project compares
new files with, and the rules of that comparison. The publication check reads a publication before
it is pushed. A comparison that finds similar code doesn't prove that a file is original, and its
report states what it covered and what it can't detect. Deleting a file from the final tree doesn't
remove it from the history.

### EU-0001-020 {#EU-0001-020}

The public source, documentation and release history of euphony MUST hold only material authorized for public distribution.

### EU-0001-021 {#EU-0001-021}

Building and running euphony from its public source MUST NOT need a private package or a private service.

### EU-0001-022 {#EU-0001-022}

Each upstream-derived file MUST have a provenance record before it is published.

### EU-0001-023 {#EU-0001-023}

Each upstream-derived file MUST carry the copyright or permission notice its upstream requires.

### EU-0001-024 {#EU-0001-024}

The publication check MUST read the whole history and every generated file of a publication before it is pushed.

### EU-0001-025 {#EU-0001-025}

The publication check MUST refuse a publication that holds an upstream-derived file whose provenance record is missing or doesn't match the file.

### EU-0001-026 {#EU-0001-026}

The publication check MUST treat as upstream-derived each file whose comparison with the pinned upstream inventory finds a derivation.

### EU-0001-027 {#EU-0001-027}

The publication check MUST refuse a publication while a required comparison is incomplete or a comparison finding is unresolved.

### EU-0001-028 {#EU-0001-028}

A change to upstream-derived code MUST update the summary of local changes in its provenance record.
