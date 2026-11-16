## Arrangement, mixer and track automation {#content}

The import carries over the arrangement, notes and automation a musician made: the tempo, meter and
markers, the tracks, the audio and note regions, the mixer and its routing, the instrument and the
effects of each track with their latency compensation, and the automation recorded on each track. A
region is a placed piece of a recording or of notes on a track. Each feature converts at each format
version whose reader of it is verified, and every instance the import doesn't convert is reported,
as the section on the conversion report states. Automation inside regions, timing and pitch edits,
comps and loop grids follow in the next section, and the settings of effects and instruments in the
one after it.

### EU-0014-021 {#EU-0014-021}

The import MUST convert a source project's tempo, meter and markers.

### EU-0014-022 {#EU-0014-022}

The import MUST convert each track of a source project with its name.

### EU-0014-023 {#EU-0014-023}

The import MUST convert each audio region with the file it plays.

### EU-0014-024 {#EU-0014-024}

The import MUST convert each note region with its notes, controller events, program changes and pitch bends.

### EU-0014-025 {#EU-0014-025}

The import MUST convert each track's fader, pan, mute, solo and routing.

### EU-0014-026 {#EU-0014-026}

The import MUST convert each track's effects in their slot order, with their bypass state.

### EU-0014-027 {#EU-0014-027}

The import MUST convert each track's automation of its mixer controls and its effect controls.

### EU-0014-028 {#EU-0014-028}

The import MUST convert a feature at a format version only where its reader of that feature is verified at that version.

### EU-0014-074 {#EU-0014-074}

The import MUST convert each track's instrument to a built-in replacement through a mapping.

### EU-0014-075 {#EU-0014-075}

The import MUST convert the latency compensation of each track's effects, so that each track keeps the alignment it had in the source project.
