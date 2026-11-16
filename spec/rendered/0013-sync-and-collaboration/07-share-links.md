## Sharing a project by link {#share-links}

A share link is an invitation written as an address that a person can paste into a message. It holds
the join address of the signaling service, an invitation identity, a random secret and the
fingerprint of the host's identity key. The link's address is on the application origin, the site
that serves the browser companion's code, which the owner configures and trusts as the application's
distributor. The secret and the fingerprint sit in the part of the address that a browser does not
send in any request, and a native client opens the link through its registered address scheme. The
signaling service and the relay are untrusted operators; the distributor is trusted, and this RFC
does not defend against a malicious distributor. Each link carries one level. A listener receives
the audio of the sink the owner assigns, the main mix by default, and reads the transport state,
markers and track labels. A commenter also adds timestamped comments and markers, which are
annotations of the revision being played, a record kind apart from musical edits. An edit link
starts the enrollment of a collaborator. Transport control is one setting of the session, which no
link carries. The host is reached through the connection path of collaboration, without an inbound
port of its own.

### EU-0013-112 {#EU-0013-112}

A share link MUST grant exactly one level, which is listen, comment or edit.

### EU-0013-113 {#EU-0013-113}

The secret of a link MUST hold at least 128 random bits from the secure random source of the operating system.

### EU-0013-114 {#EU-0013-114}

A link MUST NOT carry its secret in any part of its address that a browser sends in a request.

### EU-0013-115 {#EU-0013-115}

A joining client MUST prove to the host, over the encrypted connection, that it holds the secret of the link.

### EU-0013-216 {#EU-0013-216}

The owner MUST be able to configure the application origin, the site that serves the code of the browser companion and the addresses of its links.

### EU-0013-217 {#EU-0013-217}

The system MUST NOT let code from any origin other than the application origin read the secret or the fingerprint of a link.

### EU-0013-218 {#EU-0013-218}

A native client MUST read the secret and the fingerprint of a link itself, from the link it opens through its registered address scheme.

### EU-0013-116 {#EU-0013-116}

A joining client MUST fail the join where the host key does not match the fingerprint in the link.

### EU-0013-117 {#EU-0013-117}

A signaling or relay operator MUST NOT be able to redeem a link, read its session or place itself between the host and a participant.

### EU-0013-118 {#EU-0013-118}

Where one operator runs both the signaling service and the application origin, the approval that makes a link MUST say that this operator is trusted with the secret of the link.

### EU-0013-119 {#EU-0013-119}

A listener MUST NOT receive project objects, stems, prepared audio files or history.

### EU-0013-120 {#EU-0013-120}

The host MUST store comments and markers from a comment link as annotations of the revision being played, separate from musical edits.

### EU-0013-121 {#EU-0013-121}

A merge MUST NOT treat annotations as edits.

### EU-0013-122 {#EU-0013-122}

The owner MAY hide or delete annotations.

### EU-0013-123 {#EU-0013-123}

A link alone MUST NOT admit an editor.

### EU-0013-124 {#EU-0013-124}

The policy stream MUST record an edit level only after the owner approves the exact device identity of the person who joins.

### EU-0013-125 {#EU-0013-125}

An edit link MUST be single use and bind to the first identity the owner approves.

### EU-0013-126 {#EU-0013-126}

Every link MUST have an expiry.

### EU-0013-127 {#EU-0013-127}

A link MUST expire, by default, when the session it was made for ends.

### EU-0013-128 {#EU-0013-128}

The owner MAY choose a later expiry or a number of uses for a listen or comment link.

### EU-0013-129 {#EU-0013-129}

The host MUST record each redemption of a link with the identity of the joining client and the time.

### EU-0013-130 {#EU-0013-130}

The owner MUST be able to revoke a link, or one participant, from the sharing view of the session, the phone client or the assistant interface.

### EU-0013-131 {#EU-0013-131}

Revocation MUST refuse new redemptions of the link at once.

### EU-0013-132 {#EU-0013-132}

Revocation MUST end each stream and control channel that the link admitted.

### EU-0013-133 {#EU-0013-133}

On revocation, the host MUST rotate the stream keys of the session so that a removed client cannot decode later packets.

### EU-0013-134 {#EU-0013-134}

The dialog that makes a link MUST say that revocation cannot recall audio a listener already heard or recorded on their own device.

### EU-0013-135 {#EU-0013-135}

The host MUST NOT open an inbound port or run a public web server for a shared session.

### EU-0013-136 {#EU-0013-136}

The approval that makes a link MUST show the application origin, the signaling and relay services the session uses and the metadata they see.

### EU-0013-137 {#EU-0013-137}

The policy stream MUST record the identity, level, sink, expiry and use count of each link, with each redemption under it.

### EU-0013-138 {#EU-0013-138}

The command gateway MUST check every action of a participant against the record of the link that admitted them.

### EU-0013-139 {#EU-0013-139}

An assistant MUST NOT make a link without the owner approving its exact effect.

### EU-0013-140 {#EU-0013-140}

Transport control for participants MUST be a setting of the session that is off by default and applies to every participant.

### EU-0013-141 {#EU-0013-141}

A link MUST NOT carry transport control.

### EU-0013-142 {#EU-0013-142}

While transport control is off, the host MUST show the owner each participant request for play or stop.

### EU-0013-143 {#EU-0013-143}

A remote participant MUST NOT join a local-network audio stream.
