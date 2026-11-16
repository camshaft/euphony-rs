## Items, identities and references {#items}

The content store holds items. Each item has one kind of a closed set: source original, recording
take, rendered result, export, note sequence, MIDI file, automation curve, composition program
source, captured model, analysis result and other blob. A rendered result is a bounce, a stem, a
frozen track or a prepared segment. An item never changes once it is stored. An item's identity is
a digest computed from what it holds. For audio, the digest covers a canonical description of the
format and the decoded sample values, so the same samples stored in two encodings, or moved to a
later codec, are one item. For a note sequence, an automation curve or a composition program
source, the digest covers the canonical serialization the project model defines for it, so equal
notes or equal curves are one item whatever project or track they came from. Any other item's
digest covers its bytes. Each digest is separated by the item's kind and the version of its schema.
An imported file keeps its original bytes as a source original, beside the decoded copy a session
works with, and the notes parsed from an imported MIDI file are a second item linked to the file.
An item is stored and moved in chunks. A list of an item's chunks, signed by the computer that
publishes it, lets a reader check each chunk on its own as it arrives, seek to a frame and fetch a
range alone. The list is accepted only once its chunks decode to the item's identity, and a new
chunk format adds a list without changing the identity. Chunks don't share content between items:
storage saves by whole item. A recording is stored as one item for each take, from its start to
its stop, and a take group lists the takes with their start positions, tracks, inputs and markers.
Where capture runs across takes, the audio between takes is an item of the group, so nothing
captured is lost. A project refers to content only by an item's identity and kind and, for audio, a
half-open frame range. A trim, a split, a comp or a fade refers to its source items, so editing
doesn't make new audio content; new audio content comes from recording, import, rendering,
consolidation and export.

### EU-0008-001 {#EU-0008-001}

Each item MUST have exactly one kind of the closed set of item kinds.

### EU-0008-002 {#EU-0008-002}

An item MUST NOT change once it is stored.

### EU-0008-003 {#EU-0008-003}

The identity of an audio item MUST be computed from its decoded sample values and a canonical description of their format.

### EU-0008-004 {#EU-0008-004}

A change of the encoding or the codec version of an audio item MUST NOT change its identity.

### EU-0008-005 {#EU-0008-005}

The identity of a note sequence, an automation curve or a composition program source MUST be computed from the canonical serialization that the project model defines for it.

### EU-0008-006 {#EU-0008-006}

Two items of different kinds MUST NOT share an identity, even where their bytes are equal.

### EU-0008-007 {#EU-0008-007}

The store MUST keep the original bytes of each imported file unchanged, beside any decoded copy of it.

### EU-0008-008 {#EU-0008-008}

The note sequence parsed from an imported MIDI file MUST be stored as an item of its own, linked to the file by its provenance.

### EU-0008-009 {#EU-0008-009}

Each chunk of an item MUST be verified against the chunk list of the item before it counts as present.

### EU-0008-010 {#EU-0008-010}

A reader MUST be able to fetch and verify one frame range of an audio item without fetching the rest of it.

### EU-0008-011 {#EU-0008-011}

The store MUST NOT accept a chunk list whose chunks do not decode to the identity of its item.

### EU-0008-012 {#EU-0008-012}

A new chunk format MUST NOT change the identity of an item or any reference to it.

### EU-0008-013 {#EU-0008-013}

Each take of a recording MUST be stored as an item of its own.

### EU-0008-014 {#EU-0008-014}

A take group MUST list each take of its recording with its start position, its track, its input and its markers.

### EU-0008-015 {#EU-0008-015}

Audio captured between the takes of a continuous capture MUST be kept as an item of the take group.

### EU-0008-016 {#EU-0008-016}

A project MUST refer to content only by the identity and the kind of an item and, for audio, a frame range.

### EU-0008-017 {#EU-0008-017}

A reference to content MUST NOT hold a location, a computer or a holder.

### EU-0008-018 {#EU-0008-018}

A project MUST open with exactly the content that its revision refers to.

### EU-0008-019 {#EU-0008-019}

A trim, a split, a comp or a fade MUST NOT create new audio content.

### EU-0008-020 {#EU-0008-020}

The store MUST refuse a chunk or a chunk list that fails its verification.
