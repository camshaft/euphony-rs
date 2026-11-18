## What a sent record holds {#sent-records}

A submission record is a record of a batch. The submission schema is public and separate from the
schema of the local journal, and every field it holds has a privacy class: a closed value, a
bucketed number, a release or build identity, or a frame of euphony's own public code. The client
generalizes values that could single out a computer, and replaces each crash frame outside
euphony's own code with one frame that stands for other code. Records are pseudonymous: each
installation holds a random submission identity and a deletion key pair, both made on the computer.
The identity lets the receiving service count distinct submission identities, which aren't verified
computers since one sender can make many, and lets a person delete what they sent.

### EU-0021-065 {#EU-0021-065}

A sent record MUST hold only fields of the submission schema.

### EU-0021-066 {#EU-0021-066}

Each field of the submission schema MUST have a privacy class.

### EU-0021-067 {#EU-0021-067}

The submission schema MUST NOT hold a free-text field.

### EU-0021-068 {#EU-0021-068}

A sent record MUST NOT hold audio, notes or MIDI content, project, track, region or file names, paths, processor parameter values or state, the learning store, link, collaborator or device names, network addresses or the person's account, outside an attachment that the person adds to one problem report.

### EU-0021-069 {#EU-0021-069}

The client MUST describe the computer only by its processor family, a range of core counts, a range of memory, the operating system's family and major version, and an audio interface model from a closed list of known models.

### EU-0021-070 {#EU-0021-070}

An audio interface model outside the closed list of known models MUST be sent as other.

### EU-0021-071 {#EU-0021-071}

Each time in a batch MUST be an offset within the batch.

### EU-0021-072 {#EU-0021-072}

The client MUST remove the local project and session references from every sent record.

### EU-0021-073 {#EU-0021-073}

Each crash frame outside euphony's own code MUST be sent as a single frame that stands for other code.

### EU-0021-074 {#EU-0021-074}

Each batch MUST carry its exposure intervals and the counts of batches the client built, sent and deleted since its previous batch.

### EU-0021-075 {#EU-0021-075}

Each installation MUST hold a random submission identity made on the computer.

### EU-0021-076 {#EU-0021-076}

The submission identity MUST NOT be derived from the hardware or from an account.

### EU-0021-077 {#EU-0021-077}

A sent record MUST NOT hold any link between the batches of one installation other than the submission identity.

### EU-0021-078 {#EU-0021-078}

Each installation MUST hold a deletion key pair made on the computer.

### EU-0021-079 {#EU-0021-079}

The first batch of a submission identity MUST register the identity with its public key.

### EU-0021-080 {#EU-0021-080}

Every batch after the first and every deletion request MUST be signed with the installation's private key.

### EU-0021-081 {#EU-0021-081}

The signature of a batch MUST cover its batch identity and the digest of its contents.

### EU-0021-082 {#EU-0021-082}

A batch MUST travel over an encrypted connection that authenticates the receiving service.

### EU-0021-083 {#EU-0021-083}

A batch MUST go only to the destination that the release's public configuration sets.

### EU-0021-084 {#EU-0021-084}

Resetting the submission identity MUST start a new one.

### EU-0021-085 {#EU-0021-085}

After a reset, the client MUST keep the old key until the person deletes the old identity's data or chooses to forget the key.

### EU-0021-086 {#EU-0021-086}

Before the person forgets a key, the client MUST say that a forgotten key can't delete that identity's data later.

### EU-0021-087 {#EU-0021-087}

Before the first send, the outbox MUST say which data a deletion removes and which remains, that a public tracker issue holds only counts that passed the publication policy, and that copies others make of a published issue can't be deleted.
