## Moving content between computers {#transfer}

Every chunk request, chunk transfer and control message the store sends between the owner's
computers uses the one transport that euphony's own protocols share, and the store doesn't add a
second. A receiver asks which chunks of an item it lacks, fetches only those, checks each against
the signed chunk list before it counts as present, and records what is present only after the data
is durable, so a dropped connection resumes from the missing chunks. Traffic has three levels of
priority: the store's control messages, which are receipts, reservations, release commits, deletion
intents, barriers and outcomes, at the highest; fetches a session waits for, on project open, before
playback starts, during playback and for an audition, at the interactive level; and placement
copies, repair copies and the recall of an archived project at the lowest. Each stream opens at the
level of the work it serves, so a control message or a playback fetch doesn't wait behind queued
bulk chunks, and within a level the capacity is shared from the answers of the holders and the
network, without a fixed cap. Each connection accepts only the owner's paired computers, and is
authenticated and encrypted on every network. A holder that stops ends its transfers cleanly, and
the receiver resumes them. Where one of the owner's computers runs a platform the transport doesn't
support yet, the store waits for that support instead of using another transport.

### EU-0008-111 {#EU-0008-111}

The store MUST NOT use a second transport beside the one that the protocols of euphony share.

### EU-0008-112 {#EU-0008-112}

A receiver MUST record a chunk as present only after the chunk is verified and durable.

### EU-0008-113 {#EU-0008-113}

A transfer that a dropped connection or a stopped holder interrupts MUST resume from the chunks the receiver lacks.

### EU-0008-114 {#EU-0008-114}

The control messages of the store MUST be sent at a higher priority than any fetch or copy.

### EU-0008-115 {#EU-0008-115}

A fetch that a session waits for MUST be sent at a higher priority than placement copies, repair copies and the recall of an archived project.

### EU-0008-116 {#EU-0008-116}

Transfers between the computers of the owner MUST NOT be limited by a fixed cap on concurrency or rate.

### EU-0008-117 {#EU-0008-117}

Each connection of the store MUST accept only the paired computers of the owner.

### EU-0008-118 {#EU-0008-118}

Each connection of the store MUST be authenticated and encrypted on every network.
