## Parties and boundaries {#trust-model}

The model has three kinds of party. The owner is the one person whose workstation it is. A device
becomes the owner's by pairing, and is then either an owner device or a sink endpoint. An owner
device, such as a desktop computer, a worker, a phone, a tablet, a browser or the host of a control
surface or of networked instruments, acts as the owner, and so does an assistant the owner connects.
A sink endpoint, such as a headphone station, is bound to one listener output, its sink: it receives
that sink's stream and meters, sends that sink's mix commands and reads that sink's control map.

A collaborator is a person other than the owner, who reaches a project through a share link at one
of three levels. Listen receives the mix of the sink the owner assigns and the transport state, and
adjusts that sink's mix where the sink is the collaborator's. Comment adds annotations. Edit receives
a copy of the project and writes the collaborator's own branches. A collaborator's own devices and
assistants act within that level on the collaborator's side. Everyone else, an unpaired device, a
discovery advertisement, the operators of a signaling or relay service and the network, receives
only the public connection information.

A pairing starts on an owner device or on the local console of the owner's first computer, which
creates a fresh attempt that can be used once; the new device sends its public key into it. The
trusted device shows the new key's fingerprint and a short code derived from the attempt and the key,
the new device shows the same code, and the owner compares them and approves on the trusted device.
Copies a device or a participant already received can't be erased from a distance. The safety
limits on levels are settings that hold for the owner's devices too, not permissions. A real-time
stream is a stream of live audio or MIDI between computers, in an interoperable network format.

### EU-0001-029 {#EU-0001-029}

A device MUST become the owner's only through a pairing the owner approves on a device the engine already trusts.

### EU-0001-030 {#EU-0001-030}

Discovering a device MUST NOT pair it.

### EU-0001-031 {#EU-0001-031}

A pairing approval MUST cover the pairing attempt, the new device's key, its class and, for a sink endpoint, its sink.

### EU-0001-032 {#EU-0001-032}

A pairing MUST take the device's class and sink from the owner's approval alone.

### EU-0001-033 {#EU-0001-033}

A change of a paired device's class or sink MUST need a new pairing that the owner approves.

### EU-0001-034 {#EU-0001-034}

The owner's first computer MUST create the owner's identity on its local console before it accepts a pairing attempt.

### EU-0001-035 {#EU-0001-035}

A paired owner device MUST be able to take every action the owner can take, on every project of the owner.

### EU-0001-036 {#EU-0001-036}

The engine MUST accept an action from a paired owner device or a connected assistant as it accepts the same action from the owner, except an action that waits for the owner's approval.

### EU-0001-037 {#EU-0001-037}

A sink endpoint's key MUST NOT reach any command or object other than its own sink's stream, meters, mix commands and control map.

### EU-0001-038 {#EU-0001-038}

A person other than the owner MUST reach a project only through a share link.

### EU-0001-039 {#EU-0001-039}

A share link MUST have exactly one level of listen, comment or edit.

### EU-0001-040 {#EU-0001-040}

A collaborator MUST NOT take an action that the level of the collaborator's link doesn't allow.

### EU-0001-041 {#EU-0001-041}

Each share link MUST have an expiry.

### EU-0001-042 {#EU-0001-042}

A collaborator's peer MUST NOT receive an object of a project that none of its links reaches.

### EU-0001-043 {#EU-0001-043}

A device that is neither paired with the owner nor connected through a share link, a discovery advertisement, a signaling or relay operator and the network MUST NOT be able to send a command to a session.

### EU-0001-044 {#EU-0001-044}

The owner MUST be able to unpair any device at any time.

### EU-0001-045 {#EU-0001-045}

The owner MUST be able to revoke any share link at any time.

### EU-0001-046 {#EU-0001-046}

Unpairing a device MUST end that device's connections and commands immediately.

### EU-0001-047 {#EU-0001-047}

Revoking a share link MUST end its participants' connections immediately.

### EU-0001-048 {#EU-0001-048}

Unpairing a device or revoking a share link MUST change the keys that protect every later object.

### EU-0001-049 {#EU-0001-049}

Every connection between computers MUST be authenticated and encrypted, except a real-time stream on a network the owner marked as theirs.

### EU-0001-050 {#EU-0001-050}

A real-time stream MUST NOT run unencrypted on a network the owner hasn't marked as theirs.

### EU-0001-051 {#EU-0001-051}

The engine MUST stop an unencrypted real-time stream when the computer's network attachment no longer matches the network the owner marked.

### EU-0001-052 {#EU-0001-052}

The engine MUST NOT open an inbound network port.

### EU-0001-053 {#EU-0001-053}

A device MUST hold its keys in the platform's secure storage where the platform offers it.

### EU-0001-054 {#EU-0001-054}

Euphony MUST treat project text, metadata, imported files, scripts and tool results as data, and never as instructions.

### EU-0001-055 {#EU-0001-055}

Scripts and the parsers of imported projects, model files and analysis media MUST run isolated from the process that drives the audio device.

### EU-0001-056 {#EU-0001-056}

A script or a parser of imported material MUST NOT reach the network, a file or a device beyond what its task needs.

### EU-0001-057 {#EU-0001-057}

The output level of each sink MUST stay within the gain limit the owner sets for it, whichever party changes its mix.

### EU-0001-058 {#EU-0001-058}

A new connection of a sink endpoint MUST start muted.
