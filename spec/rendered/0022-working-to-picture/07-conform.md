## Following a new cut {#conform}

A new cut arrives as a new video with a change list from the old cut. The change list holds each
segment of the old cut, with the range it moved to in the new cut or its removal, each new segment,
and each segment's confidence. It comes from the editor's lists of both cuts, in the edit list and
timeline formats that video editing programs write, matched event by event through each event's
source reel and source timecode; or, without lists, from the guide tracks of the two cuts, aligned
by comparing short windows of their sound, together with cut detection on both videos. Conforming
moves the project's material onto the new cut by the change list.

### EU-0022-110 {#EU-0022-110}

A person MUST be able to make a change list from the editor's lists of both cuts.

### EU-0022-111 {#EU-0022-111}

A person MUST be able to make a change list from the sound of the guide tracks of both cuts when no lists are given.

### EU-0022-112 {#EU-0022-112}

Conforming MUST move each region, picture-locked marker, automation range and cue boundary with its segment.

### EU-0022-177 {#EU-0022-177}

Conforming MUST make every move of a change list in one command.

### EU-0022-113 {#EU-0022-113}

A conform MUST be undoable as one command that restores the earlier state exactly.

### EU-0022-114 {#EU-0022-114}

Conforming MUST split regions at segment boundaries with short crossfades, 10 milliseconds long by default.

### EU-0022-115 {#EU-0022-115}

Conforming MUST put material over removed picture on a muted lane of its own track.

### EU-0022-116 {#EU-0022-116}

Conforming MUST NOT delete material over removed picture.

### EU-0022-117 {#EU-0022-117}

Conforming MUST mark each new segment as a gap.

### EU-0022-118 {#EU-0022-118}

A segment whose confidence is below the project's threshold MUST be applied only after the person confirms it.

### EU-0022-119 {#EU-0022-119}

Conforming MUST NOT lengthen or shorten the music.

### EU-0022-120 {#EU-0022-120}

Conforming MUST report each cue whose picture changed inside its range, with the choices to cut it at a bar, fit its tempo again to the moved hits, or leave it.

### EU-0022-121 {#EU-0022-121}

Conforming MUST report every segment and its move.

### EU-0022-122 {#EU-0022-122}

The project MUST keep each change list, so that a conform can be compared and repeated.

### EU-0022-123 {#EU-0022-123}

A malformed video, edit list or exchange file MUST be refused without an effect on the session.
