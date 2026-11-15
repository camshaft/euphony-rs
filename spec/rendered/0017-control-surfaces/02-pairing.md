## Discovery, pairing and access {#pairing}

The discovery service finds controllers on the local network and reports each one as a typed
controller capability, with its identity, its class, the owner's device or a link participant's
device, and its key fingerprint. Discovery on the local network is an untrusted hint. Pairing sets up
a key for each device through a pairing attempt that the owner approves, with a commitment step and
a code comparison, and the owner confirms it on a screen of the workstation; each computer checks
the paired list on every connection. A device outside the local network, and the browser companion,
joins through an invitation and an authenticated connection with the same relay and origin
restrictions as other devices. The gateway is the boundary of the session host that every attach,
layout commit, control operation and delivered projection of a surface passes through. The trust
model, the levels of a link and the meaning of pairing are stated by RFC 0001; a second window or a
touch monitor wired to a computer is part of that computer and isn't paired on its own. Pairing
establishes a device's identity, not trust in every later input from it.

### EU-0017-012 {#EU-0017-012}

Discovering a device MUST NOT pair it or let it read, control or hear the session.

### EU-0017-013 {#EU-0017-013}

The system MUST report each discovered controller with its identity, its class and its key fingerprint.

### EU-0017-014 {#EU-0017-014}

Pairing a surface MUST show the owner the session's identity and a confirmation of the device's key.

### EU-0017-015 {#EU-0017-015}

A pairing MUST take effect only once the owner confirms it.

### EU-0017-016 {#EU-0017-016}

An assistant MUST NOT pair a surface.

### EU-0017-017 {#EU-0017-017}

An assistant MAY propose pairing a surface for the owner to confirm.

### EU-0017-018 {#EU-0017-018}

The pairing screen MUST show the surface, a warning for a shared screen, the limits of the device's credential storage and that unpairing cannot erase what the device has received.

### EU-0017-019 {#EU-0017-019}

The gateway MUST attach a surface only after it validates the surface's pairing or link level, its identity and its protocol capabilities.

### EU-0017-020 {#EU-0017-020}

A paired surface MUST act with the owner's access.

### EU-0017-021 {#EU-0017-021}

A surface of a participant joined by link MUST act at that link's level.

### EU-0017-022 {#EU-0017-022}

The gateway MUST check a surface's pairing or link level on every attach, layout commit, control operation and delivered projection.

### EU-0017-023 {#EU-0017-023}

An unpaired device MUST NOT receive anything beyond the public connection information.

### EU-0017-024 {#EU-0017-024}

A projection sent to a link participant's surface MUST hold only what that link's level allows, filtered before it is sent.

### EU-0017-025 {#EU-0017-025}

A surface MUST keep its credentials in the platform's secure storage where the platform offers it.

### EU-0017-026 {#EU-0017-026}

Unpairing a surface MUST end its subscriptions and its control at once.

### EU-0017-027 {#EU-0017-027}

Unpairing a surface MUST NOT stop a recording that was already authorized.

### EU-0017-028 {#EU-0017-028}

The gateway MUST validate a reconnecting surface's pairing again before it attaches.

### EU-0017-029 {#EU-0017-029}

A hardware adapter MUST bind a controller's physical port identity to its paired host and its map.

### EU-0017-030 {#EU-0017-030}

A controller whose identity is ambiguous or has changed MUST NOT inherit another controller's mapping.

### EU-0017-031 {#EU-0017-031}

Control traffic between a surface and the session MUST run over a mutually authenticated connection.
