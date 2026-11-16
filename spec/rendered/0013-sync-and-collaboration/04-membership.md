## Membership and revocation {#membership}

An owner-signed policy stream records each collaborator's level, listen, comment or edit, and each
revocation. Each peer already controls its own branches and its own retention, so the stream does
not hold a merge, export or retention permission. A revoked peer's proposals stay quarantined: an
authorized person can import them explicitly, and they are never accepted as current edits.
Revocation acts from the moment a peer learns of it. It cannot erase copies already received, and an
offline peer learns of it only when it connects again.

### EU-0013-059 {#EU-0013-059}

Membership MUST be recorded in a policy stream that the owner signs, separate from the musical history of the project.

### EU-0013-060 {#EU-0013-060}

The policy stream MUST record each collaborator's level, which is listen, comment or edit, and each revocation.

### EU-0013-061 {#EU-0013-061}

A musical merge MUST NOT change membership.

### EU-0013-062 {#EU-0013-062}

Sync MUST validate the signatures and the level of the sending peer against the current policy of the receiver before it updates accepted heads or serves objects.

### EU-0013-063 {#EU-0013-063}

The proposals of a revoked peer MUST stay quarantined until an authorized person imports them explicitly.

### EU-0013-064 {#EU-0013-064}

Revocation MUST stop new transfers to and commands from the revoked peer once the applying peer learns of it.

### EU-0013-065 {#EU-0013-065}

Revocation MUST rotate the keys of objects made after it.

### EU-0013-066 {#EU-0013-066}

When a link or an invitation is made, the system MUST show that revocation cannot erase copies already received or reach an offline peer at once.

### EU-0013-067 {#EU-0013-067}

Permission to edit a local copy MUST NOT grant permission to publish it or send it to another person.
