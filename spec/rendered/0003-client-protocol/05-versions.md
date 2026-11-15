## Versions and compatibility {#versions}

The client protocol has a major and a minor version. A minor version only adds: commands, optional
fields with defaults, capabilities and kinds of projection. Anything that removes something or
changes what it means is a new major version. At connection each side states the versions and
capabilities it supports, and the two use the newest version both support. Clients and engines from
different releases meet: computers of the person that run different releases, phone controllers
installed from an application store, and scripts. Each project configures the oldest client release
it still supports, and each release of the engine serves every minor version of its major version
from that release on.

### EU-0003-065 {#EU-0003-065}

The client protocol MUST have a major version and a minor version.

### EU-0003-066 {#EU-0003-066}

A new minor version MUST NOT remove or change the meaning of anything that an earlier minor version of the same major version holds.

### EU-0003-067 {#EU-0003-067}

A field that a minor version adds MUST be optional, with a default.

### EU-0003-068 {#EU-0003-068}

A change that removes or changes the meaning of a command, a field, a capability or a kind of projection MUST carry a new major version.

### EU-0003-069 {#EU-0003-069}

At connection, each side MUST state the protocol versions and the capabilities it supports.

### EU-0003-070 {#EU-0003-070}

The two sides of a connection MUST use the newest version that both support.

### EU-0003-071 {#EU-0003-071}

A client MUST hide each action that the engine does not serve, with the reason.

### EU-0003-072 {#EU-0003-072}

The engine MUST refuse a major version it does not speak with a typed error that lists the versions it serves.

### EU-0003-073 {#EU-0003-073}

A client that cannot work with the release of an engine MUST state which release it needs.

### EU-0003-074 {#EU-0003-074}

A side MUST NOT interpret a field or a command that its agreed version does not define.

### EU-0003-075 {#EU-0003-075}

Each release of the engine MUST serve every minor version of its major version back to the oldest client release that the project configures as supported.

### EU-0003-076 {#EU-0003-076}

An engine release that introduces a new major version MUST also serve the previous major version for at least one release.
