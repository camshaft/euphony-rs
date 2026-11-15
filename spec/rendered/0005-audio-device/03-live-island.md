## The live render island {#live-island}

The live render island is the part of the project graph that the audio device process renders
while the device plays: input monitoring, instruments played live from note input, and the track the
musician is editing, with the buses that depend on it. Everything else plays from prepared audio.
The island renders on the musician's own computer and doesn't wait on another computer for audio.
Each playback segment comes from one compiled graph revision. Where the island can't meet the
measured deadline of the device, playback keeps the previous revision the musician selected, or
pauses the preview where the musician selected none, and euphony shows that the sound isn't the
current revision.

### EU-0005-019 {#EU-0005-019}

Input monitoring MUST render in the live render island of the computer the input is attached to.

### EU-0005-020 {#EU-0005-020}

An instrument played live from note input MUST render in the live render island of the computer the musician plays it on.

### EU-0005-021 {#EU-0005-021}

While a track is being edited, the audio device process MUST render the affected part of that track, and each bus that depends on it, in the live render island.

### EU-0005-022 {#EU-0005-022}

Each playback segment MUST come from a single compiled graph revision.

### EU-0005-023 {#EU-0005-023}

Playback MUST NOT combine in one segment the audio of revisions whose automation or routing conflict.

### EU-0005-024 {#EU-0005-024}

Where the live render island cannot meet the measured deadline of the device and the musician has selected a previous revision, playback MUST keep playing that revision.

### EU-0005-025 {#EU-0005-025}

Where the live render island cannot meet the measured deadline of the device and the musician has not selected a previous revision, the audio device process MUST pause the preview.

### EU-0005-026 {#EU-0005-026}

Euphony MUST show the musician when playback sounds a revision other than the current one.

### EU-0005-027 {#EU-0005-027}

Euphony MUST NOT present a sound as the current revision when it was rendered from another revision.

### EU-0005-028 {#EU-0005-028}

The live render island MUST render the project graph without waiting on another computer for audio.
