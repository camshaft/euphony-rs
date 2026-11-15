## Automation inside regions, edits, comps and loop grids {#edits}

Automation inside a region, the timing and pitch edits inside a recording, comps assembled from
parts of several takes, and the loop grid of cells started live come over as euphony's own records,
edited through the same typed commands as the rest of the project, so that the desktop and the
assistants edit them the same way. A scene of a loop grid is a row of cells that start together. A
timing edit plays through a built-in timing processor and a pitch edit through a built-in pitch
processor; the source workstation's processing isn't published, so a converted edit sounds close to
the source, by a difference the import measures and shows, rather than identical. A comp and
automation inside a region whose sections, crossfades and curves are read exactly differ only by the
processors they pass through.

### EU-0014-029 {#EU-0014-029}

The import MUST convert automation recorded inside a region as automation anchored to that region, at positions relative to the region's start.

### EU-0014-030 {#EU-0014-030}

Automation anchored to a region MUST move, copy and loop with its region.

### EU-0014-031 {#EU-0014-031}

Automation anchored to a region MUST stay separate from its track's automation.

### EU-0014-032 {#EU-0014-032}

Where automation inside a region and track automation act on one control, the import MUST combine them as the source project does, with one of euphony's modulation combination modes.

### EU-0014-033 {#EU-0014-033}

The import MUST report a combination of automation inside a region and track automation that no modulation combination mode expresses.

### EU-0014-034 {#EU-0014-034}

The import MUST convert the timing edits inside a recording as the region's time map, of pairs of a position in the recording and its position on the timeline.

### EU-0014-035 {#EU-0014-035}

A converted time map MUST record the source workstation's timing mode as the region's source mode.

### EU-0014-036 {#EU-0014-036}

The import MUST map each timing mode of the source workstation to the closest built-in timing mode.

### EU-0014-037 {#EU-0014-037}

The conversion report MUST list the built-in mode each source timing mode maps to.

### EU-0014-038 {#EU-0014-038}

A region with a time map MUST record the version of the built-in timing processor that plays it.

### EU-0014-039 {#EU-0014-039}

The import MUST convert the pitch edits of a region as edits of each detected note: its position, length, pitch offset, pitch drift, vibrato, formant shift and gain.

### EU-0014-040 {#EU-0014-040}

The import MUST keep the notes the source workstation detected in a recording, rather than detect them again.

### EU-0014-041 {#EU-0014-041}

The import MUST convert a folder of takes as the region's takes, keeping every take.

### EU-0014-042 {#EU-0014-042}

The import MUST convert each comp as an ordered list of sections, each with the take it uses, its start, its end, and its crossfade lengths and shapes.

### EU-0014-043 {#EU-0014-043}

The import MUST convert the other saved comps of a folder of takes as alternative comps.

### EU-0014-044 {#EU-0014-044}

The import MUST convert a loop grid as a loop grid whose cells each hold their region, loop length, start quantization and play settings.

### EU-0014-045 {#EU-0014-045}

The cells of one scene of a converted loop grid MUST start together.

### EU-0014-046 {#EU-0014-046}

A converted loop grid MUST play live, with each start quantized to the session's tempo map.

### EU-0014-047 {#EU-0014-047}

The import MUST convert a performance recorded from a loop grid into the arrangement as ordinary regions, keeping the grid beside them.

### EU-0014-048 {#EU-0014-048}

The conversion report MUST show, for each converted timing edit, pitch edit and comp, the measured difference between euphony's playback of its passage and the musician's own render of that passage from the source workstation.

### EU-0014-049 {#EU-0014-049}

Each automation event inside a region, timing or pitch edit, comp section and loop cell of a source project MUST be converted at its original position or reported as not converted with its track, region and position.
