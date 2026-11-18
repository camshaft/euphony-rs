## The host service {#host-service}

Each of the owner's computers runs one host service for the owner's account. The host service
supervises the engine processes of that computer: one session host for each open session, the audio
device process once the owner activates an audio device, the worker role where the owner enrolled
the computer for rendering, and the runners of composition programs. It holds the computer's list of
sessions, its local endpoint for clients on the same computer and its network endpoint for the
owner's other paired computers. The operating system's service manager starts the host service when
a client first connects to its local endpoint. The behavior of the audio device process, of the
worker and of the runners is specified in the RFCs of those components.

### EU-0002-001 {#EU-0002-001}

Each of the owner's computers that runs euphony MUST run the host service as a per-user service of the operating system for the owner's account.

### EU-0002-002 {#EU-0002-002}

The operating system MUST start the host service when a client first connects to its local endpoint.

### EU-0002-003 {#EU-0002-003}

The host service MUST NOT poll for clients or for work.

### EU-0002-004 {#EU-0002-004}

The host service MUST start one session host for each session a client opens.

### EU-0002-005 {#EU-0002-005}

The host service MUST start the audio device process only once the owner activates an audio device.

### EU-0002-006 {#EU-0002-006}

The host service MUST run the worker role only on a computer the owner enrolled for rendering.

### EU-0002-007 {#EU-0002-007}

The host service MUST run each runner of a composition program as a process of its own.

### EU-0002-008 {#EU-0002-008}

The host service MUST hold the list of the sessions open on its computer.

### EU-0002-009 {#EU-0002-009}

The host service MUST hold its computer's local endpoint and network endpoint.

### EU-0002-010 {#EU-0002-010}

A failure of the host service MUST NOT stop a running session host or audio device process.

### EU-0002-011 {#EU-0002-011}

A restarted host service MUST find each running session host and audio device process of its computer through the endpoints they registered.

### EU-0002-012 {#EU-0002-012}

The location of the local endpoint MUST be configurable.

### EU-0002-013 {#EU-0002-013}

By default, the local endpoint SHOULD be at the location the operating system's convention sets for a per-user service.
