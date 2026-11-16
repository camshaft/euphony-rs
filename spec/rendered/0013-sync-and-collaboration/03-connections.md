## Connections between peers {#connections}

Discovery on the local network finds candidate peers but does not make them trusted. Across the
internet, collaborators connect through authenticated invitations and a signaling service that
exchanges connection candidates. A direct path is used where one exists, and a relay carries the
traffic where it does not, including through networks that allow only encrypted web traffic. The
signaling service and the relay are connection infrastructure, public or self-hosted, which the
owner chooses when first turning on internet collaboration; neither is a project host or an
authority over history, and neither is trusted with project contents. Each computer holds a
long-term device key, separate from the keys of any one connection. A browser connection carries
companion control and selected prepared audio, and full collaboration on a project takes place
between native copies. Existing standards for local discovery, local audio streams and MIDI stay
local-network features, and project sync across the internet is not sample-synchronous performance.
An offline bundle stays a separate way to recover a project's history.

### EU-0013-034 {#EU-0013-034}

Discovery on the local network MUST NOT make a peer trusted.

### EU-0013-035 {#EU-0013-035}

Collaborators MUST be able to connect and sync across the internet through home routers.

### EU-0013-036 {#EU-0013-036}

Where no direct path between two peers works, their connection MUST use a relay that the owner approved.

### EU-0013-037 {#EU-0013-037}

A relayed connection MUST work through a network that allows only encrypted web traffic.

### EU-0013-038 {#EU-0013-038}

A signaling service or a relay MUST NOT hold project history or act as an authority over it.

### EU-0013-039 {#EU-0013-039}

The owner MUST be able to choose a public or a self-hosted signaling service and relay.

### EU-0013-040 {#EU-0013-040}

The system MUST NOT configure a signaling service or a relay by default.

### EU-0013-041 {#EU-0013-041}

The approval of a connection MUST show the domains of the services it uses and the traffic metadata they see.

### EU-0013-042 {#EU-0013-042}

A peer MUST bind the other peer's identity and the session keys to the approved invitation before it reveals branch heads, manifests or object availability.

### EU-0013-043 {#EU-0013-043}

Project payloads MUST be encrypted end to end, independently of every signaling and relay operator.

### EU-0013-044 {#EU-0013-044}

A peer MUST authenticate the signaling service it uses.

### EU-0013-045 {#EU-0013-045}

A peer MUST refuse a replayed invitation.

### EU-0013-046 {#EU-0013-046}

A peer MUST close a connection whose peer key changes.

### EU-0013-047 {#EU-0013-047}

Before it sends anything else, a peer MUST verify that the other peer signed its connection key, the session nonce and the invitation identity with a device key that the invitation and the owner-signed policy admit.

### EU-0013-048 {#EU-0013-048}

Each signaling message MUST be sealed to the key of the peer it is for.

### EU-0013-214 {#EU-0013-214}

A browser companion MUST sign its connections with a key of its own, separate from the certificate of its browser connection.

### EU-0013-215 {#EU-0013-215}

A peer MUST NOT admit a paired device whose key differs from the key its pairing recorded.

### EU-0013-049 {#EU-0013-049}

Every connection that collaboration makes MUST stay encrypted end to end, whatever network it starts on.

### EU-0013-050 {#EU-0013-050}

Sync MUST exchange missing small revision objects before it fetches audio.

### EU-0013-051 {#EU-0013-051}

Sync MUST fetch audio by its digest in resumable chunks, verifying each chunk.

### EU-0013-052 {#EU-0013-052}

Transfers for playback preparation MUST take precedence over transfers of history.

### EU-0013-053 {#EU-0013-053}

A connection failure MUST keep local edits and queued sync.

### EU-0013-054 {#EU-0013-054}

Queued sync MUST resume when a connection to the peer returns.

### EU-0013-055 {#EU-0013-055}

Where neither a direct path nor an approved relay path works, a peer MUST report collaboration as unavailable.

### EU-0013-056 {#EU-0013-056}

A browser connection MUST carry only companion control and selected prepared audio.

### EU-0013-057 {#EU-0013-057}

A collaborator MAY exchange a project's history as an offline bundle to recover it.

### EU-0013-058 {#EU-0013-058}

Project sync across the internet MUST NOT be presented as sample-synchronous performance.
