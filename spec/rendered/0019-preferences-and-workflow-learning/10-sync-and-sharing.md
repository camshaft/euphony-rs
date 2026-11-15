## Synchronization, sharing and ordering {#sync-and-sharing}

Each owner's learning has a version history of its own, apart from the branches of any project. The
owner's paired devices synchronize it over authenticated, encrypted transfer. The owner can publish
a reviewed copy of a project-specific preference or mechanism with selected evidence removed, and
its recipient sees it as an attributed proposal. An offline device keeps the snapshots it can read
and can make proposals. The owner's subject authority orders the owner's acceptances, withdrawals,
resets, activations, suspensions and retirements within each learning scope: one scope for the
owner-wide preferences, and one for each project, which holds that project's preferences,
mechanisms and learning work. Reads use immutable versions and don't take the authority.

### EU-0019-164 {#EU-0019-164}

Each owner's learning MUST have a version history apart from the branches of every project.

### EU-0019-165 {#EU-0019-165}

The owner's paired devices MUST synchronize that history over an authenticated, encrypted transfer.

### EU-0019-166 {#EU-0019-166}

Sharing a project, a link to it or a digest of its audio MUST NOT include the owner's learning.

### EU-0019-167 {#EU-0019-167}

A published copy of a preference or mechanism MUST reach its recipient as an attributed proposal.

### EU-0019-168 {#EU-0019-168}

A published copy MUST NOT become the recipient's own taste or carry an approval.

### EU-0019-169 {#EU-0019-169}

Euphony MUST authorize every learning record and evidence view at each read and at each delivery to a subscription.

### EU-0019-170 {#EU-0019-170}

Where the existence of evidence would reveal another person's private work, euphony MUST hide its identity and its count as well as its content.

### EU-0019-171 {#EU-0019-171}

An offline device MUST NOT create a new activation or reconcile conflicting approvals by itself.

### EU-0019-172 {#EU-0019-172}

Synchronization MUST preserve concurrent heads, explicit conflict resolutions, signed withdrawals and scope tombstones.

### EU-0019-173 {#EU-0019-173}

A stale device MUST NOT restore a version the owner reset.

### EU-0019-174 {#EU-0019-174}

A device's unpairing MUST apply before euphony serves that device any new context or proposal on its reconnection.

### EU-0019-175 {#EU-0019-175}

The pairing screen MUST state that unpairing can't erase copies a device already received or reach a disconnected device at once.

### EU-0019-176 {#EU-0019-176}

The approval of a share MUST state that a recipient's copy can't be recalled and that an offline device can't be revoked at once.

### EU-0019-177 {#EU-0019-177}

The owner's subject authority MUST order the owner's acceptance, withdrawal, reset, activation, suspension and retirement changes within each learning scope.

### EU-0019-178 {#EU-0019-178}

Writes in two projects of the same owner MUST NOT wait on each other.

### EU-0019-179 {#EU-0019-179}

A project-scope write that depends on an owner-wide preference MUST be refused as stale where the owner-wide revision it read has moved.
