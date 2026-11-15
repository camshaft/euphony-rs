## Changing the set {#change-process}

A change to the set is a pull request against the default branch of the repository. It carries the
edits of the sources and the files the renderer writes from them. A reviewer other than its writer
reads it for consistency, for the format this RFC states, and for the direction of the system, and
the check runs on its last commit. Once the reviewers have approved that commit and the check has
passed on it, the writer merges the change, and the change waits on no other approval.

### EU-0000-047 {#EU-0000-047}

A change to the set MUST be a pull request against the default branch.

### EU-0000-048 {#EU-0000-048}

A pull request that changes the set MUST carry its source edits together with the files rendered from them.

### EU-0000-049 {#EU-0000-049}

A change MUST be reviewed by a reviewer other than its writer before it merges.

### EU-0000-050 {#EU-0000-050}

The writer of a change MUST NOT approve it.

### EU-0000-051 {#EU-0000-051}

A change MUST merge only once its check has passed on its last commit.

### EU-0000-052 {#EU-0000-052}

A change MUST merge only once each reviewer it asked has approved its last commit.

### EU-0000-053 {#EU-0000-053}

The writer of a reviewed change whose check passed MAY merge it.

### EU-0000-054 {#EU-0000-054}

A reviewed change whose check passed MUST NOT wait on an approval other than its reviews.

### EU-0000-055 {#EU-0000-055}

A change to the set MUST NOT wait on a build of the code of any implementation.

### EU-0000-056 {#EU-0000-056}

A change to a contract requirement MUST be additive or carry a new contract version with a migration path.

### EU-0000-057 {#EU-0000-057}

Before an RFC first merges, its writer MUST carry into it each change its sources had since they were read.
