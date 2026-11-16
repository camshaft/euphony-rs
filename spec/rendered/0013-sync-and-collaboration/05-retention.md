## Retiring audio and retained history {#retention}

Deprecating a revision adds a signed retention declaration and leaves the revision itself as it was.
The small history of a project is its canonical configurations, MIDI events, programs, declared
seeds, version requirements and merge resolutions. Rendered audio is regenerable only while every
input and every step of execution it needs stays available. A project that uses external samples is
not self-contained because its tracks are MIDI, and a public identifier of a processor does not by
itself mean that its old executable can be obtained. The availability of an object is one of present,
fetchable from an authorized known peer, regenerable with a verified complete input closure,
intentionally pruned, or unavailable with unknown custody. Recomputable caches can be evicted under
pressure. Each peer controls its own retention. An optional backup exports the small history and the
manifests, with a self-contained bundle for a project of MIDI and programs alone, and large audio
goes to a store the owner selects, with verified inventories and instructions to restore it. Which
content a peer may delete, the check of its references before a deletion and the approval that
deleting an original recording takes are those of RFC 0008's collection section; this section
states what deprecating a revision adds to them.

### EU-0013-068 {#EU-0013-068}

Deprecating a revision MUST add a signed retention declaration for it.

### EU-0013-069 {#EU-0013-069}

Deprecating a revision MUST NOT change its parents, configuration, musical events or authorship.

### EU-0013-070 {#EU-0013-070}

A peer MUST retain the small history of each revision that retained work references.

### EU-0013-071 {#EU-0013-071}

Rendered audio MUST be marked regenerable only while its complete input and execution closure stays available.

### EU-0013-072 {#EU-0013-072}

A project of MIDI and programs alone MUST retain its whole logical closure, including the embedded assets it uses that the owner is authorized to keep.

### EU-0013-073 {#EU-0013-073}

A peer MUST report the availability of each object as present, fetchable from an authorized known peer, regenerable, intentionally pruned or unavailable with unknown custody.

### EU-0013-074 {#EU-0013-074}

A fetch of an absent or pruned object MUST return a typed result with its authorized retention metadata.

### EU-0013-075 {#EU-0013-075}

A fetch MUST NOT return silence in place of a missing object.

### EU-0013-076 {#EU-0013-076}

The interface MUST open retained small history and show exactly which regions cannot sound.

### EU-0013-077 {#EU-0013-077}

Complete playback or export MUST wait for the missing source material, or for a new revision that replaces it.

### EU-0013-078 {#EU-0013-078}

Restoring an object by its digest or rendering it again MUST NOT change the historical revision.

### EU-0013-083 {#EU-0013-083}

A deprecation marker alone MUST NOT authorize destroying the last known original of an object.

### EU-0013-084 {#EU-0013-084}

Where the owner authorizes an irreversible pruning, the retention declaration MUST record the loss.

### EU-0013-085 {#EU-0013-085}

A claim of a backup MUST rest on verified receipts of its objects.

### EU-0013-086 {#EU-0013-086}

A peer MUST NOT treat a branch of an offline peer that it has not seen as protected.

### EU-0013-087 {#EU-0013-087}

Syncing a deprecation MUST NOT delete objects on another peer.

### EU-0013-088 {#EU-0013-088}

The owner MAY back up a project's small history and manifests to a destination they choose.

### EU-0013-089 {#EU-0013-089}

A pointer to audio MUST NOT count as a backup of that audio.

### EU-0013-090 {#EU-0013-090}

The approval of a backup destination MUST show the history, media and recipients it includes and what it leaves out.

### EU-0013-091 {#EU-0013-091}

A backup of a private project MUST stay separate from public source.

### EU-0013-092 {#EU-0013-092}

A restore MUST verify the complete manifest before it reports a playable recovery.
