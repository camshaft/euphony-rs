## Combining versions and conflicts {#merge}

A merge combines two versions from a verified common ancestor, comparing the project's objects by
stable identity: tracks, clips, notes, automation points and routing edges. Independent additions
and changes to different properties combine, and each keeps its author. A change by both sides to
the same property, a deletion of an object the other side edited, overlapping automation, competing
tempo changes and a regeneration by a program of material the other side edited by hand are
conflicts. A conflict is a typed object that holds both versions and offers musical alternatives
for a person to choose between. A merge revision records both parents and each resolution chosen. A
general merge of text or data structures does not know these musical meanings, so it does not stand
in for typed conflicts. Reconciling contradictory musical edits automatically is outside this RFC.

### EU-0013-016 {#EU-0013-016}

A merge MUST start from a verified common ancestor of the versions it combines.

### EU-0013-017 {#EU-0013-017}

A merge MUST identify tracks, clips, notes, automation points and routing edges by stable identities.

### EU-0013-018 {#EU-0013-018}

A merge MUST combine independent additions and changes to different properties without asking a person.

### EU-0013-019 {#EU-0013-019}

A merge MUST keep the authorship of each change it combines.

### EU-0013-020 {#EU-0013-020}

A merge MUST make a typed conflict of changes by both sides to the same property.

### EU-0013-021 {#EU-0013-021}

A merge MUST make a typed conflict of a deletion of an object that the other side edited.

### EU-0013-022 {#EU-0013-022}

A merge MUST make a typed conflict of overlapping automation changes.

### EU-0013-023 {#EU-0013-023}

A merge MUST make a typed conflict of competing tempo changes.

### EU-0013-024 {#EU-0013-024}

A merge MUST make a typed conflict of a program regenerating material that the other side edited by hand.

### EU-0013-025 {#EU-0013-025}

A merge preview MUST show both versions of each conflict and offer musical alternatives for it.

### EU-0013-026 {#EU-0013-026}

A merge MUST validate the timing, the routing cycles and the musical constraints of its result, also where no edits conflicted.

### EU-0013-027 {#EU-0013-027}

A merge MUST NOT resolve conflicting notes or routing by the clock time at which each edit was made.

### EU-0013-028 {#EU-0013-028}

A merge revision MUST record both parents and each resolution chosen.

### EU-0013-029 {#EU-0013-029}

A merge with an unresolved conflict MUST NOT become the revision that plays or exports.

### EU-0013-030 {#EU-0013-030}

Where two versions have more than one merge base, a merge MUST synthesize a base from them only where their own merge has no conflict.

### EU-0013-031 {#EU-0013-031}

Where the merge bases of two versions conflict with each other, the merge MUST require an explicit resolution of those conflicts.

### EU-0013-032 {#EU-0013-032}

Each conflict MUST stay shown until a person resolves it.

### EU-0013-033 {#EU-0013-033}

When two collaborators who edited while disconnected exchange their versions, every accepted edit of each MUST be kept.
