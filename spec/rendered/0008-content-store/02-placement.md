## Tiers, placement and repair {#placement}

Each of the owner's paired computers declares, in configuration the owner signs, whether it holds a
working tier, a storage tier or both, and the storage it offers each. A working computer holds what
its sessions use; a storage computer, called a holder, holds long-term and archived content, and a
computer can be a holder and a render worker at once. The owner's placement policy is a signed,
versioned list of rules. A rule selects items by kind, by whether the item can be recreated, by the
projects that use it and whether they are archived, by when a session last used it, and by size,
and places them as a number of verified copies on separate computers, the tiers that hold them, and
whether a working computer keeps a copy. The policy starts with these values, which the owner
changes: two verified copies on separate computers for source originals and recordings, one of them
on a storage tier; no copy beyond the render cache for a rendered result whose inputs are retained;
and a move to storage when the owner archives a project, with the rules on last use and size off.
Each of the owner's computers plans from the same policy and the latest availability reports, on
events, and plans the same actions: copy an item's missing chunks to a holder, verify a holder's
copy, or release a working copy. A holder that has stored and verified an item signs a receipt that
names the item, its chunk list, the holder and the storage volume. A storage volume carries a random
identifier its holder writes when it enrolls it, and a volume on network storage also names its
server, so copies on one volume or on one server count as one copy. A copy is released only through
reservations that other holders record for that release: each reserving holder keeps its copy, and
the release commits only while the policy and the pairing are those it recorded. Each release has
an epoch, so that a release planned by a computer the owner declares lost is settled once and its
late messages change nothing. A holder verifies a copy by reading each chunk, checking its digest,
decoding it and checking the item's identity. Where the verified copies of an item fall below its
policy, the planner copies it again from the remaining copies; where no verified copy remains, the
item is unavailable and each project shows the regions that can't sound. Holders don't encrypt what
they store: an owner who wants encryption at rest turns on disk encryption on each computer.

### EU-0008-021 {#EU-0008-021}

The placement policy MUST be a versioned record signed by the owner.

### EU-0008-022 {#EU-0008-022}

A change of the placement policy MUST take effect only once the owner approves it.

### EU-0008-023 {#EU-0008-023}

Each computer of the owner MUST plan placement from the same signed policy and the latest availability reports of the computers of the owner.

### EU-0008-024 {#EU-0008-024}

Placement MUST keep, for each item, the number of verified copies on separate computers that the policy sets for it.

### EU-0008-025 {#EU-0008-025}

Placement MUST plan again when a project is archived, opened or closed, a session ends, the policy changes, a holder joins, leaves or reports a storage error, a verification fails, or a computer is unpaired or declared lost.

### EU-0008-026 {#EU-0008-026}

The placement planner MUST NOT run on a timer.

### EU-0008-027 {#EU-0008-027}

Two copies on one storage volume, or on volumes of one network storage server, MUST count as one copy.

### EU-0008-028 {#EU-0008-028}

A holder MUST sign a receipt for each item it has stored and verified.

### EU-0008-029 {#EU-0008-029}

A receipt MUST count as stale once its holder restarts, reports a storage error on its volume or is unpaired, until the holder verifies the item again.

### EU-0008-030 {#EU-0008-030}

Two holders that report the same storage volume MUST be shown to the owner as a conflict until it is resolved.

### EU-0008-031 {#EU-0008-031}

A copy MUST be released only where other holders on separate computers and independent storage have reserved the item for that release, covering the copies the policy requires.

### EU-0008-032 {#EU-0008-032}

A holder that holds a reservation on an item MUST NOT release its own copy of that item.

### EU-0008-033 {#EU-0008-033}

A release MUST be cancelled where the policy or the pairing of the computers of the owner changes before it commits.

### EU-0008-034 {#EU-0008-034}

Each release MUST reach exactly one terminal state, committed or cancelled, which never changes afterwards.

### EU-0008-035 {#EU-0008-035}

After a crash, a holder MUST keep its reservations until the terminal state of the release they serve is known.

### EU-0008-036 {#EU-0008-036}

Where the computer that planned a release is declared lost, recovery MUST settle the release on the releasing holder before it frees any reservation.

### EU-0008-037 {#EU-0008-037}

Once recovery has settled a release, the releasing holder MUST refuse every later message of that release.

### EU-0008-038 {#EU-0008-038}

A source original or a recording MUST NOT be released from its last working copy before a release of it commits.

### EU-0008-039 {#EU-0008-039}

A verification MUST check each chunk of a copy and its decoded content against the identity of the item.

### EU-0008-040 {#EU-0008-040}

A holder MUST verify a copy when it is written, when the holder starts, when it reports a storage error, when the owner asks, and before a working copy is released on the strength of that copy.

### EU-0008-041 {#EU-0008-041}

Where the verified copies of an item fall below its policy, placement MUST copy it again from the remaining verified copies.

### EU-0008-042 {#EU-0008-042}

The owner MUST be shown each item that has fewer verified copies than its policy sets.

### EU-0008-043 {#EU-0008-043}

When a holder returns after its copies were repaired, placement MUST release the surplus copies that the policy does not need.

### EU-0008-044 {#EU-0008-044}

Where no verified copy of an item remains, each project that refers to it MUST show which regions cannot sound.

### EU-0008-045 {#EU-0008-045}

Each step of placement, verification, repair, release and fetch MUST emit a typed event with the item kind, the tier and the bytes.

### EU-0008-046 {#EU-0008-046}

An event of the store MUST NOT hold the file name or the project of an item.
