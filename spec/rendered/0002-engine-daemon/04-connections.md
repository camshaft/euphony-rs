## Connections and the origin of each command {#connections}

The local endpoint of the host service admits connections of the same operating-system user, checked
by the credentials of the peer, and that check admits a connection without assigning it an origin.
At attach, the host service issues the connection a client session credential bound to the session
and its generation, and the session host reads the origin of every command from that client session.
The owner's native interface authenticates with the paired-device key of its computer, kept in the
platform's secure storage. An assistant's adapter, a script, the command line and the terminal
interface each authenticate as a client session of their own, with an assistant or a script origin.
A client on another of the owner's computers connects over the mutually authenticated and encrypted
connection of a paired device, and a link participant attaches at the level of its link. The
owner's single-party trust model, pairing and link levels are specified in the overview RFC.

### EU-0002-045 {#EU-0002-045}

The local endpoint MUST admit only connections of the operating-system user that runs the host service.

### EU-0002-046 {#EU-0002-046}

Admission to the local endpoint MUST NOT assign a connection an origin.

### EU-0002-047 {#EU-0002-047}

At attach, the host service MUST issue the connection a client session credential bound to the session and its generation.

### EU-0002-048 {#EU-0002-048}

The session host MUST read the origin of each command from the client session that sent it, whatever the command claims.

### EU-0002-049 {#EU-0002-049}

The host service MUST assign the origin of the owner's native client only to a client that authenticates with the paired-device key of its computer.

### EU-0002-081 {#EU-0002-081}

The host service MUST assign a local client that doesn't authenticate with the paired-device key of its computer an assistant or a script origin, whatever the client claims.

### EU-0002-082 {#EU-0002-082}

The host service MUST assign a paired-device or link-participant origin only to a connection that authenticated as that paired device or that link.

### EU-0002-050 {#EU-0002-050}

A command that a client forwards, queues or replays MUST keep the origin of the client session that first sent it.

### EU-0002-051 {#EU-0002-051}

The session host MUST accept an owner-interaction record only from the owner's native client, made from the owner's own input on its interface and signed with the paired-device key of its computer.

### EU-0002-052 {#EU-0002-052}

The session host MUST NOT accept an approval decision from an assistant, a script, the command line or the terminal interface.

### EU-0002-053 {#EU-0002-053}

A client on another of the owner's computers MUST attach over a mutually authenticated and encrypted connection of a paired device.

### EU-0002-054 {#EU-0002-054}

The host service MUST NOT admit a remote attachment from a computer that isn't paired.

### EU-0002-055 {#EU-0002-055}

A link participant's attachment MUST receive only what its link's level allows.

### EU-0002-056 {#EU-0002-056}

The session host MUST receive a file that a local client picks as an open handle, without a grant of the file's path.
