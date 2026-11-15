## The session host {#session-host}

The session host is the daemon of one session. It starts when a client opens a local copy of a
project, and owns that copy's journal, signed revision graph, branch references, render manifest and
render scheduler, together with the session's transport and state; the audio device process owns the
audio stream. It publishes typed events to the clients attached to it, over the local endpoint or
over an authenticated connection from a peer. Every collaborator runs a session host for their own
copy and edits their own branch offline. Authority is local to a working branch and a playback
device, and shared history doesn't need a central leader. The session host reads its configuration
from project and device records: peer enrollment, the clock domain, devices, negotiated profiles,
storage policy, allowed processors, render priorities, pairings, links, source imports and script
profiles. The journal, the revision graph and the commands are specified in the RFC of the project
model, and the render scheduler in the RFC of distributed rendering.

### EU-0002-014 {#EU-0002-014}

The session host MUST run as a process apart from every client.

### EU-0002-015 {#EU-0002-015}

The session host MUST be the only writer of its session's project state.

### EU-0002-016 {#EU-0002-016}

The session host MUST own the journal, the revision graph, the branch references and the render manifest of its copy of the project.

### EU-0002-017 {#EU-0002-017}

The session host MUST keep running whether or not a client is attached to its session.

### EU-0002-018 {#EU-0002-018}

The session host MUST publish each change of its session's state to its attached clients as a typed event.

### EU-0002-019 {#EU-0002-019}

A collaborator's session host MUST accept edits to that collaborator's own branch while no other collaborator's session host is reachable.

### EU-0002-020 {#EU-0002-020}

The loss of one collaborator's session host MUST NOT make another collaborator's copy of the project read-only.

### EU-0002-021 {#EU-0002-021}

Shared project history MUST NOT require a central leader.

### EU-0002-022 {#EU-0002-022}

A lost local journal MUST be restored only from its own backup or from a peer revision the owner explicitly chooses.

### EU-0002-023 {#EU-0002-023}

The session host MUST read its configuration from project and device records.

### EU-0002-024 {#EU-0002-024}

The session host MUST read each project, device and assistant identity from records.

### EU-0002-025 {#EU-0002-025}

The session host MUST emit an operational event for each command acceptance, command conflict, preparation start, preparation completion, preparation failure, cache validation, worker arrival, worker loss, clock state change and underrun.

### EU-0002-026 {#EU-0002-026}

The type of each operational event MUST be defined in the public schema.

### EU-0002-027 {#EU-0002-027}

Each metric of the session host MUST use a closed set of categories.

### EU-0002-028 {#EU-0002-028}

A metric of the session host MUST NOT use a diagnostic identifier as a category.

### EU-0002-029 {#EU-0002-029}

A client MUST NOT have to poll the session host to learn of a change.
