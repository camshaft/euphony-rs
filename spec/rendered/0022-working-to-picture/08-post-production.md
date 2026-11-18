## Sound design and post-production tracks {#post-production}

Post-production sorts the sound of a film into dialogue, music and effects. A track's role takes the
post-production roles: dialogue, replaced dialogue, music, effects, foley, which is sound performed
to picture such as footsteps, and backgrounds. A stem bus collects one part of the mix: dialogue,
music or effects. The music-and-effects bus sums the music and effects stems. A sync point is a
sample within a region, such as the attack of a slammed gate, that lines up with a picture frame.
The sound effects category of an item is a category of the public category list that sound effects
libraries use. Replaced dialogue is speech that a performer records again while watching the picture,
to take the place of a line recorded on set. Cue beeps are short tones before a line's start that
count the performer in to it, as a streamer shows the line's start on the picture.

### EU-0022-124 {#EU-0022-124}

Track roles MUST include each post-production role this section lists.

### EU-0022-125 {#EU-0022-125}

In a project that holds the stem buses, each track's output MUST go to one of them.

### EU-0022-126 {#EU-0022-126}

Foley and background tracks MUST feed the effects stem by default.

### EU-0022-127 {#EU-0022-127}

A project MUST be able to hold a music-and-effects bus that sums the music and effects stems.

### EU-0022-128 {#EU-0022-128}

A post-production project template MUST create the stem buses, a return for each stem and their delivery records.

### EU-0022-129 {#EU-0022-129}

Each region MUST have a sync point.

### EU-0022-130 {#EU-0022-130}

Euphony MUST suggest a region's sync point from the onsets of its sound.

### EU-0022-131 {#EU-0022-131}

A sync point MUST snap to frames and to picture markers.

### EU-0022-132 {#EU-0022-132}

A person MUST be able to place a region's sync point at a timecode.

### EU-0022-133 {#EU-0022-133}

A person MUST be able to move a region by frames or by hundredths of a frame.

### EU-0022-134 {#EU-0022-134}

A location recording MUST arrive at the timecode that its file's metadata records.

### EU-0022-135 {#EU-0022-135}

Analysis MUST find where speech is present on the guide track and on dialogue tracks.

### EU-0022-136 {#EU-0022-136}

The catalog MUST find a sound effect by words, by category and by similar sound.

### EU-0022-137 {#EU-0022-137}

The catalog MUST read the sound effects category of an item from its file name and metadata.

### EU-0022-138 {#EU-0022-138}

A candidate sound effect MUST play at the playhead in step with the picture, looped over the selection, before the person places it.

### EU-0022-139 {#EU-0022-139}

Placing a sound effect MUST align its sync point, its first strong attack by default, with the chosen frame or marker.

### EU-0022-173 {#EU-0022-173}

A person MUST be able to record replaced dialogue to picture, with cue beeps and streamers before each line.
