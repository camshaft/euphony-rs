## End state and goals {#end-state}

Euphony prepares the audio of a project ahead of playback, and plays prepared audio as its normal
input. Preparing audio is rendering, and a computer that renders is a worker. The computer that
hosts the project is one of the owner's paired computers and always renders, and each other
computer the owner has paired is offered as a worker. Every worker renders through one contract, so
rendering on the host's own computer is one
worker among the others. A render cache on each computer holds prepared results. Euphony is built
for dozens of the owner's computers rendering one project. Live monitoring, live instruments and
recording stay on the musician's own computer and belong to RFC 0005, and the trust model under
which workers are the owner's paired computers belongs to RFC 0001.

### EU-0006-001 {#EU-0006-001}

The scheduler MUST be able to place a project's render work on every present worker that the owner has paired.

### EU-0006-002 {#EU-0006-002}

Audio from an interrupted, late or superseded render MUST NOT be accepted into the manifest of the current revision.

### EU-0006-004 {#EU-0006-004}

An edit MUST NOT cause a render of audio that neither the edit nor its dependencies change.

### EU-0006-005 {#EU-0006-005}

Results of passages that an edit doesn't change MUST stay valid in the manifest of the new revision.

### EU-0006-006 {#EU-0006-006}

Rendering MUST NOT stop recording or live monitoring on the musician's computer.

### EU-0006-007 {#EU-0006-007}

The loss of a worker MUST NOT lose an accepted edit.

### EU-0006-082 {#EU-0006-082}

The loss of a worker MUST NOT lose a result the host has accepted.

### EU-0006-008 {#EU-0006-008}

Render work that a lost worker left unfinished MUST be offered to another worker.

### EU-0006-009 {#EU-0006-009}

Every worker, the host's own computer included, MUST render through the same versioned render contract.

### EU-0006-010 {#EU-0006-010}

A computer the owner has paired MUST be offered as a worker as soon as it joins the owner's network.
