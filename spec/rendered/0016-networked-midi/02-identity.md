## Device identity and bindings {#identity}

A device binding is the project record that ties routes to a device. It holds the key identity of
the paired host the device is on; the physical identity of the device, where the host supplies one;
the identity and direction of the port; the version of the port description; and an ambiguity
state, resolved or unresolved. A host advertises each identifier that its backend reports for a
device, with the identifier's source and a confidence: an identifier the operating system assigns,
a product, vendor and serial number, or an instance identifier the device reports. The name of a
port is a display coordinate. The current network address, the enumeration index the operating
system assigns a port and the generation of a transport session are coordinates too, kept apart
from identity. Where a device has no stable identity, the owner confirms which physical connector
of the host it is on, and that confirmation counts as its identity. A device has a default mapping,
and each project can hold an override of it.

### EU-0016-013 {#EU-0016-013}

Each device binding MUST hold the key identity of the paired host the device is on.

### EU-0016-014 {#EU-0016-014}

Where the host supplies a physical identity for a device, the device binding MUST hold that identity with each identifier's source and confidence.

### EU-0016-015 {#EU-0016-015}

Each device binding MUST hold the identity and direction of its port.

### EU-0016-016 {#EU-0016-016}

Each device binding MUST hold an ambiguity state, resolved or unresolved.

### EU-0016-017 {#EU-0016-017}

A host MUST advertise, for each port, each identifier its backend reports for the port's device, with the identifier's source and a confidence.

### EU-0016-018 {#EU-0016-018}

A device binding MUST NOT be matched by a network address, an enumeration index, a session generation or a display name alone.

### EU-0016-019 {#EU-0016-019}

A device that returns with a verified identity MUST receive the routes of its own saved binding.

### EU-0016-020 {#EU-0016-020}

A device that cannot be told apart from another device MUST stay unresolved until the owner confirms its physical connector.

### EU-0016-021 {#EU-0016-021}

An unresolved device MUST NOT receive the routes of another device's binding.

### EU-0016-022 {#EU-0016-022}

Changed hardware on a confirmed connector MUST stay unresolved until the owner confirms it again.

### EU-0016-023 {#EU-0016-023}

The session host MUST keep a device's saved bindings and routes while the device is absent.

### EU-0016-024 {#EU-0016-024}

A host that presents a different key MUST NOT hold a binding saved for another key until the owner pairs it.

### EU-0016-025 {#EU-0016-025}

Each device MUST have a default mapping that applies in each project that doesn't override it.

### EU-0016-026 {#EU-0016-026}

Each project MUST keep its own override of a device's default mapping.
